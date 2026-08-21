use titus_lab_integration_stage_2_decision_problem::*;
use sha2::{Digest, Sha256};
use titus_lab_integration_stage_2_engagement_domain::{
    all_operations, client_identity, AuthorityGrant, EngagementDomain, EngagementType, WorkingLanguage,
};
use titus_lab_integration_stage_2_mandate_contract::{
    mandate_operations, MandateContent, MandateRegistry, Materiality, OriginalMandateInput,
    RiskClassification, HUMAN_PRINCIPAL,
};

fn expected_problem_id(client: &str, engagement: &str, mandate: &str, mandate_version: u64,
    mandate_handoff_id: &str, context: &str, generation: u64) -> String {
    let mut hash = Sha256::new();
    hash.update(b"titus-lab-stage2-decision-problem-identity-v1\0");
    let version = mandate_version.to_string();
    let generation = generation.to_string();
    for part in [client, engagement, mandate, version.as_str(), mandate_handoff_id,
        context, HUMAN_PRINCIPAL, generation.as_str()]
    {
        hash.update(part.as_bytes());
        hash.update([0]);
    }
    format!("{:x}", hash.finalize())
}

fn authority(client: &str, engagement: Option<&str>, generation: u64) -> AuthorityGrant {
    let mut operations = all_operations();
    operations.extend(mandate_operations());
    operations.extend(decision_problem_operations());
    AuthorityGrant { context_id: "synthetic-client-A".into(), client_id: client.into(),
        engagement_id: engagement.map(str::to_owned), operations, generation, current: true }
}

fn mandate_content(unresolved: Vec<String>) -> MandateContent {
    MandateContent {
        what_statement: "Evaluate a bounded operational improvement opportunity".into(),
        why_statement: "Support an evidence-based pilot decision".into(),
        in_scope: vec!["cycle-time assessment".into()],
        out_of_scope: vec!["production deployment".into()], unresolved,
        constraints: vec!["synthetic data only".into()], required_decision: "PILOT_OR_STOP".into(),
        required_outputs: vec!["operational assessment".into()],
        evidence_standard: "ATTRIBUTABLE_CURRENT_SYNTHETIC_EVIDENCE".into(),
        materiality: Materiality::Material, risk: RiskClassification::Moderate,
        progression_approvers: vec![HUMAN_PRINCIPAL.into()],
        completion_criteria: vec!["decision packet ready".into()],
    }
}

fn source() -> OriginalMandateInput { OriginalMandateInput {
    source_ref: "synthetic://client-mandate/v1".into(),
    exact_text: "Evaluate cycle-time evidence and decide whether to pilot.".into(),
    source_language: "EN".into(), provenance: "client-supplied:synthetic".into(),
} }

fn frame(unresolved: Vec<String>) -> DecisionFrame { DecisionFrame {
    decision_question: "Should the bounded cycle-time improvement be piloted?".into(),
    decision_class: "PILOT_OR_STOP".into(),
    bounded_objective: "Evaluate a bounded operational improvement opportunity".into(),
    in_scope: vec!["cycle-time assessment".into()],
    out_of_scope: vec!["production deployment".into()], unresolved,
    constraints: vec!["synthetic data only".into()],
    observed_facts: vec![ObservedFact { statement: "A synthetic baseline record exists".into(),
        evidence_ref: "synthetic://evidence/baseline".into(), provenance: "synthetic:evidence-index".into() }],
    client_assertions: vec![ClientAssertion { statement: "Cycle time is material".into(),
        source_ref: "synthetic://client-mandate/v1".into(), provenance: "client-supplied:synthetic".into() }],
    assumptions: vec![Assumption { statement: "Pilot capacity will be available".into(),
        provenance: "titus-derived:explicit-assumption".into() }],
    hypotheses: vec![Hypothesis { statement: "A bounded pilot could reduce cycle time".into(),
        disconfirmation_criterion: "No material improvement under the defined test".into(),
        provenance: "titus-derived:hypothesis".into() }],
    unknowns: vec!["achievable reduction".into()], dependencies: vec!["bounded evidence collection".into()],
    risks: vec!["synthetic evidence may not generalize".into()],
    decision_variables: vec!["cycle-time reduction".into()],
    success_criteria: vec!["decision can be resolved with attributable evidence".into()],
    criteria: vec![DecisionCriterion { name: "operational feasibility".into(),
        rationale: "required by the bounded pilot decision".into() }],
    options: vec![DecisionOption { label: "pilot".into(), origin: OptionOrigin::MandateSupplied,
        provenance: "client-mandate:required-decision".into() },
        DecisionOption { label: "stop".into(), origin: OptionOrigin::MandateSupplied,
        provenance: "client-mandate:required-decision".into() }],
    evidence_requirements: vec![EvidenceRequirement { question: "What is the attributable baseline cycle time?".into(),
        category: "operational".into(), provenance: "titus-derived:requirement".into() }],
    expected_decision_output: "PILOT_OR_STOP decision frame".into(),
    provenance: vec!["titus-derived:decision-frame".into()],
} }

