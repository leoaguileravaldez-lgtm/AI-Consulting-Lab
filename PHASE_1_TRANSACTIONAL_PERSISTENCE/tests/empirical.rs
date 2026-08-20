use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, FoundationalContracts, ObservedBehavior,
};
use ai_consulting_lab_phase1_transactional_persistence::{
    PersistenceDisposition, SqliteCliStore, TransactionalPersistenceAdapter, TransactionalStore,
    sqlite_available,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Database {
    dir: PathBuf,
    path: PathBuf,
}
impl Database {
    fn fresh() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "ai-consulting-lab-persistence-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&dir).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).unwrap();
        }
        let path = dir.join("empirical.sqlite3");
        Self { dir, path }
    }
    fn store(&self) -> SqliteCliStore {
        SqliteCliStore::new(&self.path)
    }
}
impl Drop for Database {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).unwrap();
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
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
fn input() -> BehavioralInput {
    let fixture = frozen("fixtures/synthetic_contract_case.json");
    let case = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["test_id"] == "P0_TRANSACTIONAL_PERSISTENCE")
        .unwrap();
    BehavioralInput {
        fixture_id: case["fixture_id"].as_str().unwrap().into(),
        subject_type: case["subject_type"].as_str().unwrap().into(),
        operation: case["operation"].as_str().unwrap().into(),
        precondition: case["precondition"].as_str().unwrap().into(),
        authority_condition: case["authority_condition"].as_str().unwrap().into(),
        currentness_condition: case["currentness_condition"].as_str().unwrap().into(),
        candidate_behavior: case["candidate_behavior"].as_str().unwrap().into(),
        scenario_data: case["scenario_data"].clone(),
    }
}
fn digest(input: &BehavioralInput) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&input.scenario_data).unwrap())
    )
}
fn satisfies(input: &BehavioralInput, actual: &ObservedBehavior) -> bool {
    actual.input_digest == digest(input)
        && actual.facts["accepted"] == false
        && actual.facts["state_committed"] == false
        && actual.facts["audit_committed"] == false
        && actual.facts["provenance_committed"] == false
}
fn empirically_satisfies(input: &BehavioralInput, actual: &ObservedBehavior) -> bool {
    satisfies(input, actual)
        && actual.facts["classification"] == "RolledBackInjectedFailure"
        && actual.facts["database_atomic_boundary"] == true
        && actual.facts["database_engine"] == "sqlite"
        && actual.facts["database_observed"] == true
}

#[test]
fn frozen_bindings_reconcile_exactly() {
    let manifest = frozen("traceability/manifest.json");
    let binding: Value = serde_json::from_str(
        &fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bindings.json"))
            .unwrap(),
    )
    .unwrap();
    let selected = &binding["selected_contracts"][0];
    let source = &manifest["contracts"][3];
    assert_eq!(source["contract_id"], "BC_TRANSACTIONAL_PERSISTENCE");
    assert_eq!(source["contract_id"], selected["contract_id"]);
    assert_eq!(
        source["requirement_identity"],
        selected["certified_requirement"]
    );
    assert_eq!(source["test_id"], selected["frozen_test_id"]);
    assert_eq!(source["fixture_id"], selected["frozen_fixture_id"]);
    assert_eq!(source["interface"], selected["frozen_interface"]);
    assert_eq!(binding["deferred_contracts"].as_array().unwrap().len(), 6);
}

#[test]
fn frozen_transactional_contract_is_a_database_backed_pass() {
    assert!(sqlite_available());
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let adapter = TransactionalPersistenceAdapter::new(store);
    let actual = adapter.persist_transactionally(&input()).unwrap();
    assert!(empirically_satisfies(&input(), &actual));
    assert_eq!(actual.facts["classification"], "RolledBackInjectedFailure");
    assert_eq!(actual.facts["evidence_class"], "ExpectedDatabaseRejection");
    assert_eq!(actual.facts["database_exit_code"], 19);
    assert!(
        actual.facts["database_stderr"]
            .as_str()
            .unwrap()
            .contains("NOT NULL constraint failed: provenance_records.provenance")
    );
}

#[test]
fn normal_complete_transaction_commits_all_three_records() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let mut valid = input();
    valid.scenario_data["provenance_prepare"] = true.into();
    let result = store.transact(&valid);
    assert_eq!(result.disposition, PersistenceDisposition::Committed);
    assert!(result.state_committed && result.audit_committed && result.provenance_committed);
}

#[test]
fn explicit_and_partial_statement_failures_rollback_without_leaks() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let result = store.transact(&input());
    assert_eq!(
        result.disposition,
        PersistenceDisposition::RolledBackInjectedFailure
    );
    assert!(!result.state_committed && !result.audit_committed && !result.provenance_committed);
}

#[test]
fn explicit_rollback_before_commit_leaves_no_authoritative_state() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let mut rolled_back = input();
    rolled_back.scenario_data["provenance_prepare"] = true.into();
    rolled_back.scenario_data["explicit_rollback"] = true.into();
    let result = store.transact(&rolled_back);
    assert_eq!(
        result.disposition,
        PersistenceDisposition::RolledBackExplicitly
    );
    assert!(!result.state_committed && !result.audit_committed && !result.provenance_committed);
}

