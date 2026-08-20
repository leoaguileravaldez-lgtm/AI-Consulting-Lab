use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, FoundationalContracts, ObservedBehavior,
};
use ai_consulting_lab_phase1_revocation_freshness::{
    AuthorityState, RevocationDisposition, RevocationFreshnessAdapter, SqliteRevocationStore,
    infrastructure_adequate,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    process::Command,
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
            "ai-consulting-lab-revocation-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&dir).unwrap();
        let path = dir.join("empirical.sqlite3");
        Self { dir, path }
    }
    fn store(&self) -> SqliteRevocationStore {
        SqliteRevocationStore::new(&self.path)
    }
}
impl Drop for Db {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).unwrap()
    }
}
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
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
        .find(|v| v["test_id"] == "P0_REVOCATION_FRESHNESS")
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
fn satisfies(i: &BehavioralInput, o: &ObservedBehavior) -> bool {
    o.input_digest == digest(i)
        && o.facts["accepted"] == false
        && o.facts["evaluated_revocation_epoch"] == i.scenario_data["revocation_epoch"]
        && o.facts["cache_invalidated"] == true
}
fn empirical(i: &BehavioralInput, o: &ObservedBehavior) -> bool {
    satisfies(i, o)
        && o.facts["classification"] == "ExpectedRevokedRejection"
        && o.facts["authoritative_database"] == true
}
fn current() -> (Db, SqliteRevocationStore) {
    let db = Db::fresh();
    let s = db.store();
    s.bootstrap().unwrap();
    s.seed_current(4).unwrap();
    (db, s)
}