struct Fixture { domain: EngagementDomain, mandates: MandateRegistry, client: String,
    engagement: String, other: String, mandate: String }

fn fixture(unresolved: Vec<String>) -> Fixture {
    let mut domain = EngagementDomain::default();
    let client = client_identity("synthetic-client-A", "Synthetic Industrial Client", "synthetic:client-source");
    domain.create_client("synthetic-client-A", "Synthetic Industrial Client", "ORGANIZATION",
        "synthetic:client-source", &authority(&client, None, 1)).unwrap();
    let engagement = domain.create_engagement(&client, "Operational Improvement Assessment",
        EngagementType::Operations, WorkingLanguage::Es, "synthetic-human-principal",
        "synthetic:engagement-a", &authority(&client, None, 1)).unwrap();
    let other = domain.create_engagement(&client, "Separate Assessment", EngagementType::Research,
        WorkingLanguage::En, "synthetic-human-principal", "synthetic:engagement-b",
        &authority(&client, None, 1)).unwrap();
    let mut mandates = MandateRegistry::default();
    let mandate = mandates.create(domain.engagement(&engagement).unwrap(), source(),
        mandate_content(unresolved), HUMAN_PRINCIPAL, "synthetic-client-A",
        "titus-derived:structured-mandate", &authority(&client, Some(&engagement), 1)).unwrap();
    Fixture { domain, mandates, client, engagement, other, mandate }
}

fn create(f: &Fixture, registry: &mut DecisionProblemRegistry, unresolved: Vec<String>) -> String {
    registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
        frame(unresolved), "titus-derived:decision-problem", &authority(&f.client, Some(&f.engagement), 1),
        HUMAN_PRINCIPAL).unwrap()
}

#[test]
fn valid_current_mandate_creates_bound_problem_and_planning_handoff() {
    let f = fixture(vec![]); let mut registry = DecisionProblemRegistry::default();
    let id = create(&f, &mut registry, vec![]); let problem = registry.current(&f.engagement).unwrap();
    let mandate_handoff = f.mandates.decision_problem_handoff(
        f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
    ).unwrap();
    assert_eq!(problem.engagement_id(), f.engagement); assert_eq!(problem.mandate_id(), f.mandate);
    assert_eq!(problem.mandate_version(), 1); assert_eq!(problem.working_language(), WorkingLanguage::Es);
    assert_eq!(problem.readiness(), DecisionReadiness::Ready);
    assert_eq!(problem.authority_context_id(), mandate_handoff.authority_context_id());
    assert_eq!(problem.authority_generation(), mandate_handoff.authority_generation());
    assert_eq!(id, expected_problem_id(&f.client, &f.engagement, &f.mandate, 1,
        mandate_handoff.handoff_id(), mandate_handoff.authority_context_id(),
        mandate_handoff.authority_generation()));
    assert_eq!(CERTIFIED_PARENT, "011fe1a684b6a35a5f56a5c3c2330865059be34f");
    assert_ne!(CERTIFIED_PARENT, "9d21594a2a769f1807f534b3c54be2cb2c3c8bec");
    let handoff = registry.planning_handoff(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 1).unwrap();
    assert_eq!(handoff.decision_problem_id, id); assert_eq!(handoff.scope, vec!["cycle-time assessment"]);
    assert_eq!(handoff.evidence_requirements.len(), 1);
    assert_eq!(handoff.authority_context_id, "synthetic-client-A");
    assert_eq!(handoff.authority_generation, 1);
    assert_eq!(handoff.certified_mandate_predecessor, CERTIFIED_PARENT);
}

