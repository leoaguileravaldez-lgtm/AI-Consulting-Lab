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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryDisposition {
    RecoveredCommittedState,
    DiscardedUncommittedState,
    ExpectedStaleWorkerRejection,
    ExpectedReplayRejection,
    UnknownOrIndeterminate,
    ReconciliationRequired,
    UnexpectedDatabaseError,
    UnexpectedProcessError,
    BehavioralFailure,
    HarnessFailure,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryObservation {
    pub disposition: RecoveryDisposition,
    pub retry_allowed: bool,
    pub predecessor_preserved: bool,
    pub reconciliation_required: bool,
    pub checkpoint: i64,
    pub tail: i64,
    pub outcome: String,
    pub operation_id: String,
    pub transition_predecessor: i64,
    pub transition_sequence: i64,
    pub provenance: String,
    pub stderr: String,
}

#[derive(Debug, Clone)]
pub struct RecoveryBoundary {
    worker: PathBuf,
    db: PathBuf,
}
impl RecoveryBoundary {
    pub fn new(worker: impl Into<PathBuf>, db: impl Into<PathBuf>) -> Self {
        Self {
            worker: worker.into(),
            db: db.into(),
        }
    }
    pub fn recover(&self) -> RecoveryObservation {
        let out = Command::new(&self.worker)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg(&self.db)
            .output();
        let Ok(out) = out else {
            return failure(RecoveryDisposition::UnexpectedProcessError, "WORKER_LAUNCH");
        };
        if !out.status.success() {
            return RecoveryObservation {
                stderr: String::from_utf8_lossy(&out.stderr).trim().into(),
                ..failure(
                    match out.status.code() {
                        Some(21) => RecoveryDisposition::UnexpectedDatabaseError,
                        Some(23) => RecoveryDisposition::BehavioralFailure,
                        _ => RecoveryDisposition::UnexpectedProcessError,
                    },
                    "",
                )
            };
        }
        parse(String::from_utf8_lossy(&out.stdout).trim())
    }
}
fn parse(s: &str) -> RecoveryObservation {
    let p: Vec<_> = s.split('|').collect();
    if p.len() != 11 {
        return failure(RecoveryDisposition::HarnessFailure, "PARSE");
    };
    let d = match p[0] {
        "RECOVERED_COMMITTED_STATE" => RecoveryDisposition::RecoveredCommittedState,
        "RECONCILIATION_REQUIRED" => RecoveryDisposition::ReconciliationRequired,
        _ => RecoveryDisposition::BehavioralFailure,
    };
    RecoveryObservation {
        disposition: d,
        retry_allowed: p[1] == "1",
        predecessor_preserved: p[2] == "1",
        reconciliation_required: p[3] == "1",
        checkpoint: p[4].parse().unwrap_or(-1),
        tail: p[5].parse().unwrap_or(-1),
        outcome: p[6].into(),
        operation_id: p[7].into(),
        transition_predecessor: p[8].parse().unwrap_or(-1),
        transition_sequence: p[9].parse().unwrap_or(-1),
        provenance: p[10].into(),
        stderr: String::new(),
    }
}
fn failure(d: RecoveryDisposition, e: &str) -> RecoveryObservation {
    RecoveryObservation {
        disposition: d,
        retry_allowed: false,
        predecessor_preserved: false,
        reconciliation_required: false,
        checkpoint: -1,
        tail: -1,
        outcome: "UNKNOWN".into(),
        operation_id: String::new(),
        transition_predecessor: -1,
        transition_sequence: -1,
        provenance: String::new(),
        stderr: e.into(),
    }
}
fn sql(db: &Path, q: &str) -> Result<String, String> {
    let o = Command::new(SQLITE)
        .arg("-batch")
        .arg("-bail")
        .arg(db)
        .arg(q)
        .output()
        .map_err(|e| format!("PROCESS:{e}"))?;
    if !o.status.success() {
        return Err(format!(
            "SQLITE:{:?}:{}",
            o.status.code(),
            String::from_utf8_lossy(&o.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&o.stdout).into())
}
pub fn bootstrap(db: &Path) -> Result<(), String> {
    sql(db, SCHEMA).map(|_| ())
}
pub fn seed_unknown(db: &Path) -> Result<(), String> {
    sql(db,"BEGIN IMMEDIATE; INSERT INTO checkpoints VALUES(20,'synthetic-hash-20',5); INSERT INTO journal VALUES(21,'synthetic-hash-20','UNKNOWN',0); INSERT INTO authority VALUES('synthetic',5,'CURRENT'); INSERT INTO runtime_epoch VALUES('synthetic',2); COMMIT;").map(|_|())
}
pub fn seed_committed(db: &Path) -> Result<(), String> {
    seed_unknown(db)?;
    sql(db,"BEGIN IMMEDIATE; INSERT INTO transitions VALUES('committed-op',21,22,'synthetic-hash-21'); UPDATE journal SET outcome='COMMITTED' WHERE sequence=21; COMMIT;").map(|_|())
}
pub fn seed_wrong_committed(db: &Path) -> Result<(), String> {
    seed_unknown(db)?;
    sql(db, "BEGIN IMMEDIATE; INSERT INTO transitions VALUES('wrong-unrelated-op',99,100,'wrong-provenance'); UPDATE journal SET outcome='COMMITTED' WHERE sequence=21; COMMIT;").map(|_| ())
}
pub fn revoke(db: &Path) -> Result<(), String> {
    sql(
        db,
        "UPDATE authority SET generation=6,status='REVOKED' WHERE id='synthetic';",
    )
    .map(|_| ())
}
pub fn count_transition(db: &Path, op: &str) -> Result<i64, String> {
    sql(
        db,
        &format!(
            "SELECT count(*) FROM transitions WHERE operation_id='{}';",
            op.replace('\'', "''")
        ),
    )?
    .trim()
    .parse()
    .map_err(|_| "COUNT".into())
}
pub fn stale_worker_attempt(db: &Path) -> RecoveryDisposition {
    match sql(
        db,
        "UPDATE runtime_epoch SET epoch=3 WHERE id='synthetic' AND epoch=1; SELECT changes();",
    ) {
        Ok(s) if s.lines().any(|v| v == "0") => RecoveryDisposition::ExpectedStaleWorkerRejection,
        Ok(_) => RecoveryDisposition::BehavioralFailure,
        Err(_) => RecoveryDisposition::UnexpectedDatabaseError,
    }
}
pub fn replay_attempt(db: &Path) -> RecoveryDisposition {
    match sql(
        db,
        "INSERT INTO transitions VALUES('committed-op',21,22,'synthetic-hash-21');",
    ) {
        Err(e) if e.contains("UNIQUE constraint failed") => {
            RecoveryDisposition::ExpectedReplayRejection
        }
        Err(_) => RecoveryDisposition::UnexpectedDatabaseError,
        Ok(_) => RecoveryDisposition::BehavioralFailure,
    }
}
pub fn authority(db: &Path) -> Result<String, String> {
    Ok(sql(
        db,
        "SELECT generation||'|'||status FROM authority WHERE id='synthetic';",
    )?
    .trim()
    .into())
}
const SCHEMA: &str = "PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; CREATE TABLE checkpoints(sequence INTEGER PRIMARY KEY,state_hash TEXT NOT NULL,generation INTEGER NOT NULL); CREATE TABLE journal(sequence INTEGER PRIMARY KEY,predecessor_hash TEXT NOT NULL,outcome TEXT NOT NULL CHECK(outcome IN('UNKNOWN','COMMITTED')),retry_authorized INTEGER NOT NULL CHECK(retry_authorized=0)); CREATE TABLE transitions(operation_id TEXT PRIMARY KEY,predecessor_sequence INTEGER NOT NULL,new_sequence INTEGER NOT NULL,provenance TEXT NOT NULL,CHECK(new_sequence=predecessor_sequence+1)); CREATE TABLE authority(id TEXT PRIMARY KEY,generation INTEGER NOT NULL,status TEXT NOT NULL CHECK(status IN('CURRENT','REVOKED','SUPERSEDED'))); CREATE TRIGGER no_resurrection BEFORE UPDATE ON authority WHEN OLD.status IN('REVOKED','SUPERSEDED') AND NEW.status='CURRENT' BEGIN SELECT RAISE(ABORT,'AUTHORITY_RESURRECTION'); END; CREATE TABLE runtime_epoch(id TEXT PRIMARY KEY,epoch INTEGER NOT NULL);";
fn digest(i: &BehavioralInput) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&i.scenario_data).unwrap())
    )
}
#[derive(Debug, Clone)]
pub struct RecoveryReconciliationAdapter {
    boundary: RecoveryBoundary,
}
impl RecoveryReconciliationAdapter {
    pub fn new(boundary: RecoveryBoundary) -> Self {
        Self { boundary }
    }
}
impl FoundationalContracts for RecoveryReconciliationAdapter {
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
    fn recover_and_reconcile(&self, input: &BehavioralInput) -> ContractResult {
        let o = self.boundary.recover();
        Ok(ObservedBehavior {
            input_digest: digest(input),
            facts: json!({"retry_allowed":o.retry_allowed,"predecessor_preserved":o.predecessor_preserved,"reconciliation_required":o.reconciliation_required,"classification":format!("{:?}",o.disposition),"checkpoint_sequence":o.checkpoint,"journal_tail_sequence":o.tail,"outcome":o.outcome,"operation_id":o.operation_id,"transition_predecessor":o.transition_predecessor,"transition_sequence":o.transition_sequence,"provenance":o.provenance,"fresh_process":true,"external_effect_determined":false,"stderr":o.stderr}),
        })
    }
    fn emit_audit_evidence(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn certify_empirically(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
}
pub fn infrastructure_adequate() -> bool {
    Path::new(SQLITE).is_file()
}
