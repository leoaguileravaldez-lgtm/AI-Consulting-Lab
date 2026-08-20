use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, FoundationalContracts, ObservedBehavior,
};
use ai_consulting_lab_phase1_runtime_isolation::{
    IsolationDisposition, RuntimeIsolationAdapter, RuntimeIsolationBoundary, bootstrap_authority,
    infrastructure_adequate, opaque_token, set_revoked,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Env {
    dir: PathBuf,
    db: PathBuf,
    resources: PathBuf,
    token_a: String,
}
impl Env {
    fn fresh() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "ai-consulting-lab-isolation-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        let resources = dir.join("resources");
        let db = dir.join("authority.sqlite3");
        fs::create_dir_all(resources.join("client-A")).unwrap();
        fs::create_dir_all(resources.join("client-B")).unwrap();
        fs::write(resources.join("client-A/.domain"), "client-A").unwrap();
        fs::write(resources.join("client-B/.domain"), "client-B").unwrap();
        fs::write(resources.join("client-A/object.txt"), "A-PRIVATE").unwrap();
        fs::write(resources.join("client-B/object.txt"), "B-PRIVATE").unwrap();
        let token_a = opaque_token("A", NEXT.fetch_add(1, Ordering::SeqCst));
        let token_b = opaque_token("B", NEXT.fetch_add(1, Ordering::SeqCst));
        bootstrap_authority(&db, &token_a, &token_b).unwrap();
        Self {
            dir,
            db,
            resources,
            token_a,
        }
    }
    fn boundary(&self) -> RuntimeIsolationBoundary {
        RuntimeIsolationBoundary::new(
            env!("CARGO_BIN_EXE_isolation_worker"),
            &self.db,
            &self.resources,
        )
    }
}
impl Drop for Env {
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
    let c = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["test_id"] == "P0_RUNTIME_ISOLATION")
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
        && o.facts["accepted"] == false
        && o.facts["boundary_crossed"] == false
        && o.facts["effective_actor"] == i.scenario_data["actor_type"]
        && o.facts["classification"] == "ExpectedForeignContextRejection"
        && o.facts["separate_process"] == true
        && o.facts["database_bound_capability"] == true
}

