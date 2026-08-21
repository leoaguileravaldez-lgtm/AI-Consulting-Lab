use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
use sha2::Digest;
use titus_lab_integration_stage_1_kernel::*;

static NEXT: AtomicU64 = AtomicU64::new(0);

fn setup(label: &str) -> (PathBuf, IntegrationKernel<DeterministicLocalExecutor>) {
    let root = std::env::temp_dir().join(format!(
        "titus-stage1-test-{}-{label}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::SeqCst)
    ));
    let kernel =
        IntegrationKernel::create(&root, "synthetic-client-A", 1, DeterministicLocalExecutor)
            .unwrap();
    (root, kernel)
}

#[test]
fn complete_synthetic_path_reaches_human_gate() {
    let (root, kernel) = setup("success");
    let r = kernel.execute(&synthetic_request());
    assert_eq!(
        r.classification,
        IntegrationClassification::ValidationRequired
    );
    assert_eq!(r.stored_sequence, 2);
    assert_eq!(r.provenance.len(), 8);
    assert!(r.audit_record_hash.is_some());
    assert_eq!(
        r.validation_handoff.as_ref().unwrap().status,
        "VALIDATION_REQUIRED"
    );
    assert_ne!(
        r.validation_handoff.as_ref().unwrap().producer_id,
        r.validation_handoff.as_ref().unwrap().validator_id
    );
    assert_eq!(
        r.human_gate,
        None
    );
    assert!(!r.recovery_required && !r.authority_created && !r.external_effect_truth_determined);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn wrong_context_is_rejected() {
    let (root, kernel) = setup("context");
    let mut q = synthetic_request();
    q.presented_context = "synthetic-client-B".into();
    assert_eq!(
        kernel.execute(&q).classification,
        IntegrationClassification::InvalidContext
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn missing_authority_is_rejected() {
    let (root, kernel) = setup("missing-authority");
    let mut q = synthetic_request();
    q.envelope.authority_ref.clear();
    assert_eq!(
        kernel.execute(&q).classification,
        IntegrationClassification::MissingAuthority
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn out_of_scope_authority_is_rejected() {
    let (root, kernel) = setup("scope");
    let mut q = synthetic_request();
    q.envelope.authority_scope = "integration:read".into();
    assert_eq!(
        kernel.execute(&q).classification,
        IntegrationClassification::PolicyRejected
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn revoked_authority_is_rejected() {
    let (root, kernel) = setup("revoked");
    assert!(kernel.revoke_authority(2));
    assert_eq!(
        kernel.execute(&synthetic_request()).classification,
        IntegrationClassification::StaleOrRevokedAuthority
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn execution_failure_is_not_success() {
    let (root, kernel) = setup("execution-failure");
    let mut q = synthetic_request();
    q.fault = SyntheticFault::ExecutionBehavior;
    assert_eq!(
        kernel.execute(&q).classification,
        IntegrationClassification::ExecutionBehavioralFailure
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn predecessor_conflict_is_not_success() {
    let (root, kernel) = setup("conflict");
    let mut q = synthetic_request();
    q.envelope.predecessor_sequence = 0;
    assert_eq!(
        kernel.execute(&q).classification,
        IntegrationClassification::StateConflict
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn audit_failure_is_not_success_or_authority() {
    let (root, kernel) = setup("audit-failure");
    let mut q = synthetic_request();
    q.fault = SyntheticFault::AuditContext;
    let r = kernel.execute(&q);
    assert_eq!(r.classification, IntegrationClassification::AuditFailure);
    assert!(r.recovery_required);
    assert!(!r.authority_created && !r.external_effect_truth_determined);
    assert!(r.validation_handoff.is_none() && r.human_gate.is_none());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validation_failure_is_not_success_or_certification() {
    let (root, kernel) = setup("validation-failure");
    let mut q = synthetic_request();
    q.fault = SyntheticFault::ValidationUnavailable;
    let r = kernel.execute(&q);
    assert_eq!(
        r.classification,
        IntegrationClassification::ValidationFailure
    );
    assert!(r.recovery_required);
    assert!(r.validation_handoff.is_none() && r.human_gate.is_none());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn human_principal_bypass_is_blocked() {
    let (root, kernel) = setup("human-bypass");
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let validator = PathBuf::from(env!("CARGO_BIN_EXE_stage1_independent_validator"));
    let r = kernel.execute(&synthetic_request());
    let decision = HumanDecision { actor_id: "human-principal:titus-lab".into(), result_id: r.result_id.clone().unwrap(), approve: true };
    let denied = kernel.apply_human_decision(&r, Some(&decision));
    assert_eq!(
        denied.classification,
        IntegrationClassification::HumanPrincipalBypassBlocked
    );
    assert_eq!(denied.human_gate, None);

    let (root_fail, kernel_fail) = setup("human-certified-fail");
    let mut certified_fail = kernel_fail.execute(&synthetic_request());
    let mut bad_package = construct_stage1_candidate_package(&repository_root, &certified_fail).unwrap();
    bad_package["material_observations"]["authority"] = serde_json::json!("FAIL");
    bad_package.as_object_mut().unwrap().remove("package_fingerprint");
    let bad_bytes = serde_json::to_vec(&bad_package).unwrap();
    bad_package["package_fingerprint"] = serde_json::json!(format!("{:x}", sha2::Sha256::digest(bad_bytes)));
    assert!(complete_validation_handoff_with_package(&repository_root, &validator, &mut certified_fail, bad_package).is_err());
    assert_eq!(certified_fail.human_gate, None);

    let (root_invalid, kernel_invalid) = setup("human-invalid");
    let mut invalid = kernel_invalid.execute(&synthetic_request());
    let mut invalid_package = construct_stage1_candidate_package(&repository_root, &invalid).unwrap();
    invalid_package["context_id"] = serde_json::json!("foreign-context");
    assert!(complete_validation_handoff_with_package(&repository_root, &validator, &mut invalid, invalid_package).is_err());
    assert_eq!(invalid.human_gate, None);

    let (root_pass, kernel_pass) = setup("human-pass");
    let mut validated = kernel_pass.execute(&synthetic_request());
    complete_validation_handoff(&repository_root, &validator, &mut validated).unwrap();
    assert_eq!(validated.classification, IntegrationClassification::AwaitHumanPrincipal);
    assert_eq!(validated.human_gate, Some(HumanGateState::AwaitingHumanPrincipalDecision));
    let approved = kernel_pass.apply_human_decision(&validated, Some(&HumanDecision { actor_id: "human-principal:titus-lab".into(), result_id: validated.result_id.clone().unwrap(), approve: true }));
    assert_eq!(approved.human_gate, Some(HumanGateState::ApprovedByHumanPrincipal));

    let mut stale = validated.clone();
    stale.result_id = Some("changed-candidate".into());
    assert_eq!(kernel_pass.apply_human_decision(&stale, Some(&HumanDecision { actor_id: "human-principal:titus-lab".into(), result_id: "changed-candidate".into(), approve: true })).classification, IntegrationClassification::HumanPrincipalBypassBlocked);
    fs::remove_dir_all(root).unwrap();
    fs::remove_dir_all(root_fail).unwrap();
    fs::remove_dir_all(root_invalid).unwrap();
    fs::remove_dir_all(root_pass).unwrap();
}

#[test]
fn explicit_human_decision_changes_only_gate_state() {
    let (root, kernel) = setup("human-decision");
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let validator = PathBuf::from(env!("CARGO_BIN_EXE_stage1_independent_validator"));
    let mut r = kernel.execute(&synthetic_request());
    complete_validation_handoff(&repository_root, &validator, &mut r).unwrap();
    let decision = HumanDecision {
        actor_id: "human-principal:titus-lab".into(),
        result_id: r.result_id.clone().unwrap(),
        approve: true,
    };
    let approved = kernel.apply_human_decision(&r, Some(&decision));
    assert_eq!(
        approved.human_gate,
        Some(HumanGateState::ApprovedByHumanPrincipal)
    );
    assert_eq!(approved.result_id, r.result_id);
    assert!(!approved.authority_created && !approved.external_effect_truth_determined);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn identities_results_provenance_and_handoffs_are_deterministic() {
    let (root_a, kernel_a) = setup("deterministic-a");
    let (root_b, kernel_b) = setup("deterministic-b");
    let a = kernel_a.execute(&synthetic_request());
    let b = kernel_b.execute(&synthetic_request());
    assert_eq!(a, b);
    fs::remove_dir_all(root_a).unwrap();
    fs::remove_dir_all(root_b).unwrap();
}

#[test]
fn infrastructure_setup_failure_is_explicit_non_success() {
    let root = std::env::temp_dir().join(format!(
        "titus-stage1-test-{}-infrastructure-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::SeqCst)
    ));
    fs::write(&root, b"not-a-directory").unwrap();
    let error =
        IntegrationKernel::create(&root, "synthetic-client-A", 1, DeterministicLocalExecutor)
            .err()
            .unwrap();
    assert_eq!(
        error.classification,
        IntegrationClassification::InfrastructureFailure
    );
    fs::remove_file(root).unwrap();
}

#[test]
fn truthful_candidate_package_is_deterministic_and_consumed_by_independent_validator() {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let (root_a, kernel_a) = setup("validator-a");
    let (root_b, kernel_b) = setup("validator-b");
    let mut a = kernel_a.execute(&synthetic_request());
    let b = kernel_b.execute(&synthetic_request());
    let package_a = construct_stage1_candidate_package(&repository_root, &a).unwrap();
    let package_b = construct_stage1_candidate_package(&repository_root, &b).unwrap();
    assert_eq!(package_a, package_b);
    let validator = PathBuf::from(env!("CARGO_BIN_EXE_stage1_independent_validator"));
    let verdict = complete_validation_handoff(&repository_root, &validator, &mut a).unwrap();
    assert_eq!(verdict["verdict"], "CERTIFIED_PASS");
    assert_eq!(a.validation_handoff.as_ref().unwrap().status, "INDEPENDENT_VALIDATION_COMPLETE");
    assert_eq!(a.human_gate, Some(HumanGateState::AwaitingHumanPrincipalDecision));
    fs::remove_dir_all(root_a).unwrap();
    fs::remove_dir_all(root_b).unwrap();
}