#[test]
fn wrong_engagement_and_wrong_mandate_are_rejected() {
    let f = fixture(vec![]); let mut registry = DecisionProblemRegistry::default();
    assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.other).unwrap(), &f.mandate, 1,
        frame(vec![]), "foreign", &authority(&f.client, Some(&f.other), 1), HUMAN_PRINCIPAL),
        Err(DecisionProblemError::CrossEngagementSubstitution));
    assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), "wrong", 1,
        frame(vec![]), "wrong", &authority(&f.client, Some(&f.engagement), 1), HUMAN_PRINCIPAL),
        Err(DecisionProblemError::MandateBindingMismatch));
}

#[test]
fn stale_mandate_version_invalidates_decision_problem() {
    let mut f = fixture(vec![]); let mut registry = DecisionProblemRegistry::default();
    let id = create(&f, &mut registry, vec![]);
    f.mandates.amend(f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
        mandate_content(vec![]), None, HUMAN_PRINCIPAL, "authorized:v2",
        &authority(&f.client, Some(&f.engagement), 1)).unwrap();
    assert_eq!(registry.planning_handoff(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 1),
        Err(DecisionProblemError::StaleMandate));
}

#[test]
fn scope_expansion_and_exclusion_erasure_are_blocked() {
    let f = fixture(vec![]); let mut registry = DecisionProblemRegistry::default();
    let mut expanded = frame(vec![]); expanded.in_scope.push("production deployment".into());
    assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
        expanded, "expanded", &authority(&f.client, Some(&f.engagement), 1), HUMAN_PRINCIPAL),
        Err(DecisionProblemError::ScopeExpansionRejected));
    let mut erased = frame(vec![]); erased.out_of_scope.clear();
    assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
        erased, "erased", &authority(&f.client, Some(&f.engagement), 1), HUMAN_PRINCIPAL),
        Err(DecisionProblemError::ExclusionErasureRejected));
}

#[test]
fn stale_problem_update_is_rejected_and_history_is_preserved() {
    let f = fixture(vec![]); let mut registry = DecisionProblemRegistry::default(); let id = create(&f, &mut registry, vec![]);
    let grant = authority(&f.client, Some(&f.engagement), 1);
    registry.update(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 1,
        frame(vec![]), "authorized:v2", &grant, HUMAN_PRINCIPAL).unwrap();
    assert_eq!(registry.update(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 1,
        frame(vec![]), "stale", &grant, HUMAN_PRINCIPAL), Err(DecisionProblemError::StaleWrite));
    assert_eq!(registry.current(&f.engagement).unwrap().predecessor_version(), Some(1));
    assert_eq!(registry.history(&f.engagement).unwrap()[0].status(), DecisionProblemStatus::Superseded);
}

#[test]
fn assumption_to_fact_laundering_is_blocked() {
    let f = fixture(vec![]); let mut registry = DecisionProblemRegistry::default(); let mut bad = frame(vec![]);
    bad.observed_facts[0].statement = bad.assumptions[0].statement.clone();
    assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
        bad, "launder", &authority(&f.client, Some(&f.engagement), 1), HUMAN_PRINCIPAL),
        Err(DecisionProblemError::AssumptionToFactLaundering));
}