#[test]
fn bindings_and_certified_predecessors_reconcile() {
    let m = frozen("traceability/manifest.json");
    let b: Value = serde_json::from_str(
        &fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bindings.json"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        m["contracts"][6]["contract_id"],
        b["selected_contracts"][0]["contract_id"]
    );
    assert_eq!(
        m["contracts"][6]["requirement_identity"],
        b["selected_contracts"][0]["certified_requirement"]
    );
    assert_eq!(b["deferred_contracts"].as_array().unwrap().len(), 3);
}
#[test]
fn own_context_access_uses_separate_worker_and_bound_capability() {
    assert!(infrastructure_adequate());
    let e = Env::fresh();
    let r = e.boundary().access(
        &e.token_a,
        "client-A",
        "MACHINE",
        "CURRENT",
        "client-A",
        "object.txt",
    );
    assert_eq!(r.disposition, IsolationDisposition::AcceptedOwnContext);
    assert_eq!(r.resource_value.as_deref(), Some("A-PRIVATE"));
}
#[test]
fn frozen_cross_client_request_is_denied_without_actor_amplification() {
    let e = Env::fresh();
    let a = RuntimeIsolationAdapter::new(e.boundary(), &e.token_a);
    let i = input();
    let o = a.attest_runtime_isolation(&i).unwrap();
    assert!(empirical(&i, &o));
    assert_eq!(o.facts["effective_actor"], "MACHINE");
}
#[test]
fn identity_and_resource_substitution_are_rejected() {
    let e = Env::fresh();
    let b = e.boundary();
    assert_eq!(
        b.access(
            &e.token_a,
            "client-B",
            "MACHINE",
            "CURRENT",
            "client-B",
            "object.txt"
        )
        .disposition,
        IsolationDisposition::ExpectedAuthorityRejection
    );
    assert_eq!(
        b.access(
            &e.token_a,
            "client-A",
            "HUMAN",
            "CURRENT",
            "client-A",
            "object.txt"
        )
        .disposition,
        IsolationDisposition::ExpectedAuthorityRejection
    );
    assert_eq!(
        b.access(
            &e.token_a,
            "client-A",
            "MACHINE",
            "CURRENT",
            "client-A",
            "../client-B"
        )
        .disposition,
        IsolationDisposition::ExpectedIsolationRejection
    );
}
#[test]
fn stale_or_revoked_capability_cannot_bridge_boundary() {
    let e = Env::fresh();
    set_revoked(&e.db, &e.token_a).unwrap();
    let r = e.boundary().access(
        &e.token_a,
        "client-A",
        "MACHINE",
        "REVOKED",
        "client-B",
        "object.txt",
    );
    assert_eq!(
        r.disposition,
        IsolationDisposition::ExpectedAuthorityRejection
    );
    assert_eq!(
        fs::read_to_string(e.resources.join("client-B/object.txt")).unwrap(),
        "B-PRIVATE"
    );
}
#[test]
fn rejected_and_failed_workloads_do_not_contaminate_foreign_state() {
    let e = Env::fresh();
    let before = fs::read(e.resources.join("client-B/object.txt")).unwrap();
    let b = e.boundary();
    let denied = b.access(
        &e.token_a,
        "client-A",
        "MACHINE",
        "CURRENT",
        "client-B",
        "object.txt",
    );
    assert_eq!(
        denied.disposition,
        IsolationDisposition::ExpectedForeignContextRejection
    );
    let failed = b.access(
        &e.token_a,
        "client-A",
        "MACHINE",
        "CURRENT",
        "client-A",
        "missing.txt",
    );
    assert_eq!(failed.disposition, IsolationDisposition::UnexpectedOsError);
    assert_eq!(
        fs::read(e.resources.join("client-B/object.txt")).unwrap(),
        before
    );
}
#[test]
fn false_pass_controls_are_rejected() {
    let i = input();
    let constant = ObservedBehavior {
        input_digest: "constant".into(),
        facts: json!({"accepted":false,"boundary_crossed":false,"effective_actor":"MACHINE","classification":"ExpectedForeignContextRejection","separate_process":false,"database_bound_capability":false}),
    };
    assert!(!empirical(&i, &constant));
    let labels = ObservedBehavior {
        input_digest: digest(&i),
        facts: json!({"accepted":false,"boundary_crossed":false,"effective_actor":"MACHINE","classification":"ExpectedForeignContextRejection","separate_process":false,"database_bound_capability":false}),
    };
    assert!(!empirical(&i, &labels));
    let e = Env::fresh();
    let a = RuntimeIsolationAdapter::new(e.boundary(), &e.token_a);
    let hard = a.attest_runtime_isolation(&i).unwrap();
    let mut varied = i.clone();
    varied.scenario_data["object_client"] = "client-A".into();
    assert!(!empirical(&varied, &hard));
    assert!(!empirical(
        &varied,
        &a.attest_runtime_isolation(&varied).unwrap()
    ));
}
#[test]
fn unexpected_errors_never_become_empirical_pass() {
    let e = Env::fresh();
    let bad = RuntimeIsolationBoundary::new(
        env!("CARGO_BIN_EXE_isolation_worker"),
        e.dir.join("missing.sqlite3"),
        &e.resources,
    );
    let r = bad.access(
        &e.token_a,
        "client-A",
        "MACHINE",
        "CURRENT",
        "client-B",
        "object.txt",
    );
    assert_eq!(r.disposition, IsolationDisposition::UnexpectedDatabaseError);
    assert!(!r.accepted);
}
#[test]
fn three_downstream_contracts_remain_absent() {
    let e = Env::fresh();
    let a = RuntimeIsolationAdapter::new(e.boundary(), &e.token_a);
    let i = input();
    assert_eq!(
        a.recover_and_reconcile(&i),
        Err(ContractError::ExpectedFailNotImplemented)
    );
    assert_eq!(
        a.emit_audit_evidence(&i),
        Err(ContractError::ExpectedFailNotImplemented)
    );
    assert_eq!(
        a.certify_empirically(&i),
        Err(ContractError::ExpectedFailNotImplemented)
    );
}