#[test]
fn bindings_and_predecessors_reconcile() {
    let m = frozen("traceability/manifest.json");
    let b: Value = serde_json::from_str(
        &fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bindings.json"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        m["contracts"][5]["contract_id"],
        b["selected_contracts"][0]["contract_id"]
    );
    assert_eq!(
        m["contracts"][5]["requirement_identity"],
        b["selected_contracts"][0]["certified_requirement"]
    );
    assert_eq!(
        b["certified_predecessors"],
        json!([
            "BC_IDENTITY_VERSION_PROVENANCE",
            "BC_TRANSACTIONAL_PERSISTENCE",
            "BC_GLOBAL_UNIQUENESS_CONCURRENCY"
        ])
    );
    assert_eq!(b["deferred_contracts"].as_array().unwrap().len(), 4)
}
#[test]
fn current_before_revocation_then_atomic_revocation() {
    assert!(infrastructure_adequate());
    let (_db, s) = current();
    let before = s.authorize_cached(4);
    assert_eq!(before.disposition, RevocationDisposition::AcceptedCurrent);
    let revoked = s.revoke(4, 5);
    assert_eq!(
        revoked.disposition,
        RevocationDisposition::RevocationCommitted
    );
    assert_eq!(revoked.authority_state, AuthorityState::Revoked);
    assert_eq!(revoked.authoritative_generation, 5);
    assert_eq!(revoked.evaluated_epoch, 5)
}
#[test]
fn frozen_revocation_dominates_stale_cache() {
    let (_db, s) = current();
    assert_eq!(
        s.revoke(4, 5).disposition,
        RevocationDisposition::RevocationCommitted
    );
    let a = RevocationFreshnessAdapter::new(s);
    let actual = a.propagate_revocation_and_freshness(&input()).unwrap();
    assert!(empirical(&input(), &actual))
}
#[test]
fn retry_cannot_resurrect_revoked_authority() {
    let (_db, s) = current();
    s.revoke(4, 5);
    let stale_retry = s.revoke(4, 5);
    assert_eq!(
        stale_retry.disposition,
        RevocationDisposition::ExpectedStaleRejection
    );
    assert_eq!(stale_retry.authority_state, AuthorityState::Revoked);
    assert_eq!(stale_retry.authoritative_generation, 5);
    let r = s.attempt_resurrection();
    assert_eq!(r.disposition, RevocationDisposition::ExpectedStaleRejection);
    assert_eq!(r.authority_state, AuthorityState::Revoked);
    assert_eq!(r.authoritative_generation, 5);
    assert!(r.database_error.unwrap().contains("AUTHORITY_RESURRECTION"))
}
#[test]
fn superseded_generation_rejects_stale_projection() {
    let (_db, s) = current();
    assert_eq!(
        s.supersede(4, 5).disposition,
        RevocationDisposition::RevocationCommitted
    );
    let r = s.authorize_cached(4);
    assert_eq!(
        r.disposition,
        RevocationDisposition::ExpectedSupersededRejection
    );
    assert!(r.cache_invalidated);
    assert_eq!(r.authority_state, AuthorityState::Superseded);
    let replay = s.attempt_resurrection();
    assert_eq!(
        replay.disposition,
        RevocationDisposition::ExpectedStaleRejection
    );
    assert_eq!(replay.authority_state, AuthorityState::Superseded);
    assert_eq!(replay.authoritative_generation, 5);
}
#[test]
fn unknown_never_becomes_current() {
    let db = Db::fresh();
    let s = db.store();
    s.bootstrap().unwrap();
    let r = s.authorize_cached(4);
    assert_eq!(r.disposition, RevocationDisposition::UnknownOrIndeterminate);
    assert_ne!(r.disposition, RevocationDisposition::AcceptedCurrent)
}
#[test]
fn unrelated_database_error_is_not_pass() {
    let (db, s) = current();
    s.revoke(4, 5);
    let ok=Command::new("/usr/bin/sqlite3").arg("-batch").arg("-bail").arg(&db.path).arg("CREATE TRIGGER unrelated BEFORE UPDATE ON projections BEGIN SELECT RAISE(ABORT,'UNRELATED_DATABASE_ERROR'); END;").status().unwrap();
    assert!(ok.success());
    let r = s.authorize_cached(4);
    assert_eq!(
        r.disposition,
        RevocationDisposition::UnexpectedDatabaseError
    );
    assert!(!r.cache_invalidated)
}
#[test]
fn false_pass_controls_rejected() {
    let base = input();
    let constant = ObservedBehavior {
        input_digest: "constant".into(),
        facts: json!({}),
    };
    let cache = ObservedBehavior {
        input_digest: digest(&base),
        facts: json!({"accepted":true,"evaluated_revocation_epoch":4,"cache_invalidated":false,"classification":"CacheCurrent","authoritative_database":false}),
    };
    let unknown = ObservedBehavior {
        input_digest: digest(&base),
        facts: json!({"accepted":true,"evaluated_revocation_epoch":-1,"cache_invalidated":false,"classification":"Unknown","authoritative_database":false}),
    };
    assert!(!empirical(&base, &constant));
    assert!(!empirical(&base, &cache));
    assert!(!empirical(&base, &unknown));
    let (_db, s) = current();
    s.revoke(4, 5);
    let a = RevocationFreshnessAdapter::new(s);
    let hard = a.propagate_revocation_and_freshness(&base).unwrap();
    let mut varied = base.clone();
    varied.scenario_data["independent_probe_nonce"] = 1.into();
    assert!(!satisfies(&varied, &hard));
    assert!(empirical(
        &varied,
        &a.propagate_revocation_and_freshness(&varied).unwrap()
    ))
}
#[test]
fn four_downstream_contracts_absent() {
    let (_db, s) = current();
    let a = RevocationFreshnessAdapter::new(s);
    let i = input();
    let rs = [
        a.attest_runtime_isolation(&i),
        a.recover_and_reconcile(&i),
        a.emit_audit_evidence(&i),
        a.certify_empirically(&i),
    ];
    assert!(
        rs.into_iter()
            .all(|r| r == Err(ContractError::ExpectedFailNotImplemented))
    )
}
