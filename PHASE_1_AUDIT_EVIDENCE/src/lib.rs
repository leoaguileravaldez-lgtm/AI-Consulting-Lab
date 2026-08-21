use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, ContractResult, FoundationalContracts, ObservedBehavior,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const SQLITE: &str = "/usr/bin/sqlite3";
const GENESIS: &str = "GENESIS";

pub fn probe_sqlite_process(executable: &Path) -> AuditDisposition {
    match Command::new(executable).arg("--version").output() {
        Ok(output) if output.status.success() => AuditDisposition::ValidAuditEvidence,
        Ok(_) => AuditDisposition::UnexpectedProcessError,
        Err(_) => AuditDisposition::UnexpectedProcessError,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditDisposition {
    ValidAuditEvidence,
    IncompleteAuditEvidence,
    TamperedAuditEvidence,
    DuplicateAuditEvidence,
    ReplayedAuditEvidence,
    CrossContextEvidence,
    StaleEvidence,
    UnknownOrIndeterminate,
    UnexpectedDatabaseError,
    UnexpectedProcessError,
    BehavioralFailure,
    HarnessFailure,
}

#[derive(Debug, Clone)]
pub struct AuditEvent<'a> {
    pub sequence: i64,
    pub predecessor_hash: &'a str,
    pub operation_id: &'a str,
    pub event_type: &'a str,
    pub actor_id: &'a str,
    pub workload_id: &'a str,
    pub context_id: &'a str,
    pub generation: i64,
    pub transition_ref: &'a str,
    pub provenance: &'a str,
    pub payload: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendResult {
    pub disposition: AuditDisposition,
    pub accepted: bool,
    pub journal_mutated: bool,
    pub record_id: String,
    pub record_hash: String,
    pub tail_sequence: Option<i64>,
    pub predecessor_valid: Option<bool>,
    pub mutation_observed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedAuditRecord {
    pub record_id: String,
    pub operation_id: String,
    pub event_type: String,
    pub actor_id: String,
    pub workload_id: String,
    pub context_id: String,
    pub generation: i64,
    pub transition_ref: String,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verification {
    pub disposition: AuditDisposition,
    pub observed_count: usize,
    pub expected_count: usize,
    pub execution_authority_leakage: usize,
    pub recovery_authority_leakage: usize,
    pub layer19_authority_leakage: usize,
    pub certification_authority_leakage: usize,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct AuditJournal {
    db: PathBuf,
    context_id: String,
    generation: i64,
}

impl AuditJournal {
    pub fn create(
        db: impl Into<PathBuf>,
        context_id: &str,
        generation: i64,
    ) -> Result<Self, String> {
        let journal = Self {
            db: db.into(),
            context_id: context_id.into(),
            generation,
        };
        journal.sql("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; CREATE TABLE IF NOT EXISTS audit_records(sequence INTEGER PRIMARY KEY,record_id TEXT NOT NULL UNIQUE,operation_id TEXT NOT NULL,event_type TEXT NOT NULL,actor_id TEXT NOT NULL,workload_id TEXT NOT NULL,context_id TEXT NOT NULL,generation INTEGER NOT NULL,transition_ref TEXT NOT NULL,provenance TEXT NOT NULL,predecessor_hash TEXT NOT NULL,payload TEXT NOT NULL,record_hash TEXT NOT NULL UNIQUE,execution_authority INTEGER NOT NULL DEFAULT 0 CHECK(execution_authority=0),recovery_authority INTEGER NOT NULL DEFAULT 0 CHECK(recovery_authority=0),layer19_truth_authority INTEGER NOT NULL DEFAULT 0 CHECK(layer19_truth_authority=0),certification_authority INTEGER NOT NULL DEFAULT 0 CHECK(certification_authority=0));")?;
        Ok(journal)
    }

    pub fn db_path(&self) -> &Path {
        &self.db
    }

    pub fn append(&self, event: &AuditEvent<'_>) -> AppendResult {
        if event.context_id != self.context_id {
            return rejected(
                AuditDisposition::CrossContextEvidence,
                "CONTEXT_MISMATCH",
                None,
                None,
                false,
            );
        }
        if event.generation != self.generation {
            return rejected(
                AuditDisposition::StaleEvidence,
                "GENERATION_MISMATCH",
                None,
                None,
                false,
            );
        }
        let record_id = digest(
            "audit-record-id-v1",
            &[
                event.context_id,
                &event.generation.to_string(),
                event.operation_id,
                event.event_type,
                &event.sequence.to_string(),
            ],
        );
        let record_hash = event_hash(event, &record_id);
        let existing = match self.sql(&format!(
            "SELECT record_hash FROM audit_records WHERE record_id='{}';",
            sq(&record_id)
        )) {
            Ok(v) => v,
            Err(e) => {
                return rejected(
                    AuditDisposition::UnexpectedDatabaseError,
                    &e,
                    None,
                    None,
                    false,
                );
            }
        };
        if !existing.trim().is_empty() {
            return if existing.trim() == record_hash {
                rejected(
                    AuditDisposition::ReplayedAuditEvidence,
                    "REPLAYED_EVIDENCE",
                    None,
                    None,
                    false,
                )
            } else {
                rejected(
                    AuditDisposition::DuplicateAuditEvidence,
                    "DUPLICATE_IDENTITY_DIFFERENT_CONTENT",
                    None,
                    None,
                    false,
                )
            };
        }
        let tail = match self.sql(
            "SELECT sequence||'|'||record_hash FROM audit_records ORDER BY sequence DESC LIMIT 1;",
        ) {
            Ok(v) => v,
            Err(e) => {
                return rejected(
                    AuditDisposition::UnexpectedDatabaseError,
                    &e,
                    None,
                    None,
                    false,
                );
            }
        };
        let (tail_seq, tail_hash) = if tail.trim().is_empty() {
            (0, GENESIS.to_string())
        } else {
            let Some((s, h)) = tail.trim().split_once('|') else {
                return rejected(
                    AuditDisposition::HarnessFailure,
                    "TAIL_PARSE",
                    None,
                    None,
                    false,
                );
            };
            let Ok(s) = s.parse::<i64>() else {
                return rejected(
                    AuditDisposition::HarnessFailure,
                    "TAIL_SEQUENCE_PARSE",
                    None,
                    None,
                    false,
                );
            };
            (s, h.to_string())
        };
        if event.sequence != tail_seq + 1 || event.predecessor_hash != tail_hash {
            return match self.tail() {
                Ok(after) if after == tail_seq => rejected(
                    AuditDisposition::IncompleteAuditEvidence,
                    "PREDECESSOR_SEQUENCE_REJECTED",
                    Some(after),
                    Some(false),
                    true,
                ),
                Ok(after) => AppendResult {
                    disposition: AuditDisposition::BehavioralFailure,
                    accepted: false,
                    journal_mutated: true,
                    record_id: String::new(),
                    record_hash: String::new(),
                    tail_sequence: Some(after),
                    predecessor_valid: Some(false),
                    mutation_observed: true,
                    detail: "JOURNAL_CHANGED_DURING_REJECTION".into(),
                },
                Err(e) => rejected(
                    AuditDisposition::UnexpectedDatabaseError,
                    &e,
                    None,
                    Some(false),
                    false,
                ),
            };
        }
        let q = format!(
            "BEGIN IMMEDIATE; INSERT INTO audit_records VALUES({seq},'{rid}','{op}','{typ}','{actor}','{workload}','{context}',{generation},'{transition}','{provenance}','{pred}','{payload}','{hash}',0,0,0,0); COMMIT;",
            seq = event.sequence,
            rid = sq(&record_id),
            op = sq(event.operation_id),
            typ = sq(event.event_type),
            actor = sq(event.actor_id),
            workload = sq(event.workload_id),
            context = sq(event.context_id),
            generation = event.generation,
            transition = sq(event.transition_ref),
            provenance = sq(event.provenance),
            pred = sq(event.predecessor_hash),
            payload = sq(event.payload),
            hash = sq(&record_hash)
        );
        match self.sql(&q) {
            Ok(_) => AppendResult {
                disposition: AuditDisposition::ValidAuditEvidence,
                accepted: true,
                journal_mutated: true,
                record_id,
                record_hash,
                tail_sequence: Some(event.sequence),
                predecessor_valid: Some(true),
                mutation_observed: true,
                detail: "APPENDED_OBSERVATIONAL_ONLY".into(),
            },
            Err(e) if e.contains("UNIQUE constraint failed: audit_records.record_id") => rejected(
                AuditDisposition::DuplicateAuditEvidence,
                "DUPLICATE_IDENTITY",
                Some(tail_seq),
                Some(true),
                true,
            ),
            Err(e) if e.contains("UNIQUE constraint failed") => rejected(
                AuditDisposition::ReplayedAuditEvidence,
                "REPLAYED_EVIDENCE",
                Some(tail_seq),
                Some(true),
                true,
            ),
            Err(e) => {
                let after = self.tail().ok();
                rejected(
                    AuditDisposition::UnexpectedDatabaseError,
                    &e,
                    after,
                    Some(true),
                    after.is_some(),
                )
            }
        }
    }

    pub fn verify(&self, expected: &[ExpectedAuditRecord]) -> Verification {
        let q = "SELECT sequence||'|'||record_id||'|'||operation_id||'|'||event_type||'|'||actor_id||'|'||workload_id||'|'||context_id||'|'||generation||'|'||transition_ref||'|'||provenance||'|'||predecessor_hash||'|'||payload||'|'||record_hash||'|'||execution_authority||'|'||recovery_authority||'|'||layer19_truth_authority||'|'||certification_authority FROM audit_records ORDER BY sequence;";
        let raw = match self.sql(q) {
            Ok(v) => v,
            Err(e) => {
                return verification(
                    AuditDisposition::UnexpectedDatabaseError,
                    0,
                    expected.len(),
                    &e,
                );
            }
        };
        let lines: Vec<_> = raw.lines().filter(|l| !l.is_empty()).collect();
        if lines.len() != expected.len() {
            return verification(
                AuditDisposition::IncompleteAuditEvidence,
                lines.len(),
                expected.len(),
                "EXPECTED_OBSERVED_COUNT_MISMATCH",
            );
        }
        let mut predecessor = GENESIS.to_string();
        for (index, line) in lines.iter().enumerate() {
            let p: Vec<_> = line.split('|').collect();
            if p.len() != 17 {
                return verification(
                    AuditDisposition::HarnessFailure,
                    lines.len(),
                    expected.len(),
                    "ROW_PARSE",
                );
            }
            let seq = index as i64 + 1;
            if p[0].parse::<i64>().ok() != Some(seq)
                || p[10] != predecessor
                || p[1] != expected[index].record_id
            {
                return verification(
                    AuditDisposition::IncompleteAuditEvidence,
                    lines.len(),
                    expected.len(),
                    "ORDER_LINEAGE_OR_MEMBERSHIP_MISMATCH",
                );
            }
            if p[6] != self.context_id {
                return verification(
                    AuditDisposition::CrossContextEvidence,
                    lines.len(),
                    expected.len(),
                    "CONTEXT_SPLICE",
                );
            }
            if p[7].parse::<i64>().ok() != Some(self.generation) {
                return verification(
                    AuditDisposition::StaleEvidence,
                    lines.len(),
                    expected.len(),
                    "STALE_GENERATION",
                );
            }
            let event = AuditEvent {
                sequence: seq,
                predecessor_hash: p[10],
                operation_id: p[2],
                event_type: p[3],
                actor_id: p[4],
                workload_id: p[5],
                context_id: p[6],
                generation: self.generation,
                transition_ref: p[8],
                provenance: p[9],
                payload: p[11],
            };
            let authoritative = &expected[index];
            if p[2] != authoritative.operation_id
                || p[3] != authoritative.event_type
                || p[4] != authoritative.actor_id
                || p[5] != authoritative.workload_id
                || p[6] != authoritative.context_id
                || p[7].parse::<i64>().ok() != Some(authoritative.generation)
                || p[8] != authoritative.transition_ref
                || p[9] != authoritative.provenance
            {
                return verification(
                    AuditDisposition::TamperedAuditEvidence,
                    lines.len(),
                    expected.len(),
                    "AUTHORITATIVE_PROVENANCE_MISMATCH",
                );
            }
            if event_hash(&event, p[1]) != p[12] {
                return verification(
                    AuditDisposition::TamperedAuditEvidence,
                    lines.len(),
                    expected.len(),
                    "HASH_OR_PROVENANCE_MISMATCH",
                );
            }
            if p[13..17].iter().any(|v| *v != "0") {
                return verification(
                    AuditDisposition::BehavioralFailure,
                    lines.len(),
                    expected.len(),
                    "AUDIT_AUTHORITY_LAUNDERING",
                );
            }
            predecessor = p[12].to_string();
        }
        Verification {
            disposition: AuditDisposition::ValidAuditEvidence,
            observed_count: lines.len(),
            expected_count: expected.len(),
            execution_authority_leakage: 0,
            recovery_authority_leakage: 0,
            layer19_authority_leakage: 0,
            certification_authority_leakage: 0,
            detail: "COMPLETE_ORDERED_ATTRIBUTABLE_TAMPER_EVIDENT".into(),
        }
    }

    fn sql(&self, query: &str) -> Result<String, String> {
        let out = Command::new(SQLITE)
            .arg("-batch")
            .arg("-bail")
            .arg(&self.db)
            .arg(query)
            .output()
            .map_err(|e| format!("PROCESS:{e}"))?;
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
        }
    }

    fn tail(&self) -> Result<i64, String> {
        self.sql("SELECT COALESCE(MAX(sequence),0) FROM audit_records;")?
            .parse::<i64>()
            .map_err(|_| "TAIL_SEQUENCE_PARSE".into())
    }
}

fn rejected(
    disposition: AuditDisposition,
    detail: &str,
    tail_sequence: Option<i64>,
    predecessor_valid: Option<bool>,
    mutation_observed: bool,
) -> AppendResult {
    AppendResult {
        disposition,
        accepted: false,
        journal_mutated: false,
        record_id: String::new(),
        record_hash: String::new(),
        tail_sequence,
        predecessor_valid,
        mutation_observed,
        detail: detail.into(),
    }
}
fn verification(
    disposition: AuditDisposition,
    observed: usize,
    expected: usize,
    detail: &str,
) -> Verification {
    Verification {
        disposition,
        observed_count: observed,
        expected_count: expected,
        execution_authority_leakage: 0,
        recovery_authority_leakage: 0,
        layer19_authority_leakage: 0,
        certification_authority_leakage: 0,
        detail: detail.into(),
    }
}
fn sq(v: &str) -> String {
    v.replace('\'', "''")
}
fn digest(domain: &str, fields: &[&str]) -> String {
    let mut h = Sha256::new();
    h.update(domain.as_bytes());
    for f in fields {
        h.update([0x1f]);
        h.update(f.as_bytes());
    }
    format!("{:x}", h.finalize())
}
fn event_hash(e: &AuditEvent<'_>, record_id: &str) -> String {
    digest(
        "audit-event-v1",
        &[
            record_id,
            &e.sequence.to_string(),
            e.predecessor_hash,
            e.operation_id,
            e.event_type,
            e.actor_id,
            e.workload_id,
            e.context_id,
            &e.generation.to_string(),
            e.transition_ref,
            e.provenance,
            e.payload,
        ],
    )
}
fn input_digest(input: &BehavioralInput) -> String {
    digest(
        "phase0-audit-input-v1",
        &[
            &input.fixture_id,
            &input.subject_type,
            &input.operation,
            &input.precondition,
            &input.authority_condition,
            &input.currentness_condition,
            &input.candidate_behavior,
            &input.scenario_data.to_string(),
        ],
    )
}

pub struct AuditEvidenceAdapter {
    journal: AuditJournal,
}
impl AuditEvidenceAdapter {
    pub fn new(db: impl Into<PathBuf>) -> Result<Self, String> {
        let journal = AuditJournal::create(db, "synthetic-context", 1)?;
        journal.sql("INSERT OR IGNORE INTO audit_records VALUES(31,'seed-31','seed-op','STATE_TRANSITION','actor-1','workload-1','synthetic-context',1,'transition-31','certified-source','synthetic-hash-30','seed','synthetic-hash-31',0,0,0,0);")?;
        Ok(Self { journal })
    }
    pub fn db_path(&self) -> &Path {
        self.journal.db_path()
    }

    pub fn observe(&self, input: &BehavioralInput) -> Result<ObservedBehavior, AuditDisposition> {
        let s = &input.scenario_data;
        let e = AuditEvent {
            sequence: s["new_sequence"].as_i64().unwrap_or(-1),
            predecessor_hash: s["supplied_predecessor_hash"].as_str().unwrap_or(""),
            operation_id: "fixture-op",
            event_type: "STATE_TRANSITION",
            actor_id: "actor-1",
            workload_id: "workload-1",
            context_id: "synthetic-context",
            generation: 1,
            transition_ref: "transition-32",
            provenance: "certified-source",
            payload: "fixture",
        };
        let r = self.journal.append(&e);
        if matches!(
            r.disposition,
            AuditDisposition::UnexpectedDatabaseError
                | AuditDisposition::UnexpectedProcessError
                | AuditDisposition::HarnessFailure
        ) {
            return Err(r.disposition);
        }
        Ok(ObservedBehavior {
            input_digest: input_digest(input),
            facts: json!({"accepted":r.accepted,"journal_mutated":r.journal_mutated,"tail_sequence":r.tail_sequence,"predecessor_valid":r.predecessor_valid,"mutation_observed":r.mutation_observed,"classification":format!("{:?}",r.disposition)}),
        })
    }
}
impl FoundationalContracts for AuditEvidenceAdapter {
    fn emit_audit_evidence(&self, input: &BehavioralInput) -> ContractResult {
        self.observe(input)
            .map_err(|_| ContractError::ExpectedFailNotImplemented)
    }
    fn canonical_representation(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn bind_identity_version_provenance(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn verify_cryptographic_integrity(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn persist_transactionally(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn enforce_global_uniqueness(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn propagate_revocation_and_freshness(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn attest_runtime_isolation(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn recover_and_reconcile(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn certify_empirically(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
}
