use ai_consulting_lab_phase0_contracts::{BehavioralInput, FoundationalContracts};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
use titus_lab_phase1_empirical_certification::CertificationAdapter;
static NEXT: AtomicU64 = AtomicU64::new(0);
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .into()
}
fn base() -> Value {
    serde_json::from_str(
        &fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("candidate_evidence.json"),
        )
        .unwrap(),
    )
    .unwrap()
}
fn temp_package(v: &Value) -> (PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!(
        "titus-cert-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::SeqCst)
    ));
    fs::create_dir(&dir).unwrap();
    let path = dir.join("package.json");
    fs::write(&path, serde_json::to_vec(v).unwrap()).unwrap();
    (dir, path)
}
fn run(v: &Value) -> (Value, bool) {
    let (dir, path) = temp_package(v);
    let out = Command::new(env!("CARGO_BIN_EXE_independent_validator"))
        .arg(root())
        .arg(path)
        .output()
        .unwrap();
    fs::remove_dir_all(dir).unwrap();
    (
        serde_json::from_slice(&out.stdout).unwrap(),
        out.status.success(),
    )
}
fn run_path(root_path: &Path, package: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_independent_validator"))
        .arg(root_path)
        .arg(package)
        .output()
        .unwrap()
}
fn frozen_input() -> BehavioralInput {
    let v: Value = serde_json::from_str(
        &fs::read_to_string(root().join(
            "PHASE_0_CONTRACT_FIRST_EXECUTABLE_BASELINE/fixtures/synthetic_contract_case.json",
        ))
        .unwrap(),
    )
    .unwrap();
    let c = v["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["test_id"] == "P0_EMPIRICAL_CERTIFICATION")
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
fn input_id(v: &mut Value) {
    let raw = format!(
        "{}|{}|{}|{}",
        v["candidate_commit"].as_str().unwrap(),
        v["contract_id"].as_str().unwrap(),
        v["context_id"].as_str().unwrap(),
        v["generation"].as_i64().unwrap()
    );
    let mut h = Sha256::new();
    h.update(raw);
    v["certification_input_id"] = json!(format!("{:x}", h.finalize()));
}

#[test]
fn frozen_contract_dag_and_predecessors_are_exact() {
    let m: Value = serde_json::from_str(
        &fs::read_to_string(
            root().join("16_OPERATIONAL_REALIZATION_CONFORMANCE_ARCHITECTURE/CANONICAL_MODEL.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        m["object_types"][12]["id"],
        "FUTURE_EMPIRICAL_CERTIFICATION_CONTRACT"
    );
    assert_eq!(m["invariants"][18]["id"], "S");
    assert_eq!(m["transition_rules"][11]["id"], "T12_DEFINE_FUTURE_TESTS");
    assert_eq!(
        m["transition_rules"][11]["sources"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
}
#[test]
fn known_good_exact_package_is_certified() {
    let (v, ok) = run(&base());
    assert!(ok);
    assert_eq!(v["verdict"], "CERTIFIED_PASS");
    assert_eq!(v["independent_expectation"], "ACCEPT");
}
#[test]
fn frozen_material_failure_denies_despite_aggregate_score() {
    let a = CertificationAdapter::new(env!("CARGO_BIN_EXE_independent_validator"), root());
    let input = frozen_input();
    let o = a.certify_empirically(&input).unwrap();
    assert_eq!(o.facts["accepted"], false);
    assert_eq!(o.facts["material_failure_preserved"], true);
    assert_eq!(o.facts["deployment_authority_created"], false);
    assert_eq!(o.facts["aggregate_score_ignored"], true);
    assert_eq!(o.facts["reason"], "MATERIAL_FAILURE_NOT_AGGREGATED_AWAY");
}
#[test]
fn frozen_digest_is_exact_serialized_scenario_sha256_not_debug_subject() {
    let a = CertificationAdapter::new(env!("CARGO_BIN_EXE_independent_validator"), root());
    let input = frozen_input();
    let observed = a.certify_empirically(&input).unwrap();
    let exact = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&input.scenario_data).unwrap())
    );
    let debug_subject = format!("{:x}", Sha256::digest(format!("{:?}", input).as_bytes()));
    assert_eq!(observed.input_digest, exact);
    assert_ne!(observed.input_digest, debug_subject);
}
#[test]
fn wrong_commit_is_invalid_even_with_recomputed_input_id() {
    let mut p = base();
    p["candidate_commit"] = json!("wrong-commit");
    input_id(&mut p);
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "INVALID_EVIDENCE");
}
#[test]
fn wrong_contract_is_invalid_even_with_recomputed_input_id() {
    let mut p = base();
    p["contract_id"] = json!("BC_AUDIT_EVIDENCE");
    input_id(&mut p);
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "INVALID_EVIDENCE");
}
#[test]
fn incomplete_evidence_is_not_certified() {
    let mut p = base();
    p["evidence"].as_array_mut().unwrap().pop();
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "INCOMPLETE_EVIDENCE");
}
#[test]
fn altered_evidence_hash_is_invalid() {
    let mut p = base();
    p["evidence"][6]["sha256"] = json!("altered");
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "INVALID_EVIDENCE");
}
#[test]
fn stale_generation_is_invalid() {
    let mut p = base();
    p["generation"] = json!(0);
    input_id(&mut p);
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "INVALID_EVIDENCE");
}
#[test]
fn cross_context_is_invalid() {
    let mut p = base();
    p["context_id"] = json!("foreign-context");
    input_id(&mut p);
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "INVALID_EVIDENCE");
}
#[test]
fn forged_evidence_provenance_is_invalid() {
    let mut p = base();
    p["evidence"][6]["path"] =
        json!("PHASE_1_RECOVERY_RECONCILIATION/LOCAL_IMPLEMENTATION_EVIDENCE.json");
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "INVALID_EVIDENCE");
}
#[test]
fn duplicate_evidence_identity_is_not_certified() {
    let mut p = base();
    p["evidence"][6] = p["evidence"][5].clone();
    let (v, ok) = run(&p);
    assert!(!ok);
    assert!(v["verdict"] == "INCOMPLETE_EVIDENCE" || v["verdict"] == "INVALID_EVIDENCE");
}
#[test]
fn implementation_declared_pass_cannot_cure_material_failure() {
    let mut p = base();
    p["material_results"][1]["observed"] = json!("FAIL");
    p["aggregate_score"] = json!(100);
    p["implementation_declared_expectation"] = json!("CERTIFIED_PASS");
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "CERTIFIED_FAIL");
    assert_eq!(v["reason"], "MATERIAL_FAILURE_NOT_AGGREGATED_AWAY");
}
#[test]
fn audit_pass_is_input_not_certification_authority() {
    let mut p = base();
    p["material_results"][2]["observed"] = json!("FAIL");
    assert_eq!(p["evidence"][6]["status"], "EMPIRICAL_PASS");
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "CERTIFIED_FAIL");
}
#[test]
fn validator_independence_failure_is_not_certified() {
    let mut p = base();
    p["independence"]["validator_id"] = p["independence"]["producer_id"].clone();
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "INVALID_EVIDENCE");
}
#[test]
fn validator_error_is_not_certified() {
    let out = Command::new(env!("CARGO_BIN_EXE_independent_validator"))
        .output()
        .unwrap();
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(!out.status.success());
    assert_eq!(v["verdict"], "VALIDATOR_FAILURE");
}
#[test]
fn infrastructure_error_is_not_certified() {
    let missing = std::env::temp_dir().join("titus-cert-missing-package.json");
    let out = run_path(&root(), &missing);
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(!out.status.success());
    assert_eq!(v["verdict"], "INFRASTRUCTURE_FAILURE");
}
#[test]
fn certified_fail_cannot_substitute_for_missing_frozen_authority_facts() {
    let mut p = base();
    p["material_results"][0]["observed"] = json!("FAIL");
    p.as_object_mut().unwrap().remove("authority_observations");
    let (v, ok) = run(&p);
    assert!(!ok);
    assert_eq!(v["verdict"], "INCOMPLETE_EVIDENCE");
    assert_eq!(v["reason"], "AUTHORITY_OBSERVATION_MISSING");
}
#[test]
fn certification_has_zero_operational_authority() {
    let (v, ok) = run(&base());
    assert!(ok);
    assert_eq!(
        (
            v["certification_authority_laundering"].as_i64(),
            v["execution_authority_leakage"].as_i64(),
            v["retry_authority_leakage"].as_i64(),
            v["recovery_authority_leakage"].as_i64(),
            v["layer19_operational_authority_leakage"].as_i64(),
            v["deployment_authority_leakage"].as_i64()
        ),
        (Some(0), Some(0), Some(0), Some(0), Some(0), Some(0))
    );
}
#[test]
fn deterministic_verdict_fingerprint_and_cleanup() {
    let (a, ok1) = run(&base());
    let (b, ok2) = run(&base());
    assert!(ok1 && ok2);
    assert_eq!(a, b);
}
