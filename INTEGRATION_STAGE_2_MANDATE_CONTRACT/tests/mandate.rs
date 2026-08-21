use titus_lab_integration_stage_2_engagement_domain::{
    all_operations, client_identity, AuthorityGrant, EngagementDomain, EngagementType,
    WorkingLanguage,
};
use titus_lab_integration_stage_2_mandate_contract::*;
use sha2::{Digest, Sha256};

fn expected_handoff_id(engagement: &str, mandate: &str, version: u64,
    context: &str, generation: u64) -> String {
    let mut hash = Sha256::new();
    hash.update(b"titus-lab-stage2-mandate-decision-problem-handoff-v1\0");
    let version = version.to_string();
    let generation = generation.to_string();
    for part in [engagement, mandate, version.as_str(), context, generation.as_str()] {
        hash.update(part.as_bytes());
        hash.update([0]);
    }
    format!("{:x}", hash.finalize())
}

fn authority(client: &str, engagement: Option<&str>, generation: u64) -> AuthorityGrant {
    let mut operations = all_operations();
    operations.extend(mandate_operations());
    AuthorityGrant {
        context_id: "synthetic-client-A".into(),
        client_id: client.into(),
        engagement_id: engagement.map(str::to_owned),
        operations,
        generation,
        current: true,
    }
}

fn engagement_domain() -> (EngagementDomain, String, String, String) {
    let mut domain = EngagementDomain::default();
    let client = client_identity(
        "synthetic-client-A", "Synthetic Industrial Client", "synthetic:client-source",
    );
    domain.create_client(
        "synthetic-client-A", "Synthetic Industrial Client", "ORGANIZATION",
        "synthetic:client-source", &authority(&client, None, 1),
    ).unwrap();
    let first = domain.create_engagement(
        &client, "Operational Improvement Assessment", EngagementType::Operations,
        WorkingLanguage::Es, "synthetic-human-principal", "synthetic:engagement-a",
        &authority(&client, None, 1),
    ).unwrap();
    let second = domain.create_engagement(
        &client, "Separate Assessment", EngagementType::Research,
        WorkingLanguage::En, "synthetic-human-principal", "synthetic:engagement-b",
        &authority(&client, None, 1),
    ).unwrap();
    (domain, client, first, second)
}

fn source() -> OriginalMandateInput {
    OriginalMandateInput {
        source_ref: "synthetic://client-mandate/v1".into(),
        exact_text: "Evaluate cycle-time evidence and decide whether to pilot.".into(),
        source_language: "EN".into(),
        provenance: "client-supplied:synthetic".into(),
    }
}

fn content() -> MandateContent {
    MandateContent {
        what_statement: "Evaluate a bounded operational improvement opportunity".into(),
        why_statement: "Support an evidence-based pilot decision".into(),
        in_scope: vec!["cycle-time assessment".into()],
        out_of_scope: vec!["production deployment".into()],
        unresolved: vec!["site expansion".into()],
        constraints: vec!["synthetic data only".into()],
        required_decision: "PILOT_OR_STOP".into(),
        required_outputs: vec!["operational assessment".into()],
        evidence_standard: "ATTRIBUTABLE_CURRENT_SYNTHETIC_EVIDENCE".into(),
        materiality: Materiality::Material,
        risk: RiskClassification::Moderate,
        progression_approvers: vec![HUMAN_PRINCIPAL.into()],
        completion_criteria: vec!["decision packet ready".into()],
    }
}

fn registry() -> (EngagementDomain, MandateRegistry, String, String, String) {
    let (domain, client, first, second) = engagement_domain();
    let mut mandates = MandateRegistry::default();
    let mandate = mandates.create(
        domain.engagement(&first).unwrap(), source(), content(), HUMAN_PRINCIPAL,
        "synthetic-client-A", "titus-derived:structured-mandate",
        &authority(&client, Some(&first), 1),
    ).unwrap();
    (domain, mandates, client, first, mandate + &format!("|{second}"))
}

fn split_ids(combined: &str) -> (&str, &str) { combined.split_once('|').unwrap() }

#[test]
fn valid_mandate_creation_and_handoff_pass() {
    let (domain, mandates, _, engagement, combined) = registry();
    let (mandate_id, _) = split_ids(&combined);
    let mandate = mandates.current(&engagement).unwrap();
    assert_eq!(mandate.engagement_id(), engagement);
    assert_eq!(mandate.version(), 1);
    assert_eq!(mandate.working_language(), WorkingLanguage::Es);
    let handoff = mandates.decision_problem_handoff(
        domain.engagement(&engagement).unwrap(), mandate_id, 1,
    ).unwrap();
    assert_eq!(handoff.mandate_id(), mandate_id);
    assert_eq!(handoff.authority_ref, HUMAN_PRINCIPAL);
    assert_eq!(handoff.authority_context_id(), "synthetic-client-A");
    assert_eq!(handoff.handoff_id(), expected_handoff_id(
        &engagement, mandate_id, 1, handoff.authority_context_id(), 1,
    ));
    assert_eq!(handoff.engagement_id(), engagement);
    assert_eq!(handoff.mandate_version(), 1);
    assert_eq!(handoff.authority_generation(), 1);
}

#[test]
fn cross_engagement_and_same_client_substitution_are_rejected() {
    let (domain, mut mandates, client, engagement, combined) = registry();
    let (mandate_id, other) = split_ids(&combined);
    let result = mandates.amend(
        domain.engagement(other).unwrap(), mandate_id, 1, content(), None,
        HUMAN_PRINCIPAL, "foreign", &authority(&client, Some(other), 1),
    );
    assert_eq!(result, Err(MandateError::CrossEngagementSubstitution));
    assert_eq!(mandates.decision_problem_handoff(
        domain.engagement(other).unwrap(), mandate_id, 1,
    ), Err(MandateError::CrossEngagementSubstitution));
    assert_eq!(mandates.current(&engagement).unwrap().engagement_id(), engagement);
}

