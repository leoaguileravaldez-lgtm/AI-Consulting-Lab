use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use titus_lab_integration_stage_2_engagement_domain::{AuthorityGrant, Engagement, Lifecycle, WorkingLanguage};
use titus_lab_integration_stage_2_mandate_contract::{DecisionProblemHandoff, MandateError, MandateRegistry, HUMAN_PRINCIPAL};

pub const CERTIFIED_PARENT: &str = "011fe1a684b6a35a5f56a5c3c2330865059be34f";

fn identity(domain: &str, parts: &[&str]) -> String {
    let mut hash = Sha256::new();
    hash.update(format!("titus-lab-stage2-decision-problem-{domain}-v1\0"));
    for part in parts { hash.update(part.as_bytes()); hash.update([0]); }
    format!("{:x}", hash.finalize())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionProblemStatus { Current, Superseded, Closed }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionReadiness { Ready, RequiresClarification }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionOrigin { ClientSupplied, MandateSupplied, TitusDerived }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedFact { pub statement: String, pub evidence_ref: String, pub provenance: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientAssertion { pub statement: String, pub source_ref: String, pub provenance: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assumption { pub statement: String, pub provenance: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hypothesis { pub statement: String, pub disconfirmation_criterion: String, pub provenance: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionCriterion { pub name: String, pub rationale: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionOption { pub label: String, pub origin: OptionOrigin, pub provenance: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRequirement { pub question: String, pub category: String, pub provenance: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionFrame {
    pub decision_question: String,
    pub decision_class: String,
    pub bounded_objective: String,
    pub in_scope: Vec<String>,
    pub out_of_scope: Vec<String>,
    pub unresolved: Vec<String>,
    pub constraints: Vec<String>,
    pub observed_facts: Vec<ObservedFact>,
    pub client_assertions: Vec<ClientAssertion>,
    pub assumptions: Vec<Assumption>,
    pub hypotheses: Vec<Hypothesis>,
    pub unknowns: Vec<String>,
    pub dependencies: Vec<String>,
    pub risks: Vec<String>,
    pub decision_variables: Vec<String>,
    pub success_criteria: Vec<String>,
    pub criteria: Vec<DecisionCriterion>,
    pub options: Vec<DecisionOption>,
    pub evidence_requirements: Vec<EvidenceRequirement>,
    pub expected_decision_output: String,
    pub provenance: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionProblemError {
    InvalidEngagement, InvalidAuthority, MissingAuthority, StaleOrRevokedAuthority,
    CrossEngagementSubstitution, HumanPrincipalRequired, MandateBindingMismatch,
    StaleMandate, DecisionProblemAlreadyExists, DecisionProblemNotFound, StaleWrite,
    InvalidTransition, ScopeExpansionRejected, ExclusionErasureRejected,
    AmbiguousDecisionProblem, AssumptionToFactLaundering, HandoffBindingMismatch,
    RequiresClarification,
}

impl From<MandateError> for DecisionProblemError {
    fn from(value: MandateError) -> Self {
        match value {
            MandateError::CrossEngagementSubstitution => Self::CrossEngagementSubstitution,
            MandateError::HandoffBindingMismatch | MandateError::MandateNotFound => Self::MandateBindingMismatch,
            MandateError::StaleWrite | MandateError::InvalidTransition => Self::StaleMandate,
            _ => Self::MandateBindingMismatch,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionProblem {
    decision_problem_id: String,
    client_id: String,
    engagement_id: String,
    mandate_id: String,
    mandate_version: u64,
    mandate_handoff_id: String,
    version: u64,
    predecessor_version: Option<u64>,
    status: DecisionProblemStatus,
    readiness: DecisionReadiness,
    authority_ref: String,
    authority_context_id: String,
    authority_generation: u64,
    working_language: WorkingLanguage,
    frame: DecisionFrame,
    provenance: Vec<String>,
}

impl DecisionProblem {
    pub fn decision_problem_id(&self) -> &str { &self.decision_problem_id }
    pub fn client_id(&self) -> &str { &self.client_id }
    pub fn engagement_id(&self) -> &str { &self.engagement_id }
    pub fn mandate_id(&self) -> &str { &self.mandate_id }
    pub fn mandate_version(&self) -> u64 { self.mandate_version }
    pub fn mandate_handoff_id(&self) -> &str { &self.mandate_handoff_id }
    pub fn version(&self) -> u64 { self.version }
    pub fn predecessor_version(&self) -> Option<u64> { self.predecessor_version }
    pub fn status(&self) -> DecisionProblemStatus { self.status }
    pub fn readiness(&self) -> DecisionReadiness { self.readiness }
    pub fn authority_ref(&self) -> &str { &self.authority_ref }
    pub fn authority_context_id(&self) -> &str { &self.authority_context_id }
    pub fn authority_generation(&self) -> u64 { self.authority_generation }
    pub fn working_language(&self) -> WorkingLanguage { self.working_language }
    pub fn frame(&self) -> &DecisionFrame { &self.frame }
    pub fn provenance(&self) -> &[String] { &self.provenance }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DecisionProblemAggregate { current: DecisionProblem, history: Vec<DecisionProblem> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanningHandoff {
    pub handoff_id: String,
    pub client_id: String,
    pub engagement_id: String,
    pub mandate_id: String,
    pub mandate_version: u64,
    pub decision_problem_id: String,
    pub decision_problem_version: u64,
    pub decision_question: String,
    pub decision_class: String,
    pub scope: Vec<String>,
    pub exclusions: Vec<String>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<Assumption>,
    pub unknowns: Vec<String>,
    pub criteria: Vec<DecisionCriterion>,
    pub evidence_requirements: Vec<EvidenceRequirement>,
    pub authority_ref: String,
    pub authority_context_id: String,
    pub authority_generation: u64,
    pub certified_mandate_predecessor: String,
    pub working_language: WorkingLanguage,
    pub provenance: Vec<String>,
}

#[derive(Debug, Default)]
pub struct DecisionProblemRegistry { problems_by_engagement: BTreeMap<String, DecisionProblemAggregate> }

impl DecisionProblemRegistry {
    pub fn current(&self, engagement_id: &str) -> Option<&DecisionProblem> {
        self.problems_by_engagement.get(engagement_id).map(|aggregate| &aggregate.current)
    }

    pub fn history(&self, engagement_id: &str) -> Option<&[DecisionProblem]> {
        self.problems_by_engagement.get(engagement_id).map(|aggregate| aggregate.history.as_slice())
    }

    pub fn create(
        &mut self, mandates: &MandateRegistry, engagement: &Engagement, mandate_id: &str,
        mandate_version: u64, frame: DecisionFrame, provenance: &str,
        authority: &AuthorityGrant, authorizer_ref: &str,
    ) -> Result<String, DecisionProblemError> {
        validate_engagement(engagement)?;
        validate_authority(engagement, authority, "DEFINE_DECISION_PROBLEM")?;
        validate_human(authorizer_ref)?;
        if self.problems_by_engagement.contains_key(engagement.engagement_id()) {
            return Err(DecisionProblemError::DecisionProblemAlreadyExists);
        }
        let handoff = mandates.decision_problem_handoff(engagement, mandate_id, mandate_version)?;
        validate_handoff_authority(&handoff, authority, authorizer_ref)?;
        validate_frame(&frame, &handoff)?;
        let version_text = mandate_version.to_string();
        let generation = handoff.authority_generation().to_string();
        let problem_id = identity("identity", &[
            engagement.client_id(), engagement.engagement_id(), mandate_id, &version_text,
            handoff.handoff_id(), handoff.authority_context_id(), authorizer_ref, &generation,
        ]);
        let readiness = if handoff.unresolved.is_empty() && frame.unresolved.is_empty() {
            DecisionReadiness::Ready
        } else { DecisionReadiness::RequiresClarification };
        let problem = DecisionProblem {
            decision_problem_id: problem_id.clone(), client_id: engagement.client_id().into(),
            engagement_id: engagement.engagement_id().into(), mandate_id: mandate_id.into(),
            mandate_version, mandate_handoff_id: handoff.handoff_id().into(),
            version: 1, predecessor_version: None, status: DecisionProblemStatus::Current,
            readiness, authority_ref: authorizer_ref.into(),
            authority_context_id: handoff.authority_context_id().into(),
            authority_generation: handoff.authority_generation(), working_language: handoff.working_language,
            frame, provenance: vec![provenance.into()],
        };
        self.problems_by_engagement.insert(engagement.engagement_id().into(),
            DecisionProblemAggregate { current: problem, history: vec![] });
        Ok(problem_id)
    }

    pub fn update(
        &mut self, mandates: &MandateRegistry, engagement: &Engagement,
        problem_id: &str, expected_version: u64, frame: DecisionFrame, provenance: &str,
        authority: &AuthorityGrant, authorizer_ref: &str,
    ) -> Result<(), DecisionProblemError> {
        validate_engagement(engagement)?;
        validate_authority(engagement, authority, "UPDATE_DECISION_PROBLEM")?;
        validate_human(authorizer_ref)?;
        let snapshot = self.current(engagement.engagement_id()).ok_or(DecisionProblemError::DecisionProblemNotFound)?;
        if snapshot.decision_problem_id != problem_id { return Err(DecisionProblemError::HandoffBindingMismatch); }
        if snapshot.status != DecisionProblemStatus::Current { return Err(DecisionProblemError::InvalidTransition); }
        if snapshot.version != expected_version { return Err(DecisionProblemError::StaleWrite); }
        let handoff = current_handoff(mandates, engagement, snapshot)?;
        validate_handoff_authority(&handoff, authority, authorizer_ref)?;
        validate_frame(&frame, &handoff)?;
        let aggregate = self.problems_by_engagement.get_mut(engagement.engagement_id()).unwrap();
        let mut prior = aggregate.current.clone();
        prior.status = DecisionProblemStatus::Superseded;
        aggregate.history.push(prior);
        aggregate.current.predecessor_version = Some(aggregate.current.version);
        aggregate.current.version += 1;
        aggregate.current.frame = frame;
        aggregate.current.readiness = if handoff.unresolved.is_empty() && aggregate.current.frame.unresolved.is_empty() {
            DecisionReadiness::Ready
        } else { DecisionReadiness::RequiresClarification };
        aggregate.current.authority_context_id = handoff.authority_context_id().into();
        aggregate.current.authority_generation = handoff.authority_generation();
        aggregate.current.working_language = handoff.working_language;
        aggregate.current.provenance.push(provenance.into());
        Ok(())
    }

    pub fn close(
        &mut self, mandates: &MandateRegistry, engagement: &Engagement,
        problem_id: &str, expected_version: u64, provenance: &str,
        authority: &AuthorityGrant, authorizer_ref: &str,
    ) -> Result<(), DecisionProblemError> {
        validate_engagement(engagement)?;
        validate_authority(engagement, authority, "CLOSE_DECISION_PROBLEM")?;
        validate_human(authorizer_ref)?;
        let snapshot = self.current(engagement.engagement_id()).ok_or(DecisionProblemError::DecisionProblemNotFound)?;
        if snapshot.decision_problem_id != problem_id { return Err(DecisionProblemError::HandoffBindingMismatch); }
        if snapshot.status != DecisionProblemStatus::Current { return Err(DecisionProblemError::InvalidTransition); }
        if snapshot.version != expected_version { return Err(DecisionProblemError::StaleWrite); }
        let handoff = current_handoff(mandates, engagement, snapshot)?;
        validate_handoff_authority(&handoff, authority, authorizer_ref)?;
        let aggregate = self.problems_by_engagement.get_mut(engagement.engagement_id()).unwrap();
        let mut prior = aggregate.current.clone();
        prior.status = DecisionProblemStatus::Superseded;
        aggregate.history.push(prior);
        aggregate.current.predecessor_version = Some(aggregate.current.version);
        aggregate.current.version += 1;
        aggregate.current.status = DecisionProblemStatus::Closed;
        aggregate.current.provenance.push(provenance.into());
        Ok(())
    }

    pub fn planning_handoff(
        &self, mandates: &MandateRegistry, engagement: &Engagement,
        problem_id: &str, version: u64,
    ) -> Result<PlanningHandoff, DecisionProblemError> {
        let problem = self.current(engagement.engagement_id()).ok_or(DecisionProblemError::DecisionProblemNotFound)?;
        if problem.decision_problem_id != problem_id || problem.version != version || problem.status != DecisionProblemStatus::Current {
            return Err(DecisionProblemError::HandoffBindingMismatch);
        }
        current_handoff(mandates, engagement, problem)?;
        if problem.readiness != DecisionReadiness::Ready { return Err(DecisionProblemError::RequiresClarification); }
        let handoff_id = identity("planning-handoff", &[
            problem.engagement_id(), problem.mandate_id(), &problem.mandate_version.to_string(),
            problem.decision_problem_id(), &problem.version.to_string(), &problem.authority_generation.to_string(),
        ]);
        Ok(PlanningHandoff {
            handoff_id, client_id: problem.client_id.clone(), engagement_id: problem.engagement_id.clone(),
            mandate_id: problem.mandate_id.clone(), mandate_version: problem.mandate_version,
            decision_problem_id: problem.decision_problem_id.clone(), decision_problem_version: problem.version,
            decision_question: problem.frame.decision_question.clone(), decision_class: problem.frame.decision_class.clone(),
            scope: problem.frame.in_scope.clone(), exclusions: problem.frame.out_of_scope.clone(),
            constraints: problem.frame.constraints.clone(), assumptions: problem.frame.assumptions.clone(),
            unknowns: problem.frame.unknowns.clone(), criteria: problem.frame.criteria.clone(),
            evidence_requirements: problem.frame.evidence_requirements.clone(),
            authority_ref: problem.authority_ref.clone(),
            authority_context_id: problem.authority_context_id.clone(),
            authority_generation: problem.authority_generation,
            certified_mandate_predecessor: CERTIFIED_PARENT.into(),
            working_language: problem.working_language, provenance: problem.provenance.clone(),
        })
    }
}

fn validate_engagement(engagement: &Engagement) -> Result<(), DecisionProblemError> {
    if matches!(engagement.lifecycle(), Lifecycle::Closed | Lifecycle::Stopped) {
        return Err(DecisionProblemError::InvalidEngagement);
    }
    Ok(())
}

fn validate_authority(engagement: &Engagement, authority: &AuthorityGrant, operation: &str) -> Result<(), DecisionProblemError> {
    if !authority.current { return Err(DecisionProblemError::StaleOrRevokedAuthority); }
    if authority.client_id != engagement.client_id() { return Err(DecisionProblemError::InvalidAuthority); }
    if authority.engagement_id.as_deref() != Some(engagement.engagement_id()) {
        return Err(DecisionProblemError::CrossEngagementSubstitution);
    }
    if !authority.operations.iter().any(|candidate| candidate == operation) {
        return Err(DecisionProblemError::MissingAuthority);
    }
    Ok(())
}

fn validate_human(authorizer_ref: &str) -> Result<(), DecisionProblemError> {
    if authorizer_ref != HUMAN_PRINCIPAL { return Err(DecisionProblemError::HumanPrincipalRequired); }
    Ok(())
}

fn validate_handoff_authority(handoff: &DecisionProblemHandoff, authority: &AuthorityGrant, authorizer_ref: &str) -> Result<(), DecisionProblemError> {
    if handoff.client_id != authority.client_id || authority.engagement_id.as_deref() != Some(handoff.engagement_id()) {
        return Err(DecisionProblemError::CrossEngagementSubstitution);
    }
    if handoff.authority_ref != authorizer_ref { return Err(DecisionProblemError::HumanPrincipalRequired); }
    if authority.context_id != handoff.authority_context_id() { return Err(DecisionProblemError::InvalidAuthority); }
    if authority.generation != handoff.authority_generation() { return Err(DecisionProblemError::StaleOrRevokedAuthority); }
    Ok(())
}

fn current_handoff(mandates: &MandateRegistry, engagement: &Engagement, problem: &DecisionProblem) -> Result<DecisionProblemHandoff, DecisionProblemError> {
    let handoff = mandates.decision_problem_handoff(engagement, problem.mandate_id(), problem.mandate_version())
        .map_err(|_| DecisionProblemError::StaleMandate)?;
    if handoff.handoff_id() != problem.mandate_handoff_id
        || handoff.engagement_id() != problem.engagement_id
        || handoff.mandate_id() != problem.mandate_id
        || handoff.mandate_version() != problem.mandate_version
        || handoff.authority_context_id() != problem.authority_context_id
        || handoff.authority_generation() != problem.authority_generation
    { return Err(DecisionProblemError::StaleMandate); }
    Ok(handoff)
}

fn nonempty(values: &[String]) -> bool { !values.is_empty() && values.iter().all(|value| !value.trim().is_empty()) }

fn validate_frame(frame: &DecisionFrame, mandate: &DecisionProblemHandoff) -> Result<(), DecisionProblemError> {
    if frame.decision_question.trim().is_empty() || frame.expected_decision_output.trim().is_empty()
        || !nonempty(&frame.success_criteria) || !nonempty(&frame.decision_variables)
        || frame.criteria.is_empty() || frame.evidence_requirements.is_empty()
    { return Err(DecisionProblemError::AmbiguousDecisionProblem); }
    if frame.decision_class != mandate.required_decision || frame.bounded_objective != mandate.authorized_objective {
        return Err(DecisionProblemError::MandateBindingMismatch);
    }
    if frame.in_scope != mandate.in_scope { return Err(DecisionProblemError::ScopeExpansionRejected); }
    if frame.out_of_scope != mandate.out_of_scope { return Err(DecisionProblemError::ExclusionErasureRejected); }
    if frame.constraints != mandate.constraints { return Err(DecisionProblemError::MandateBindingMismatch); }
    if mandate.unresolved.iter().any(|item| !frame.unresolved.contains(item)) {
        return Err(DecisionProblemError::MandateBindingMismatch);
    }
    let typed = frame.observed_facts.iter().all(|fact| !fact.statement.trim().is_empty() && !fact.evidence_ref.trim().is_empty() && !fact.provenance.trim().is_empty())
        && frame.client_assertions.iter().all(|item| !item.statement.trim().is_empty() && !item.source_ref.trim().is_empty() && !item.provenance.trim().is_empty())
        && frame.assumptions.iter().all(|item| !item.statement.trim().is_empty() && !item.provenance.trim().is_empty())
        && frame.hypotheses.iter().all(|item| !item.statement.trim().is_empty() && !item.disconfirmation_criterion.trim().is_empty() && !item.provenance.trim().is_empty())
        && frame.criteria.iter().all(|item| !item.name.trim().is_empty() && !item.rationale.trim().is_empty())
        && frame.options.iter().all(|item| !item.label.trim().is_empty() && !item.provenance.trim().is_empty())
        && frame.evidence_requirements.iter().all(|item| !item.question.trim().is_empty() && !item.category.trim().is_empty() && !item.provenance.trim().is_empty());
    if !typed { return Err(DecisionProblemError::AmbiguousDecisionProblem); }
    if frame.observed_facts.iter().any(|fact| frame.assumptions.iter().any(|item| item.statement == fact.statement)
        || frame.client_assertions.iter().any(|item| item.statement == fact.statement))
    { return Err(DecisionProblemError::AssumptionToFactLaundering); }
    Ok(())
}

pub fn decision_problem_operations() -> Vec<String> {
    ["DEFINE_DECISION_PROBLEM", "UPDATE_DECISION_PROBLEM", "CLOSE_DECISION_PROBLEM"]
        .iter().map(|operation| operation.to_string()).collect()
}
