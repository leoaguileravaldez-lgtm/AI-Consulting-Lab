use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use titus_lab_integration_stage_2_engagement_domain::{
    AuthorityGrant, Engagement, Lifecycle, WorkingLanguage,
};

pub const CERTIFIED_PARENT: &str = "6fc3b3fac9b079be216dbff6e7bdc8a26a0b9f66";
pub const HUMAN_PRINCIPAL: &str = "human-principal:titus-lab";

fn identity(domain: &str, parts: &[&str]) -> String {
    let mut hash = Sha256::new();
    hash.update(format!("titus-lab-stage2-mandate-{domain}-v1\0"));
    for part in parts {
        hash.update(part.as_bytes());
        hash.update([0]);
    }
    format!("{:x}", hash.finalize())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Materiality { Low, Moderate, Material, Critical }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskClassification { Low, Moderate, Material, Critical }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MandateStatus { Current, Superseded, Closed }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeDisposition { InScope, OutOfScope, RequiresClarification }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MandateError {
    InvalidEngagement,
    InvalidAuthority,
    MissingAuthority,
    StaleOrRevokedAuthority,
    CrossEngagementSubstitution,
    HumanPrincipalRequired,
    AmbiguousMandate,
    MandateAlreadyExists,
    MandateNotFound,
    StaleWrite,
    InvalidTransition,
    ScopeExpansionRejected,
    HandoffBindingMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalMandateInput {
    pub source_ref: String,
    pub exact_text: String,
    pub source_language: String,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MandateContent {
    pub what_statement: String,
    pub why_statement: String,
    pub in_scope: Vec<String>,
    pub out_of_scope: Vec<String>,
    pub unresolved: Vec<String>,
    pub constraints: Vec<String>,
    pub required_decision: String,
    pub required_outputs: Vec<String>,
    pub evidence_standard: String,
    pub materiality: Materiality,
    pub risk: RiskClassification,
    pub progression_approvers: Vec<String>,
    pub completion_criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MandateContract {
    mandate_id: String,
    client_id: String,
    engagement_id: String,
    version: u64,
    predecessor_version: Option<u64>,
    status: MandateStatus,
    authorizer_ref: String,
    authority_generation: u64,
    creation_context: String,
    working_language: WorkingLanguage,
    original_inputs: Vec<OriginalMandateInput>,
    content: MandateContent,
    provenance: Vec<String>,
}

impl MandateContract {
    pub fn mandate_id(&self) -> &str { &self.mandate_id }
    pub fn client_id(&self) -> &str { &self.client_id }
    pub fn engagement_id(&self) -> &str { &self.engagement_id }
    pub fn version(&self) -> u64 { self.version }
    pub fn predecessor_version(&self) -> Option<u64> { self.predecessor_version }
    pub fn status(&self) -> MandateStatus { self.status }
    pub fn authorizer_ref(&self) -> &str { &self.authorizer_ref }
    pub fn authority_generation(&self) -> u64 { self.authority_generation }
    pub fn working_language(&self) -> WorkingLanguage { self.working_language }
    pub fn original_inputs(&self) -> &[OriginalMandateInput] { &self.original_inputs }
    pub fn content(&self) -> &MandateContent { &self.content }
    pub fn provenance(&self) -> &[String] { &self.provenance }

    pub fn scope_disposition(&self, request: &str) -> ScopeDisposition {
        if self.content.in_scope.iter().any(|item| item == request) {
            ScopeDisposition::InScope
        } else if self.content.out_of_scope.iter().any(|item| item == request) {
            ScopeDisposition::OutOfScope
        } else {
            ScopeDisposition::RequiresClarification
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MandateAggregate {
    current: MandateContract,
    history: Vec<MandateContract>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionProblemHandoff {
    handoff_id: String,
    pub client_id: String,
    engagement_id: String,
    mandate_id: String,
    mandate_version: u64,
    pub authorized_objective: String,
    pub in_scope: Vec<String>,
    pub out_of_scope: Vec<String>,
    pub unresolved: Vec<String>,
    pub constraints: Vec<String>,
    pub required_decision: String,
    pub authority_ref: String,
    authority_context_id: String,
    authority_generation: u64,
    pub working_language: WorkingLanguage,
    pub source_refs: Vec<String>,
    pub provenance: Vec<String>,
}

impl DecisionProblemHandoff {
    pub fn handoff_id(&self) -> &str { &self.handoff_id }
    pub fn engagement_id(&self) -> &str { &self.engagement_id }
    pub fn mandate_id(&self) -> &str { &self.mandate_id }
    pub fn mandate_version(&self) -> u64 { self.mandate_version }
    pub fn authority_context_id(&self) -> &str { &self.authority_context_id }
    pub fn authority_generation(&self) -> u64 { self.authority_generation }
}

#[derive(Debug, Default)]
pub struct MandateRegistry {
    mandates_by_engagement: BTreeMap<String, MandateAggregate>,
}

impl MandateRegistry {
    pub fn current(&self, engagement_id: &str) -> Option<&MandateContract> {
        self.mandates_by_engagement.get(engagement_id).map(|entry| &entry.current)
    }

    pub fn history(&self, engagement_id: &str) -> Option<&[MandateContract]> {
        self.mandates_by_engagement.get(engagement_id).map(|entry| entry.history.as_slice())
    }

    fn mandate_owner(&self, mandate_id: &str) -> Option<&str> {
        self.mandates_by_engagement.values()
            .find(|entry| entry.current.mandate_id == mandate_id)
            .map(|entry| entry.current.engagement_id.as_str())
    }

    pub fn create(
        &mut self,
        engagement: &Engagement,
        source: OriginalMandateInput,
        content: MandateContent,
        authorizer_ref: &str,
        creation_context: &str,
        provenance: &str,
        authority: &AuthorityGrant,
    ) -> Result<String, MandateError> {
        validate_engagement(engagement)?;
        validate_authority(engagement, authority, "DEFINE_MANDATE")?;
        if authority.context_id != creation_context { return Err(MandateError::InvalidAuthority); }
        validate_human(authorizer_ref)?;
        validate_source(&source)?;
        validate_content(&content)?;
        if self.mandates_by_engagement.contains_key(engagement.engagement_id()) {
            return Err(MandateError::MandateAlreadyExists);
        }
        let generation = authority.generation.to_string();
        let mandate_id = identity("identity", &[
            engagement.client_id(), engagement.engagement_id(), &source.source_ref,
            creation_context, authorizer_ref, &generation,
        ]);
        let contract = MandateContract {
            mandate_id: mandate_id.clone(),
            client_id: engagement.client_id().into(),
            engagement_id: engagement.engagement_id().into(),
            version: 1,
            predecessor_version: None,
            status: MandateStatus::Current,
            authorizer_ref: authorizer_ref.into(),
            authority_generation: authority.generation,
            creation_context: creation_context.into(),
            working_language: engagement.working_language(),
            original_inputs: vec![source],
            content,
            provenance: vec![provenance.into()],
        };
        self.mandates_by_engagement.insert(
            engagement.engagement_id().into(),
            MandateAggregate { current: contract, history: vec![] },
        );
        Ok(mandate_id)
    }

    pub fn amend(
        &mut self,
        engagement: &Engagement,
        mandate_id: &str,
        expected_version: u64,
        content: MandateContent,
        additional_source: Option<OriginalMandateInput>,
        authorizer_ref: &str,
        provenance: &str,
        authority: &AuthorityGrant,
    ) -> Result<(), MandateError> {
        validate_engagement(engagement)?;
        validate_authority(engagement, authority, "AMEND_MANDATE")?;
        validate_human(authorizer_ref)?;
        validate_content(&content)?;
        if let Some(source) = additional_source.as_ref() { validate_source(source)?; }
        if self.mandate_owner(mandate_id).is_some_and(|owner| owner != engagement.engagement_id()) {
            return Err(MandateError::CrossEngagementSubstitution);
        }
        let aggregate = self.mandates_by_engagement.get_mut(engagement.engagement_id())
            .ok_or(MandateError::MandateNotFound)?;
        validate_current_binding(&aggregate.current, engagement, mandate_id, expected_version, authority)?;
        let mut prior = aggregate.current.clone();
        prior.status = MandateStatus::Superseded;
        aggregate.history.push(prior);
        let current = &mut aggregate.current;
        current.predecessor_version = Some(current.version);
        current.version += 1;
        current.content = content;
        if let Some(source) = additional_source { current.original_inputs.push(source); }
        current.authorizer_ref = authorizer_ref.into();
        current.authority_generation = authority.generation;
        current.working_language = engagement.working_language();
        current.provenance.push(provenance.into());
        Ok(())
    }

    pub fn refresh_working_language(
        &mut self,
        engagement: &Engagement,
        mandate_id: &str,
        expected_version: u64,
        authorizer_ref: &str,
        provenance: &str,
        authority: &AuthorityGrant,
    ) -> Result<(), MandateError> {
        let content = self.current(engagement.engagement_id())
            .ok_or(MandateError::MandateNotFound)?.content.clone();
        self.amend(engagement, mandate_id, expected_version, content, None,
            authorizer_ref, provenance, authority)
    }

    pub fn authorize_request(
        &self,
        engagement_id: &str,
        mandate_id: &str,
        version: u64,
        request: &str,
    ) -> Result<(), MandateError> {
        let mandate = self.current(engagement_id).ok_or(MandateError::MandateNotFound)?;
        if mandate.mandate_id != mandate_id || mandate.version != version || mandate.status != MandateStatus::Current {
            return Err(MandateError::StaleWrite);
        }
        match mandate.scope_disposition(request) {
            ScopeDisposition::InScope => Ok(()),
            ScopeDisposition::OutOfScope | ScopeDisposition::RequiresClarification => {
                Err(MandateError::ScopeExpansionRejected)
            }
        }
    }

    pub fn close(
        &mut self,
        engagement: &Engagement,
        mandate_id: &str,
        expected_version: u64,
        authorizer_ref: &str,
        provenance: &str,
        authority: &AuthorityGrant,
    ) -> Result<(), MandateError> {
        validate_engagement(engagement)?;
        validate_authority(engagement, authority, "CLOSE_MANDATE")?;
        validate_human(authorizer_ref)?;
        if self.mandate_owner(mandate_id).is_some_and(|owner| owner != engagement.engagement_id()) {
            return Err(MandateError::CrossEngagementSubstitution);
        }
        let aggregate = self.mandates_by_engagement.get_mut(engagement.engagement_id())
            .ok_or(MandateError::MandateNotFound)?;
        validate_current_binding(&aggregate.current, engagement, mandate_id, expected_version, authority)?;
        let mut prior = aggregate.current.clone();
        prior.status = MandateStatus::Superseded;
        aggregate.history.push(prior);
        aggregate.current.predecessor_version = Some(aggregate.current.version);
        aggregate.current.version += 1;
        aggregate.current.status = MandateStatus::Closed;
        aggregate.current.provenance.push(provenance.into());
        Ok(())
    }

    pub fn decision_problem_handoff(
        &self,
        engagement: &Engagement,
        mandate_id: &str,
        version: u64,
    ) -> Result<DecisionProblemHandoff, MandateError> {
        if self.mandate_owner(mandate_id).is_some_and(|owner| owner != engagement.engagement_id()) {
            return Err(MandateError::CrossEngagementSubstitution);
        }
        let mandate = self.current(engagement.engagement_id()).ok_or(MandateError::MandateNotFound)?;
        if mandate.engagement_id != engagement.engagement_id()
            || mandate.client_id != engagement.client_id()
            || mandate.mandate_id != mandate_id
            || mandate.version != version
            || mandate.status != MandateStatus::Current
        {
            return Err(MandateError::HandoffBindingMismatch);
        }
        let version_text = version.to_string();
        let handoff_id = identity("decision-problem-handoff", &[
            mandate.engagement_id(), mandate.mandate_id(), &version_text,
            &mandate.creation_context,
            &mandate.authority_generation.to_string(),
        ]);
        Ok(DecisionProblemHandoff {
            handoff_id,
            client_id: mandate.client_id.clone(),
            engagement_id: mandate.engagement_id.clone(),
            mandate_id: mandate.mandate_id.clone(),
            mandate_version: mandate.version,
            authorized_objective: mandate.content.what_statement.clone(),
            in_scope: mandate.content.in_scope.clone(),
            out_of_scope: mandate.content.out_of_scope.clone(),
            unresolved: mandate.content.unresolved.clone(),
            constraints: mandate.content.constraints.clone(),
            required_decision: mandate.content.required_decision.clone(),
            authority_ref: mandate.authorizer_ref.clone(),
            authority_context_id: mandate.creation_context.clone(),
            authority_generation: mandate.authority_generation,
            working_language: mandate.working_language,
            source_refs: mandate.original_inputs.iter().map(|source| source.source_ref.clone()).collect(),
            provenance: mandate.provenance.clone(),
        })
    }
}

fn validate_engagement(engagement: &Engagement) -> Result<(), MandateError> {
    if matches!(engagement.lifecycle(), Lifecycle::Closed | Lifecycle::Stopped) {
        return Err(MandateError::InvalidEngagement);
    }
    Ok(())
}

fn validate_authority(
    engagement: &Engagement,
    authority: &AuthorityGrant,
    operation: &str,
) -> Result<(), MandateError> {
    if !authority.current { return Err(MandateError::StaleOrRevokedAuthority); }
    if authority.client_id != engagement.client_id() {
        return Err(MandateError::InvalidAuthority);
    }
    if authority.engagement_id.as_deref() != Some(engagement.engagement_id()) {
        return Err(MandateError::CrossEngagementSubstitution);
    }
    if !authority.operations.iter().any(|candidate| candidate == operation) {
        return Err(MandateError::MissingAuthority);
    }
    Ok(())
}

fn validate_human(authorizer_ref: &str) -> Result<(), MandateError> {
    if authorizer_ref != HUMAN_PRINCIPAL {
        return Err(MandateError::HumanPrincipalRequired);
    }
    Ok(())
}

fn validate_source(source: &OriginalMandateInput) -> Result<(), MandateError> {
    if source.source_ref.is_empty() || source.exact_text.is_empty()
        || source.source_language.is_empty() || source.provenance.is_empty()
    {
        return Err(MandateError::AmbiguousMandate);
    }
    Ok(())
}

fn validate_content(content: &MandateContent) -> Result<(), MandateError> {
    let required_lists = [
        content.in_scope.as_slice(), content.out_of_scope.as_slice(),
        content.constraints.as_slice(), content.required_outputs.as_slice(),
        content.progression_approvers.as_slice(), content.completion_criteria.as_slice(),
    ];
    if content.what_statement.is_empty() || content.why_statement.is_empty()
        || content.required_decision.is_empty() || content.evidence_standard.is_empty()
        || required_lists.iter().any(|list| list.is_empty() || list.iter().any(|item| item.is_empty()))
        || content.unresolved.iter().any(|item| item.is_empty())
    {
        return Err(MandateError::AmbiguousMandate);
    }
    let overlaps = content.in_scope.iter().any(|item|
        content.out_of_scope.contains(item) || content.unresolved.contains(item))
        || content.out_of_scope.iter().any(|item| content.unresolved.contains(item));
    if overlaps { return Err(MandateError::AmbiguousMandate); }
    Ok(())
}

fn validate_current_binding(
    current: &MandateContract,
    engagement: &Engagement,
    mandate_id: &str,
    expected_version: u64,
    authority: &AuthorityGrant,
) -> Result<(), MandateError> {
    if current.engagement_id != engagement.engagement_id()
        || current.client_id != engagement.client_id()
        || current.mandate_id != mandate_id
    {
        return Err(MandateError::CrossEngagementSubstitution);
    }
    if current.status != MandateStatus::Current { return Err(MandateError::InvalidTransition); }
    if current.version != expected_version { return Err(MandateError::StaleWrite); }
    if current.creation_context != authority.context_id {
        return Err(MandateError::InvalidAuthority);
    }
    if authority.generation < current.authority_generation {
        return Err(MandateError::StaleOrRevokedAuthority);
    }
    Ok(())
}

pub fn mandate_operations() -> Vec<String> {
    ["DEFINE_MANDATE", "AMEND_MANDATE", "CLOSE_MANDATE"]
        .iter().map(|operation| operation.to_string()).collect()
}
