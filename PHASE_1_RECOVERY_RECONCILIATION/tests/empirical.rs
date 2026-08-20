use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, FoundationalContracts, ObservedBehavior,
};
use ai_consulting_lab_phase1_recovery_reconciliation::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Db {
    dir: PathBuf,
    path: PathBuf,
}
impl Db {
    fn fresh() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "ai-consulting-lab-recovery-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&dir).unwrap();
        let path = dir.join("recovery.sqlite3");
        bootstrap(&path).unwrap();
        Self { dir, path }
    }
    fn boundary(&self) -> RecoveryBoundary {
        RecoveryBoundary::new(env!("CARGO_BIN_EXE_recovery_worker"), &self.path)
    }
}
impl Drop for Db {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).unwrap()
    }
}
fn interrupted(db: &Db) {
    let mut c = Command::new("/usr/bin/sqlite3")
        .arg("-batch")
        .arg("-bail")
        .arg(&db.path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    c.stdin.as_mut().unwrap().write_all(b"BEGIN IMMEDIATE; INSERT INTO transitions VALUES('uncommitted-op',21,22,'synthetic-hash-21'); SELECT 'INTERRUPTION_READY';\n").unwrap();
    let mut line = String::new();
    BufReader::new(c.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    assert_eq!(line.trim(), "INTERRUPTION_READY");
    c.kill().unwrap();
    c.wait().unwrap();
}
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .into()
}
fn frozen(p: &str) -> Value {
    serde_json::from_str(
        &fs::read_to_string(
            root()
                .join("PHASE_0_CONTRACT_FIRST_EXECUTABLE_BASELINE")
                .join(p),
        )
        .unwrap(),
    )
    .unwrap()
}
fn input() -> BehavioralInput {
    let f = frozen("fixtures/synthetic_contract_case.json");
    let c = f["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["test_id"] == "P0_RECOVERY_RECONCILIATION")
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
fn digest(i: &BehavioralInput) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&i.scenario_data).unwrap())
    )
}
fn empirical(i: &BehavioralInput, o: &ObservedBehavior) -> bool {
    o.input_digest == digest(i)
        && o.facts["retry_allowed"] == false
        && o.facts["predecessor_preserved"] == true
        && o.facts["reconciliation_required"] == true
        && o.facts["classification"] == "ReconciliationRequired"
        && o.facts["fresh_process"] == true
        && o.facts["external_effect_determined"] == false
}
#[test]
fn bindings_reconcile() {
    let m = frozen("traceability/manifest.json");
    let b: Value = serde_json::from_str(
        &fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bindings.json"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        m["contracts"][7]["contract_id"],
        b["selected_contracts"][0]["contract_id"]
    );
    assert_eq!(
        m["contracts"][7]["requirement_identity"],
        b["selected_contracts"][0]["certified_requirement"]
    );
    assert_eq!(b["deferred_contracts"].as_array().unwrap().len(), 2)
}
#[test]
fn committed_state_reconstructs_in_fresh_process() {
    assert!(infrastructure_adequate());
    let d = Db::fresh();
    seed_committed(&d.path).unwrap();
    let r = d.boundary().recover();
    assert_eq!(r.disposition, RecoveryDisposition::RecoveredCommittedState);
    assert_eq!(count_transition(&d.path, "committed-op").unwrap(), 1);
    assert!(r.predecessor_preserved);
    assert_eq!(r.operation_id, "committed-op");
    assert_eq!(r.transition_predecessor, 21);
    assert_eq!(r.transition_sequence, 22);
    assert_eq!(r.provenance, "synthetic-hash-21")
}
#[test]
fn wrong_schema_valid_committed_transition_is_not_reconstructed() {
    let d = Db::fresh();
    seed_wrong_committed(&d.path).unwrap();
    let r = d.boundary().recover();
    assert_eq!(r.disposition, RecoveryDisposition::BehavioralFailure);
    assert_ne!(r.disposition, RecoveryDisposition::RecoveredCommittedState)
}
#[test]
fn interrupted_transaction_is_not_resurrected() {
    let d = Db::fresh();
    seed_unknown(&d.path).unwrap();
    interrupted(&d);
    assert_eq!(count_transition(&d.path, "uncommitted-op").unwrap(), 0);
    assert_eq!(
        d.boundary().recover().disposition,
        RecoveryDisposition::ReconciliationRequired
    )
}
#[test]
fn frozen_unknown_outcome_reconstructs_lineage_and_blocks_retry() {
    let d = Db::fresh();
    seed_unknown(&d.path).unwrap();
    let a = RecoveryReconciliationAdapter::new(d.boundary());
    let i = input();
    let o = a.recover_and_reconcile(&i).unwrap();
    assert!(empirical(&i, &o));
    assert_eq!(o.facts["checkpoint_sequence"], 20);
    assert_eq!(o.facts["journal_tail_sequence"], 21);
    assert_eq!(o.facts["outcome"], "UNKNOWN")
}
#[test]
fn stale_worker_and_replay_are_rejected() {
    let d = Db::fresh();
    seed_committed(&d.path).unwrap();
    assert_eq!(
        stale_worker_attempt(&d.path),
        RecoveryDisposition::ExpectedStaleWorkerRejection
    );
    assert_eq!(
        replay_attempt(&d.path),
        RecoveryDisposition::ExpectedReplayRejection
    );
    assert_eq!(count_transition(&d.path, "committed-op").unwrap(), 1)
}
#[test]
fn revoked_authority_does_not_resurrect() {
    let d = Db::fresh();
    seed_unknown(&d.path).unwrap();
    revoke(&d.path).unwrap();
    assert_eq!(
        d.boundary().recover().disposition,
        RecoveryDisposition::ReconciliationRequired
    );
    assert_eq!(authority(&d.path).unwrap(), "6|REVOKED")
}
#[test]
fn false_pass_controls_rejected() {
    let i = input();
    for facts in [
        json!({}),
        json!({"retry_allowed":true,"predecessor_preserved":true,"reconciliation_required":false,"classification":"RecoveredCommittedState","fresh_process":false,"external_effect_determined":true}),
        json!({"retry_allowed":false,"predecessor_preserved":false,"reconciliation_required":true,"classification":"ReconciliationRequired","fresh_process":true,"external_effect_determined":false}),
    ] {
        assert!(!empirical(
            &i,
            &ObservedBehavior {
                input_digest: digest(&i),
                facts
            }
        ))
    }
}
#[test]
fn unexpected_database_error_not_pass() {
    let d = Db::fresh();
    let b = RecoveryBoundary::new(
        env!("CARGO_BIN_EXE_recovery_worker"),
        d.dir.join("missing.sqlite3"),
    );
    let r = b.recover();
    assert_eq!(r.disposition, RecoveryDisposition::UnexpectedDatabaseError);
    assert!(!r.retry_allowed)
}
#[test]
fn downstream_contracts_absent() {
    let d = Db::fresh();
    seed_unknown(&d.path).unwrap();
    let a = RecoveryReconciliationAdapter::new(d.boundary());
    let i = input();
    assert_eq!(
        a.emit_audit_evidence(&i),
        Err(ContractError::ExpectedFailNotImplemented)
    );
    assert_eq!(
        a.certify_empirically(&i),
        Err(ContractError::ExpectedFailNotImplemented)
    )
}