#[test]
fn invalid_predecessor_and_provenance_are_deterministic_non_mutating_rejections() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let mut predecessor = input();
    predecessor.scenario_data["predecessor_version"] = 6.into();
    predecessor.scenario_data["provenance_prepare"] = true.into();
    let predecessor_result = store.transact(&predecessor);
    assert_eq!(
        predecessor_result.disposition,
        PersistenceDisposition::RejectedInvalidPredecessor
    );
    assert_eq!(
        predecessor_result
            .database_error
            .as_ref()
            .unwrap()
            .exit_code,
        Some(19)
    );
    assert!(
        predecessor_result
            .database_error
            .as_ref()
            .unwrap()
            .stderr
            .contains("CHECK constraint failed: predecessor_version = 7")
    );
    let mut malformed = input();
    malformed.scenario_data["audit_prepare"] = false.into();
    assert_eq!(
        store.transact(&malformed).disposition,
        PersistenceDisposition::RejectedInvalidProvenance
    );
}

#[test]
fn committed_history_is_database_enforced_append_only() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let mut valid = input();
    valid.scenario_data["provenance_prepare"] = true.into();
    assert_eq!(
        store.transact(&valid).disposition,
        PersistenceDisposition::Committed
    );
    for sql in [
        "UPDATE audit_events SET event_payload='changed';",
        "DELETE FROM provenance_records;",
    ] {
        let status = Command::new("/usr/bin/sqlite3")
            .arg("-batch")
            .arg("-bail")
            .arg(&db.path)
            .arg(sql)
            .status()
            .unwrap();
        assert!(
            !status.success(),
            "append-only mutation unexpectedly succeeded"
        );
    }
}

#[test]
fn replay_is_rejected_without_corrupting_committed_history() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let mut valid = input();
    valid.scenario_data["provenance_prepare"] = true.into();
    assert_eq!(
        store.transact(&valid).disposition,
        PersistenceDisposition::Committed
    );
    let replay = store.transact(&valid);
    assert_eq!(replay.disposition, PersistenceDisposition::RejectedReplay);
    assert!(replay.state_committed && replay.audit_committed && replay.provenance_committed);
    assert!(
        replay
            .database_error
            .as_ref()
            .unwrap()
            .stderr
            .contains("UNIQUE constraint failed: operation_guard.operation_id")
    );
}

#[test]
fn unrelated_database_error_cannot_masquerade_as_expected_rejection() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let trigger = Command::new("/usr/bin/sqlite3")
        .arg("-batch")
        .arg("-bail")
        .arg(&db.path)
        .arg("CREATE TRIGGER unrelated_failure BEFORE INSERT ON provenance_records BEGIN SELECT RAISE(ABORT, 'UNRELATED_DATABASE_ERROR'); END;")
        .status()
        .unwrap();
    assert!(trigger.success());
    let adapter = TransactionalPersistenceAdapter::new(store);
    let actual = adapter.persist_transactionally(&input()).unwrap();
    assert_eq!(actual.facts["classification"], "UnexpectedDatabaseError");
    assert_eq!(actual.facts["evidence_class"], "UnexpectedDatabaseError");
    assert_eq!(actual.facts["database_exit_code"], 19);
    assert!(
        actual.facts["database_stderr"]
            .as_str()
            .unwrap()
            .contains("UNRELATED_DATABASE_ERROR")
    );
    assert!(!empirically_satisfies(&input(), &actual));
}

#[test]
fn frozen_false_pass_families_remain_rejected() {
    let base = input();
    let constant = ObservedBehavior {
        input_digest: "constant".into(),
        facts: json!({}),
    };
    let echo = ObservedBehavior {
        input_digest: digest(&base),
        facts: base.scenario_data.clone(),
    };
    let partial = ObservedBehavior {
        input_digest: digest(&base),
        facts: json!({"accepted":true,"state_committed":true,"audit_committed":false,"provenance_committed":false}),
    };
    assert!(!satisfies(&base, &constant));
    assert!(!satisfies(&base, &echo));
    assert!(!satisfies(&base, &partial));
    let application_only_fake = ObservedBehavior {
        input_digest: digest(&base),
        facts: json!({
            "accepted": false,
            "state_committed": false,
            "audit_committed": false,
            "provenance_committed": false,
            "classification": "ApplicationSequencing",
            "database_atomic_boundary": false,
            "database_engine": null,
            "database_observed": false
        }),
    };
    assert!(!empirically_satisfies(&base, &application_only_fake));
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let adapter = TransactionalPersistenceAdapter::new(store);
    let hardcoded = adapter.persist_transactionally(&base).unwrap();
    let mut varied = base.clone();
    varied.scenario_data["independent_probe_nonce"] = 1.into();
    assert!(!satisfies(&varied, &hardcoded));
    assert!(satisfies(
        &varied,
        &adapter.persist_transactionally(&varied).unwrap()
    ));
}

#[test]
fn database_settings_support_the_bounded_evidence_claim() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let output = Command::new("/usr/bin/sqlite3")
        .arg("-batch")
        .arg(&db.path)
        .arg("PRAGMA journal_mode; PRAGMA synchronous=FULL; PRAGMA synchronous; PRAGMA read_uncommitted;")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "wal\n2\n0\n");
}

#[test]
fn six_downstream_contracts_remain_explicitly_absent() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    let adapter = TransactionalPersistenceAdapter::new(store);
    let case = input();
    let results = [
        adapter.enforce_global_uniqueness(&case),
        adapter.propagate_revocation_and_freshness(&case),
        adapter.attest_runtime_isolation(&case),
        adapter.recover_and_reconcile(&case),
        adapter.emit_audit_evidence(&case),
        adapter.certify_empirically(&case),
    ];
    assert!(
        results
            .into_iter()
            .all(|result| result == Err(ContractError::ExpectedFailNotImplemented))
    );
}