#[test]
fn stale_mandate_update_is_rejected_and_history_is_preserved() {
    let (domain, mut mandates, client, engagement, combined) = registry();
    let (mandate_id, _) = split_ids(&combined);
    let grant = authority(&client, Some(&engagement), 1);
    mandates.amend(domain.engagement(&engagement).unwrap(), mandate_id, 1,
        content(), None, HUMAN_PRINCIPAL, "authorized:v2", &grant).unwrap();
    assert_eq!(mandates.amend(domain.engagement(&engagement).unwrap(), mandate_id, 1,
        content(), None, HUMAN_PRINCIPAL, "stale", &grant), Err(MandateError::StaleWrite));
    assert_eq!(mandates.current(&engagement).unwrap().predecessor_version(), Some(1));
    assert_eq!(mandates.history(&engagement).unwrap()[0].status(), MandateStatus::Superseded);
}

#[test]
fn unauthorized_scope_expansion_is_rejected() {
    let (_, mandates, _, engagement, combined) = registry();
    let (mandate_id, _) = split_ids(&combined);
    assert_eq!(mandates.authorize_request(&engagement, mandate_id, 1,
        "cycle-time assessment"), Ok(()));
    assert_eq!(mandates.authorize_request(&engagement, mandate_id, 1,
        "production deployment"), Err(MandateError::ScopeExpansionRejected));
    assert_eq!(mandates.authorize_request(&engagement, mandate_id, 1,
        "site expansion"), Err(MandateError::ScopeExpansionRejected));
    assert_eq!(mandates.authorize_request(&engagement, mandate_id, 1,
        "unlisted work"), Err(MandateError::ScopeExpansionRejected));
}

#[test]
fn invalid_lifecycle_transition_is_rejected() {
    let (domain, mut mandates, client, engagement, combined) = registry();
    let (mandate_id, _) = split_ids(&combined);
    let grant = authority(&client, Some(&engagement), 1);
    mandates.close(domain.engagement(&engagement).unwrap(), mandate_id, 1,
        HUMAN_PRINCIPAL, "close", &grant).unwrap();
    assert_eq!(mandates.close(domain.engagement(&engagement).unwrap(), mandate_id, 2,
        HUMAN_PRINCIPAL, "close-again", &grant), Err(MandateError::InvalidTransition));
}

#[test]
fn working_language_change_preserves_original_source() {
    let (mut domain, mut mandates, client, engagement, combined) = registry();
    let (mandate_id, _) = split_ids(&combined);
    let before = mandates.current(&engagement).unwrap().original_inputs()[0].clone();
    domain.change_working_language(&engagement, 1, WorkingLanguage::En,
        "language-change", &authority(&client, Some(&engagement), 1)).unwrap();
    mandates.refresh_working_language(domain.engagement(&engagement).unwrap(), mandate_id, 1,
        HUMAN_PRINCIPAL, "language-reference", &authority(&client, Some(&engagement), 1)).unwrap();
    let current = mandates.current(&engagement).unwrap();
    assert_eq!(current.working_language(), WorkingLanguage::En);
    assert_eq!(&current.original_inputs()[0], &before);
}

#[test]
fn mandate_cannot_amplify_non_human_authority() {
    let (domain, client, engagement, _) = engagement_domain();
    let mut mandates = MandateRegistry::default();
    assert_eq!(mandates.create(domain.engagement(&engagement).unwrap(), source(), content(),
        "client-requestor", "synthetic-client-A", "attempt",
        &authority(&client, Some(&engagement), 1)), Err(MandateError::HumanPrincipalRequired));
    let mut missing = authority(&client, Some(&engagement), 1);
    missing.operations.retain(|operation| operation != "DEFINE_MANDATE");
    assert_eq!(mandates.create(domain.engagement(&engagement).unwrap(), source(), content(),
        HUMAN_PRINCIPAL, "synthetic-client-A", "attempt", &missing),
        Err(MandateError::MissingAuthority));
}

#[test]
fn wrong_mandate_or_version_handoff_is_rejected() {
    let (domain, mandates, _, engagement, combined) = registry();
    let (mandate_id, _) = split_ids(&combined);
    assert_eq!(mandates.decision_problem_handoff(domain.engagement(&engagement).unwrap(),
        "wrong-mandate", 1), Err(MandateError::HandoffBindingMismatch));
    assert_eq!(mandates.decision_problem_handoff(domain.engagement(&engagement).unwrap(),
        mandate_id, 2), Err(MandateError::HandoffBindingMismatch));
}

#[test]
fn deterministic_recreation_matches() {
    let (domain_a, mandates_a, _, engagement_a, combined_a) = registry();
    let (domain_b, mandates_b, _, engagement_b, combined_b) = registry();
    let (mandate_a, _) = split_ids(&combined_a);
    let (mandate_b, _) = split_ids(&combined_b);
    assert_eq!(engagement_a, engagement_b);
    assert_eq!(mandate_a, mandate_b);
    assert_eq!(mandates_a.current(&engagement_a), mandates_b.current(&engagement_b));
    assert_eq!(mandates_a.decision_problem_handoff(domain_a.engagement(&engagement_a).unwrap(),
        mandate_a, 1).unwrap(), mandates_b.decision_problem_handoff(
        domain_b.engagement(&engagement_b).unwrap(), mandate_b, 1).unwrap());
    let handoff = mandates_a.decision_problem_handoff(
        domain_a.engagement(&engagement_a).unwrap(), mandate_a, 1,
    ).unwrap();
    assert_eq!(handoff.authority_context_id(), "synthetic-client-A");
}
