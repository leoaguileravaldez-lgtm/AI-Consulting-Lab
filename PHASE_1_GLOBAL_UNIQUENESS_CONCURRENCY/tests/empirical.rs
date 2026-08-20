use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, FoundationalContracts, ObservedBehavior,
};
use ai_consulting_lab_phase1_global_uniqueness_concurrency::{
    ConcurrencyDisposition, FirstWriter, GlobalUniquenessAdapter, SqliteConcurrencyStore,
    infrastructure_available,
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
            "ai-consulting-lab-concurrency-{}-{}",
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
    fn store(&self) -> SqliteConcurrencyStore {
        SqliteConcurrencyStore::new(&self.path)
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
        .find(|v| v["test_id"] == "P0_GLOBAL_UNIQUENESS_CONCURRENCY")
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
fn frozen_satisfies(input: &BehavioralInput, actual: &ObservedBehavior) -> bool {
    actual.input_digest == digest(input)
        && actual.facts["accepted"] == false
        && actual.facts["stored_sequence"] == input.scenario_data["expected_sequence"]
        && actual.facts["stored_fence"] == input.scenario_data["current_fence"]
        && actual.facts["successor_created"] == false
}
fn empirical_satisfies(input: &BehavioralInput, actual: &ObservedBehavior) -> bool {
    frozen_satisfies(input, actual)
        && actual.facts["classification"] == "ExpectedStaleWriteRejection"
        && actual.facts["database_cas"] == true
        && actual.facts["database_engine"] == "sqlite"
}
fn seeded_current() -> (Database, SqliteConcurrencyStore) {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    store.seed("synthetic-successor", 8, 12).unwrap();
    (db, store)
}

#[test]
fn frozen_bindings_and_certified_predecessors_reconcile() {
    let manifest = frozen("traceability/manifest.json");
    let binding: Value = serde_json::from_str(
        &fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bindings.json"))
            .unwrap(),
    )
    .unwrap();
    let source = &manifest["contracts"][4];
    let selected = &binding["selected_contracts"][0];
    assert_eq!(source["contract_id"], "BC_GLOBAL_UNIQUENESS_CONCURRENCY");
    assert_eq!(source["contract_id"], selected["contract_id"]);
    assert_eq!(
        source["requirement_identity"],
        selected["certified_requirement"]
    );
    assert_eq!(source["test_id"], selected["frozen_test_id"]);
    assert_eq!(source["fixture_id"], selected["frozen_fixture_id"]);
    assert_eq!(source["interface"], selected["frozen_interface"]);
    assert_eq!(
        binding["certified_predecessors"],
        json!([
            "BC_IDENTITY_VERSION_PROVENANCE",
            "BC_TRANSACTIONAL_PERSISTENCE"
        ])
    );
    assert_eq!(binding["deferred_contracts"].as_array().unwrap().len(), 5);
}

#[test]
fn frozen_stale_writer_contract_is_database_backed_pass() {
    assert!(infrastructure_available());
    let (_db, store) = seeded_current();
    let adapter = GlobalUniquenessAdapter::new(store);
    let actual = adapter.enforce_global_uniqueness(&input()).unwrap();
    assert!(empirical_satisfies(&input(), &actual));
}

#[test]
fn two_overlapping_independent_contenders_produce_exactly_one_winner() {
    for first in [FirstWriter::A, FirstWriter::B] {
        let db = Database::fresh();
        let store = db.store();
        store.bootstrap().unwrap();
        store.seed("synthetic-successor", 7, 11).unwrap();
        let race = store.race_two_contenders(first).unwrap();
        assert_eq!(race.independent_connections, 2);
        assert!(race.overlapping_snapshots);
        assert_eq!(race.snapshot_a, (7, 11));
        assert_eq!(race.snapshot_b, (7, 11));
        assert_ne!(race.winner, race.loser);
        assert_eq!(
            race.loser_disposition,
            ConcurrencyDisposition::ExpectedConcurrencyRejection
        );
        assert_eq!(
            (race.final_sequence, race.final_fence, race.history_count),
            (8, 12, 1)
        );
        assert!(race.database_enforced);
        assert_eq!(race.loser_error.exit_code, Some(1));
        assert!(race.loser_error.stderr.contains("database is locked (5)"));
    }
}

#[test]
fn stale_writer_is_rejected_by_atomic_database_cas() {
    let (_db, store) = seeded_current();
    let result = store.cas("stale", "synthetic-successor", 7, 11, 8, 12);
    assert_eq!(
        result.disposition,
        ConcurrencyDisposition::ExpectedStaleWriteRejection
    );
    assert_eq!(
        (
            result.stored_sequence,
            result.stored_fence,
            result.history_count
        ),
        (8, 12, 1)
    );
    assert!(!result.successor_created);
}

#[test]
fn duplicate_identity_is_database_rejected_without_history_damage() {
    let (_db, store) = seeded_current();
    let result = store.attempt_duplicate_domain("synthetic-successor");
    assert_eq!(
        result.disposition,
        ConcurrencyDisposition::ExpectedUniquenessRejection
    );
    assert_eq!(
        (
            result.stored_sequence,
            result.stored_fence,
            result.history_count
        ),
        (8, 12, 1)
    );
    let error = result.database_error.unwrap();
    assert_eq!(error.exit_code, Some(19));
    assert!(
        error
            .stderr
            .contains("UNIQUE constraint failed: uniqueness_domains.domain")
    );
}

#[test]
fn replay_is_rejected_and_winner_history_is_preserved() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    store.seed("synthetic-successor", 7, 11).unwrap();
    let winner = store.cas("operation-one", "synthetic-successor", 7, 11, 8, 12);
    assert_eq!(winner.disposition, ConcurrencyDisposition::Committed);
    let replay = store.cas("operation-one", "synthetic-successor", 8, 12, 9, 13);
    assert_eq!(
        replay.disposition,
        ConcurrencyDisposition::ExpectedReplayRejection
    );
    assert_eq!(
        (
            replay.stored_sequence,
            replay.stored_fence,
            replay.history_count
        ),
        (8, 12, 1)
    );
}

#[test]
fn unrelated_database_error_is_not_laundered_into_pass() {
    let db = Database::fresh();
    let store = db.store();
    store.bootstrap().unwrap();
    store.seed("synthetic-successor", 7, 11).unwrap();
    let status=Command::new("/usr/bin/sqlite3").arg("-batch").arg("-bail").arg(&db.path)
        .arg("CREATE TRIGGER unrelated BEFORE UPDATE ON uniqueness_domains BEGIN SELECT RAISE(ABORT,'UNRELATED_DATABASE_ERROR'); END;").status().unwrap();
    assert!(status.success());
    let result = store.cas("operation", "synthetic-successor", 7, 11, 8, 12);
    assert_eq!(
        result.disposition,
        ConcurrencyDisposition::UnexpectedDatabaseError
    );
    assert_eq!(
        (
            result.stored_sequence,
            result.stored_fence,
            result.history_count
        ),
        (7, 11, 0)
    );
    assert!(
        result
            .database_error
            .unwrap()
            .stderr
            .contains("UNRELATED_DATABASE_ERROR")
    );
}

#[test]
fn fixed_false_pass_families_are_rejected() {
    let base = input();
    let constant = ObservedBehavior {
        input_digest: "constant".into(),
        facts: json!({}),
    };
    let echo = ObservedBehavior {
        input_digest: digest(&base),
        facts: base.scenario_data.clone(),
    };
    let last_write = ObservedBehavior {
        input_digest: digest(&base),
        facts: json!({"accepted":true,"stored_sequence":8,"stored_fence":12,"successor_created":true,"classification":"Committed","database_cas":false}),
    };
    let application_only = ObservedBehavior {
        input_digest: digest(&base),
        facts: json!({"accepted":false,"stored_sequence":8,"stored_fence":12,"successor_created":false,"classification":"ApplicationOnly","database_cas":false,"database_engine":null}),
    };
    assert!(!empirical_satisfies(&base, &constant));
    assert!(!empirical_satisfies(&base, &echo));
    assert!(!empirical_satisfies(&base, &last_write));
    assert!(!empirical_satisfies(&base, &application_only));
    let (_db, store) = seeded_current();
    let adapter = GlobalUniquenessAdapter::new(store);
    let hardcoded = adapter.enforce_global_uniqueness(&base).unwrap();
    let mut varied = base.clone();
    varied.scenario_data["independent_probe_nonce"] = 1.into();
    assert!(!frozen_satisfies(&varied, &hardcoded));
    assert!(empirical_satisfies(
        &varied,
        &adapter.enforce_global_uniqueness(&varied).unwrap()
    ));
}

#[test]
fn five_downstream_contracts_remain_explicitly_absent() {
    let (_db, store) = seeded_current();
    let adapter = GlobalUniquenessAdapter::new(store);
    let case = input();
    let results = [
        adapter.propagate_revocation_and_freshness(&case),
        adapter.attest_runtime_isolation(&case),
        adapter.recover_and_reconcile(&case),
        adapter.emit_audit_evidence(&case),
        adapter.certify_empirically(&case),
    ];
    assert!(
        results
            .into_iter()
            .all(|r| r == Err(ContractError::ExpectedFailNotImplemented))
    );
}
