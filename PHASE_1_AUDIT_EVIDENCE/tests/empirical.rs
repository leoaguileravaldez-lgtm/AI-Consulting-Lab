use ai_consulting_lab_phase0_contracts::{BehavioralInput, FoundationalContracts};
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};
use titus_lab_phase1_audit_evidence::*;

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    dir: PathBuf,
    journal: AuditJournal,
    ids: Vec<String>,
    hashes: Vec<String>,
    expected: Vec<ExpectedAuditRecord>,
}
impl Fixture {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "titus-audit-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&dir).unwrap();
        let journal =
            AuditJournal::create(dir.join("audit.sqlite3"), "client-a/engagement-1", 7).unwrap();
        Self {
            dir,
            journal,
            ids: vec![],
            hashes: vec![],
            expected: vec![],
        }
    }
    fn append(&mut self, seq: i64, operation: &str, payload: &str) -> AppendResult {
        let pred = if seq == 1 {
            "GENESIS"
        } else {
            &self.hashes[(seq - 2) as usize]
        };
        let e = AuditEvent {
            sequence: seq,
            predecessor_hash: pred,
            operation_id: operation,
            event_type: "AUTHORITATIVE_TRANSITION",
            actor_id: "actor-principal-1",
            workload_id: "workload-audit-1",
            context_id: "client-a/engagement-1",
            generation: 7,
            transition_ref: if seq == 1 {
                "transition-40"
            } else {
                "transition-41"
            },
            provenance: "certified-state-store-v1",
            payload,
        };
        let r = self.journal.append(&e);
        if r.accepted {
            self.ids.push(r.record_id.clone());
            self.hashes.push(r.record_hash.clone());
            self.expected.push(ExpectedAuditRecord {
                record_id: r.record_id.clone(),
                operation_id: operation.into(),
                event_type: "AUTHORITATIVE_TRANSITION".into(),
                actor_id: "actor-principal-1".into(),
                workload_id: "workload-audit-1".into(),
                context_id: "client-a/engagement-1".into(),
                generation: 7,
                transition_ref: if seq == 1 {
                    "transition-40"
                } else {
                    "transition-41"
                }
                .into(),
                provenance: "certified-state-store-v1".into(),
            });
        }
        r
    }
    fn sql(&self, q: &str) -> std::process::Output {
        Command::new("/usr/bin/sqlite3")
            .arg("-batch")
            .arg("-bail")
            .arg(self.journal.db_path())
            .arg(q)
            .output()
            .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).unwrap()
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .into()
}
fn frozen(path: &str) -> Value {
    serde_json::from_str(
        &fs::read_to_string(
            root()
                .join("PHASE_0_CONTRACT_FIRST_EXECUTABLE_BASELINE")
                .join(path),
        )
        .unwrap(),
    )
    .unwrap()
}
fn audit_input() -> BehavioralInput {
    let f = frozen("fixtures/synthetic_contract_case.json");
    let c = f["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["test_id"] == "P0_AUDIT_EVIDENCE")
        .unwrap();
    BehavioralInput {
        fixture_id: c["fixture_id"].as_str().unwrap().into(),
        subject_type: c["subject_type"].as_str().unwrap().into(),
        operation: c["operation"].as_str().unwrap().into(),
        precondition: c["precondition"].as_str().unwrap().into(),
        authority_condition: c["authority_condition"].as_str().unwrap().into(),
        currentness_condition: c["currentness_condition"].as_str().unwrap().into(),
        candidate_behavior: c["candidate_behavior"].as_str().unwrap().into(),
        scenario_data: c["scenario_data"].clone(),
    }
}

#[test]
fn frozen_contract_binding_and_dag_are_exact() {
    let m = frozen("traceability/manifest.json");
    let c = &m["contracts"][8];
    assert_eq!(c["contract_id"], "BC_AUDIT_EVIDENCE");
    assert_eq!(
        c["requirement_identity"],
        "TELEMETRY_AUDIT_EVIDENCE_CONTRACT+O"
    );
    let source = fs::read_to_string(
        root().join("16_OPERATIONAL_REALIZATION_CONFORMANCE_ARCHITECTURE/CANONICAL_MODEL.json"),
    )
    .unwrap();
    assert!(source.contains("T11_REQUIRE_TELEMETRY"));
    assert!(source.contains("\"TELEMETRY_AUDIT_EVIDENCE_CONTRACT\"],\"destination\":\"FUTURE_EMPIRICAL_CERTIFICATION_CONTRACT\""));
}
#[test]
fn frozen_skipped_predecessor_is_rejected_without_mutation() {
    let dir = std::env::temp_dir().join(format!(
        "titus-adapter-{}",
        NEXT.fetch_add(1, Ordering::SeqCst)
    ));
    fs::create_dir(&dir).unwrap();
    let a = AuditEvidenceAdapter::new(dir.join("a.db")).unwrap();
    let o = a.emit_audit_evidence(&audit_input()).unwrap();
    assert_eq!(o.facts["accepted"], false);
    assert_eq!(o.facts["journal_mutated"], false);
    assert_eq!(o.facts["tail_sequence"], 31);
    assert_eq!(o.facts["predecessor_valid"], false);
    assert_eq!(o.facts["mutation_observed"], true);
    assert_eq!(o.facts["classification"], "IncompleteAuditEvidence");
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn attributable_ordered_complete_history_is_valid() {
    let mut f = Fixture::new();
    f.append(1, "op-1", "prepared");
    f.append(2, "op-1", "committed");
    let v = f.journal.verify(&f.expected);
    assert_eq!(v.disposition, AuditDisposition::ValidAuditEvidence);
    assert_eq!((v.observed_count, v.expected_count), (2, 2));
}
#[test]
fn wrong_predecessor_and_reorder_are_rejected() {
    let f = Fixture::new();
    let e = AuditEvent {
        sequence: 1,
        predecessor_hash: "foreign",
        operation_id: "op",
        event_type: "E",
        actor_id: "a",
        workload_id: "w",
        context_id: "client-a/engagement-1",
        generation: 7,
        transition_ref: "t",
        provenance: "p",
        payload: "x",
    };
    let r = f.journal.append(&e);
    assert_eq!(r.disposition, AuditDisposition::IncompleteAuditEvidence);
    assert!(!r.journal_mutated);
}
#[test]
fn omission_is_distinct_from_valid_present_records() {
    let mut f = Fixture::new();
    f.append(1, "op-1", "prepared");
    let mut expected = f.expected.clone();
    let mut missing = expected[0].clone();
    missing.record_id = "required-missing-id".into();
    expected.push(missing);
    assert_eq!(
        f.journal.verify(&expected).disposition,
        AuditDisposition::IncompleteAuditEvidence
    );
}
#[test]
fn payload_tamper_is_detected() {
    let mut f = Fixture::new();
    f.append(1, "op-1", "prepared");
    assert!(
        f.sql("UPDATE audit_records SET payload='modified' WHERE sequence=1;")
            .status
            .success()
    );
    assert_eq!(
        f.journal.verify(&f.expected).disposition,
        AuditDisposition::TamperedAuditEvidence
    );
}
#[test]
fn provenance_tamper_is_detected() {
    let mut f = Fixture::new();
    f.append(1, "op-1", "prepared");
    assert!(
        f.sql("UPDATE audit_records SET provenance='fabricated' WHERE sequence=1;")
            .status
            .success()
    );
    assert_eq!(
        f.journal.verify(&f.expected).disposition,
        AuditDisposition::TamperedAuditEvidence
    );
}
#[test]
fn duplicate_identity_with_substituted_content_is_rejected() {
    let mut f = Fixture::new();
    f.append(1, "op-1", "prepared");
    let e = AuditEvent {
        sequence: 1,
        predecessor_hash: "GENESIS",
        operation_id: "op-1",
        event_type: "AUTHORITATIVE_TRANSITION",
        actor_id: "other-actor",
        workload_id: "workload-audit-1",
        context_id: "client-a/engagement-1",
        generation: 7,
        transition_ref: "transition-40",
        provenance: "certified-state-store-v1",
        payload: "changed",
    };
    assert_eq!(
        f.journal.append(&e).disposition,
        AuditDisposition::DuplicateAuditEvidence
    );
}
#[test]
fn exact_replay_is_rejected() {
    let mut f = Fixture::new();
    let r = f.append(1, "op-1", "prepared");
    let e = AuditEvent {
        sequence: 1,
        predecessor_hash: "GENESIS",
        operation_id: "op-1",
        event_type: "AUTHORITATIVE_TRANSITION",
        actor_id: "actor-principal-1",
        workload_id: "workload-audit-1",
        context_id: "client-a/engagement-1",
        generation: 7,
        transition_ref: "transition-40",
        provenance: "certified-state-store-v1",
        payload: "prepared",
    };
    assert!(r.accepted);
    assert_eq!(
        f.journal.append(&e).disposition,
        AuditDisposition::ReplayedAuditEvidence
    );
}
#[test]
fn cross_context_substitution_is_rejected() {
    let f = Fixture::new();
    let e = AuditEvent {
        sequence: 1,
        predecessor_hash: "GENESIS",
        operation_id: "op",
        event_type: "E",
        actor_id: "a",
        workload_id: "w",
        context_id: "client-b/engagement-9",
        generation: 7,
        transition_ref: "t",
        provenance: "p",
        payload: "x",
    };
    assert_eq!(
        f.journal.append(&e).disposition,
        AuditDisposition::CrossContextEvidence
    );
}
#[test]
fn stale_generation_is_rejected() {
    let f = Fixture::new();
    let e = AuditEvent {
        sequence: 1,
        predecessor_hash: "GENESIS",
        operation_id: "op",
        event_type: "E",
        actor_id: "a",
        workload_id: "w",
        context_id: "client-a/engagement-1",
        generation: 6,
        transition_ref: "t",
        provenance: "p",
        payload: "x",
    };
    assert_eq!(
        f.journal.append(&e).disposition,
        AuditDisposition::StaleEvidence
    );
}
#[test]
fn audit_has_zero_authority_leakage() {
    let mut f = Fixture::new();
    f.append(1, "op-1", "prepared");
    let v = f.journal.verify(&f.expected);
    assert_eq!(
        (
            v.execution_authority_leakage,
            v.recovery_authority_leakage,
            v.layer19_authority_leakage,
            v.certification_authority_leakage
        ),
        (0, 0, 0, 0)
    );
    assert!(
        !f.sql("UPDATE audit_records SET execution_authority=1 WHERE sequence=1;")
            .status
            .success()
    );
}
#[test]
fn unexpected_database_error_cannot_pass() {
    let f = Fixture::new();
    fs::remove_file(f.journal.db_path()).unwrap();
    fs::create_dir(f.journal.db_path()).unwrap();
    let v = f.journal.verify(&[]);
    assert_eq!(v.disposition, AuditDisposition::UnexpectedDatabaseError);
}
#[test]
fn unexpected_process_error_cannot_pass() {
    assert_eq!(
        probe_sqlite_process(std::path::Path::new("/definitely/unavailable/sqlite3")),
        AuditDisposition::UnexpectedProcessError
    );
}
#[test]
fn cleanup_recreate_is_deterministic() {
    let (ids, hashes) = {
        let mut f = Fixture::new();
        f.append(1, "op-1", "prepared");
        f.append(2, "op-1", "committed");
        (f.ids.clone(), f.hashes.clone())
    };
    let mut g = Fixture::new();
    g.append(1, "op-1", "prepared");
    g.append(2, "op-1", "committed");
    assert_eq!((g.ids.clone(), g.hashes.clone()), (ids, hashes));
    assert_eq!(
        g.journal.verify(&g.expected).disposition,
        AuditDisposition::ValidAuditEvidence
    );
}

#[test]
fn hash_consistent_wrong_provenance_is_rejected_against_authoritative_binding() {
    let f = Fixture::new();
    let e = AuditEvent {
        sequence: 1,
        predecessor_hash: "GENESIS",
        operation_id: "op-1",
        event_type: "AUTHORITATIVE_TRANSITION",
        actor_id: "actor-principal-1",
        workload_id: "workload-audit-1",
        context_id: "client-a/engagement-1",
        generation: 7,
        transition_ref: "transition-40",
        provenance: "fabricated-but-hash-consistent-source",
        payload: "prepared",
    };
    let r = f.journal.append(&e);
    assert!(r.accepted);
    let authoritative = [ExpectedAuditRecord {
        record_id: r.record_id,
        operation_id: "op-1".into(),
        event_type: "AUTHORITATIVE_TRANSITION".into(),
        actor_id: "actor-principal-1".into(),
        workload_id: "workload-audit-1".into(),
        context_id: "client-a/engagement-1".into(),
        generation: 7,
        transition_ref: "transition-40".into(),
        provenance: "certified-state-store-v1".into(),
    }];
    assert_eq!(
        f.journal.verify(&authoritative).disposition,
        AuditDisposition::TamperedAuditEvidence
    );
}

#[test]
fn adapter_database_error_is_not_expected_predecessor_rejection() {
    let dir = std::env::temp_dir().join(format!(
        "titus-adapter-error-{}",
        NEXT.fetch_add(1, Ordering::SeqCst)
    ));
    fs::create_dir(&dir).unwrap();
    let adapter = AuditEvidenceAdapter::new(dir.join("audit.db")).unwrap();
    fs::remove_file(adapter.db_path()).unwrap();
    fs::create_dir(adapter.db_path()).unwrap();
    assert_eq!(
        adapter.observe(&audit_input()),
        Err(AuditDisposition::UnexpectedDatabaseError)
    );
    assert!(adapter.emit_audit_evidence(&audit_input()).is_err());
    fs::remove_dir_all(dir).unwrap();
}