#[test]
fn human_principal_and_bounded_authority_are_required() {
    let f = fixture(vec![]); let mut registry = DecisionProblemRegistry::default();
    assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
        frame(vec![]), "attempt", &authority(&f.client, Some(&f.engagement), 1), "client-requestor"),
        Err(DecisionProblemError::HumanPrincipalRequired));
    let mut missing = authority(&f.client, Some(&f.engagement), 1);
    missing.operations.retain(|op| op != "DEFINE_DECISION_PROBLEM");
    assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
        frame(vec![]), "attempt", &missing, HUMAN_PRINCIPAL), Err(DecisionProblemError::MissingAuthority));
    let mut wrong_context = authority(&f.client, Some(&f.engagement), 1);
    wrong_context.context_id = "foreign-context".into();
    assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
        frame(vec![]), "attempt", &wrong_context, HUMAN_PRINCIPAL), Err(DecisionProblemError::InvalidAuthority));
    for generation in [0, 2] {
        assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
            frame(vec![]), "attempt", &authority(&f.client, Some(&f.engagement), generation), HUMAN_PRINCIPAL),
            Err(DecisionProblemError::StaleOrRevokedAuthority));
    }
    let mut revoked = authority(&f.client, Some(&f.engagement), 1); revoked.current = false;
    assert_eq!(registry.create(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &f.mandate, 1,
        frame(vec![]), "attempt", &revoked, HUMAN_PRINCIPAL), Err(DecisionProblemError::StaleOrRevokedAuthority));

    let id = create(&f, &mut registry, vec![]);
    assert_eq!(registry.update(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 1,
        frame(vec![]), "wrong-context", &wrong_context, HUMAN_PRINCIPAL), Err(DecisionProblemError::InvalidAuthority));
    assert_eq!(registry.update(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 1,
        frame(vec![]), "wrong-generation", &authority(&f.client, Some(&f.engagement), 2), HUMAN_PRINCIPAL),
        Err(DecisionProblemError::StaleOrRevokedAuthority));
}

#[test]
fn material_ambiguity_requires_clarification_and_blocks_handoff() {
    let unresolved = vec!["site expansion".into()]; let f = fixture(unresolved.clone());
    let mut registry = DecisionProblemRegistry::default(); let id = create(&f, &mut registry, unresolved);
    assert_eq!(registry.current(&f.engagement).unwrap().readiness(), DecisionReadiness::RequiresClarification);
    assert_eq!(registry.planning_handoff(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 1),
        Err(DecisionProblemError::RequiresClarification));
}

#[test]
fn lifecycle_and_wrong_handoff_version_are_rejected() {
    let f = fixture(vec![]); let mut registry = DecisionProblemRegistry::default(); let id = create(&f, &mut registry, vec![]);
    assert_eq!(registry.planning_handoff(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 2),
        Err(DecisionProblemError::HandoffBindingMismatch));
    let grant = authority(&f.client, Some(&f.engagement), 1);
    registry.close(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 1,
        "close", &grant, HUMAN_PRINCIPAL).unwrap();
    assert_eq!(registry.close(&f.mandates, f.domain.engagement(&f.engagement).unwrap(), &id, 2,
        "close-again", &grant, HUMAN_PRINCIPAL), Err(DecisionProblemError::InvalidTransition));
}

#[test]
fn deterministic_recreation_matches_identity_frame_and_handoff() {
    let a = fixture(vec![]); let b = fixture(vec![]);
    let mut ra = DecisionProblemRegistry::default(); let mut rb = DecisionProblemRegistry::default();
    let ida = create(&a, &mut ra, vec![]); let idb = create(&b, &mut rb, vec![]);
    assert_eq!(ida, idb); assert_eq!(ra.current(&a.engagement), rb.current(&b.engagement));
    assert_eq!(ra.planning_handoff(&a.mandates, a.domain.engagement(&a.engagement).unwrap(), &ida, 1).unwrap(),
        rb.planning_handoff(&b.mandates, b.domain.engagement(&b.engagement).unwrap(), &idb, 1).unwrap());
}
