use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use titus_lab_integration_stage_1_kernel::{
    HumanGateState, IntegrationClassification, KernelResult, STAGE_I_CANDIDATE_VERSION,
};

pub const STAGE_I_PARENT: &str = "1f66ea04107193d80eed0671c137ead2e7e32a5b";

fn id(domain: &str, parts: &[&str]) -> String {
    let mut h = Sha256::new();
    h.update(format!("titus-lab-stage2-{domain}-v1\0"));
    for part in parts { h.update(part.as_bytes()); h.update([0]); }
    format!("{:x}", h.finalize())
}
pub fn client_identity(context:&str,name:&str,provenance:&str)->String{id("client",&[context,name,provenance])}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkingLanguage { Es, En }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngagementType { Strategy, DueDiligence, Engineering, SoftwareBuild, BusinessPlan, FinancialAnalysis, Operations, VentureBuild, Research, Other }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimClass { Fact, Inference, Assumption, Estimate, Projection, Recommendation }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lifecycle { Draft, Defined, Active, AwaitingHumanDecision, Stopped, Closed }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionType { Invest, Build, Pilot, Proceed, Modify, Conditional, DoNotProceed }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainError { InvalidContext, MissingAuthority, ScopeRejected, InvalidClient, InvalidEngagement, StaleOrRevokedAuthority, StaleWrite, CrossEngagementMutation, InvalidTransition, ClosedEngagement, EvidenceInvariant, ValidationRequired, HumanPrincipalBypass, InfrastructureFailure, HarnessFailure }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityGrant { pub context_id:String, pub client_id:String, pub engagement_id:Option<String>, pub operations:Vec<String>, pub generation:u64, pub current:bool }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client { pub client_id:String, pub canonical_name:String, pub client_type:String, pub active:bool, pub version:u64, pub provenance:String }
pub trait EngagementOwned { fn engagement_id(&self)->&str; }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Objective { engagement_id:String, pub objective_id:String, pub client_statement:String, pub interpreted_objective:String, pub success_criteria:String, pub constraints:Vec<String>, pub exclusions:Vec<String>, pub unresolved_assumptions:Vec<String>, pub provenance:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceReference { engagement_id:String, pub source_id:String, pub original_language:String, pub original_content_ref:String, pub fingerprint:String, pub provenance:String, pub access_scope:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceReference { engagement_id:String, pub evidence_id:String, pub source_id:String, pub classification:ClaimClass, pub validation_status:String, pub provenance:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requirement { engagement_id:String, pub requirement_id:String, pub description:String, pub category:String, pub source_ref:String, pub status:String, pub version:u64, pub predecessor:Option<String>, pub provenance:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workstream { engagement_id:String, pub workstream_id:String, pub kind:String, pub provenance:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision { engagement_id:String, pub decision_id:String, pub decision_type:DecisionType, pub recommendation_ref:String, pub human_decision:Option<bool>, pub provenance:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deliverable { engagement_id:String, pub deliverable_id:String, pub title:String, pub kind:String, pub version:u64, pub output_language:WorkingLanguage, pub source_refs:Vec<String>, pub status:String, pub provenance:String, pub audit_ref:Option<String>, pub validation_ref:Option<String> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildReference { engagement_id:String, pub build_id:String, pub artifact_ref:String, pub test_ref:Option<String>, pub provenance:String }
macro_rules! engagement_owned {($($t:ty),+)=>{$(impl EngagementOwned for $t{fn engagement_id(&self)->&str{&self.engagement_id}})+};}
engagement_owned!(Objective,SourceReference,EvidenceReference,Requirement,Workstream,Decision,Deliverable,BuildReference);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedLocalizedArtifact { pub derivation_id:String, pub source_id:String, pub original_language:String, pub target_language:WorkingLanguage, pub provenance:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageIReference { pub candidate_version:String, pub result_id:String, pub package_fingerprint:String, pub validator_verdict:String, pub human_gate:HumanGateState }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Engagement {
    engagement_id:String, client_id:String, title:String, engagement_type:EngagementType,
    lifecycle:Lifecycle, version:u64, predecessor_version:Option<u64>, created_by:String,
    working_language:WorkingLanguage, provenance:Vec<String>, objective:Option<Objective>,
    sources:Vec<SourceReference>, evidence:Vec<EvidenceReference>, requirements:Vec<Requirement>,
    workstreams:Vec<Workstream>, decisions:Vec<Decision>, deliverables:Vec<Deliverable>,
    builds:Vec<BuildReference>, audit_references:Vec<String>, validation_references:Vec<String>,
    stage1_gate:Option<StageIReference>,
}

#[derive(Debug, Default)]
pub struct EngagementDomain { clients:BTreeMap<String,Client>, engagements:BTreeMap<String,Engagement> }

impl Engagement {
    pub fn engagement_id(&self)->&str{&self.engagement_id}
    pub fn client_id(&self)->&str{&self.client_id}
    pub fn title(&self)->&str{&self.title}
    pub fn engagement_type(&self)->EngagementType{self.engagement_type}
    pub fn lifecycle(&self)->Lifecycle{self.lifecycle}
    pub fn version(&self)->u64{self.version}
    pub fn predecessor_version(&self)->Option<u64>{self.predecessor_version}
    pub fn created_by(&self)->&str{&self.created_by}
    pub fn working_language(&self)->WorkingLanguage{self.working_language}
    pub fn provenance(&self)->&[String]{&self.provenance}
    pub fn objective(&self)->Option<&Objective>{self.objective.as_ref()}
    pub fn sources(&self)->&[SourceReference]{&self.sources}
    pub fn evidence(&self)->&[EvidenceReference]{&self.evidence}
    pub fn requirements(&self)->&[Requirement]{&self.requirements}
    pub fn workstreams(&self)->&[Workstream]{&self.workstreams}
    pub fn decisions(&self)->&[Decision]{&self.decisions}
    pub fn deliverables(&self)->&[Deliverable]{&self.deliverables}
    pub fn builds(&self)->&[BuildReference]{&self.builds}
    pub fn audit_references(&self)->&[String]{&self.audit_references}
    pub fn validation_references(&self)->&[String]{&self.validation_references}
    pub fn stage1_gate(&self)->Option<&StageIReference>{self.stage1_gate.as_ref()}
}

impl EngagementDomain {
    pub fn client(&self,client_id:&str)->Option<&Client>{self.clients.get(client_id)}
    pub fn engagement(&self,engagement_id:&str)->Option<&Engagement>{self.engagements.get(engagement_id)}
    pub fn create_client(&mut self, context:&str, name:&str, client_type:&str, provenance:&str, grant:&AuthorityGrant) -> Result<String,DomainError> {
        if name.is_empty() || context != grant.context_id { return Err(DomainError::InvalidContext); }
        if !grant.current { return Err(DomainError::StaleOrRevokedAuthority); }
        if !grant.operations.iter().any(|x|x=="CREATE_ENGAGEMENT") { return Err(DomainError::MissingAuthority); }
        let client_id=client_identity(context,name,provenance);
        if grant.client_id != client_id { return Err(DomainError::InvalidClient); }
        self.clients.entry(client_id.clone()).or_insert(Client{client_id:client_id.clone(),canonical_name:name.into(),client_type:client_type.into(),active:true,version:1,provenance:provenance.into()});
        Ok(client_id)
    }
    pub fn create_engagement(&mut self, client_id:&str, title:&str, kind:EngagementType, language:WorkingLanguage, actor:&str, provenance:&str, grant:&AuthorityGrant) -> Result<String,DomainError> {
        self.authorize(grant,client_id,None,"CREATE_ENGAGEMENT")?;
        if !self.clients.get(client_id).is_some_and(|c|c.active) { return Err(DomainError::InvalidClient); }
        let eid=id("engagement",&[client_id,title,actor,provenance]);
        self.engagements.entry(eid.clone()).or_insert(Engagement{engagement_id:eid.clone(),client_id:client_id.into(),title:title.into(),engagement_type:kind,lifecycle:Lifecycle::Draft,version:1,predecessor_version:None,created_by:actor.into(),working_language:language,provenance:vec![provenance.into()],objective:None,sources:vec![],evidence:vec![],requirements:vec![],workstreams:vec![],decisions:vec![],deliverables:vec![],builds:vec![],audit_references:vec![],validation_references:vec![],stage1_gate:None});
        Ok(eid)
    }
    fn authorize(&self, g:&AuthorityGrant, client:&str, engagement:Option<&str>, op:&str)->Result<(),DomainError>{
        if !g.current{return Err(DomainError::StaleOrRevokedAuthority)}
        if g.client_id!=client{return Err(DomainError::InvalidContext)}
        if let Some(e)=engagement { if g.engagement_id.as_deref()!=Some(e){return Err(DomainError::CrossEngagementMutation)} }
        if !g.operations.iter().any(|x|x==op){return Err(DomainError::MissingAuthority)} Ok(())
    }
    fn begin_change(&self,eid:&str,expected:u64,g:&AuthorityGrant,op:&str)->Result<(),DomainError>{let e=self.engagements.get(eid).ok_or(DomainError::InvalidEngagement)?;self.authorize(g,&e.client_id,Some(eid),op)?;if e.lifecycle==Lifecycle::Closed||e.lifecycle==Lifecycle::Stopped{return Err(DomainError::ClosedEngagement)}if e.version!=expected{return Err(DomainError::StaleWrite)}Ok(())}
    fn bump(e:&mut Engagement,provenance:&str){e.predecessor_version=Some(e.version);e.version+=1;e.provenance.push(provenance.into())}
    pub fn define_objective(&mut self,eid:&str,expected:u64,client_statement:&str,interpreted:&str,provenance:&str,g:&AuthorityGrant)->Result<String,DomainError>{self.begin_change(eid,expected,g,"UPDATE_OBJECTIVE")?;let e=self.engagements.get_mut(eid).unwrap();let oid=id("objective",&[eid,client_statement,interpreted,provenance]);e.objective=Some(Objective{engagement_id:eid.into(),objective_id:oid.clone(),client_statement:client_statement.into(),interpreted_objective:interpreted.into(),success_criteria:"bounded-success".into(),constraints:vec![],exclusions:vec![],unresolved_assumptions:vec!["requires-evidence".into()],provenance:provenance.into()});e.lifecycle=Lifecycle::Defined;Self::bump(e,provenance);Ok(oid)}
    pub fn change_working_language(&mut self,eid:&str,expected:u64,language:WorkingLanguage,provenance:&str,g:&AuthorityGrant)->Result<(),DomainError>{self.begin_change(eid,expected,g,"CHANGE_WORKING_LANGUAGE")?;let e=self.engagements.get_mut(eid).unwrap();e.working_language=language;Self::bump(e,provenance);Ok(())}
    pub fn register_source(&mut self,eid:&str,expected:u64,original_language:&str,content_ref:&str,fingerprint:&str,provenance:&str,g:&AuthorityGrant)->Result<String,DomainError>{self.begin_change(eid,expected,g,"REGISTER_SOURCE")?;let e=self.engagements.get_mut(eid).unwrap();let sid=id("source",&[eid,content_ref,fingerprint,provenance]);e.sources.push(SourceReference{engagement_id:eid.into(),source_id:sid.clone(),original_language:original_language.into(),original_content_ref:content_ref.into(),fingerprint:fingerprint.into(),provenance:provenance.into(),access_scope:eid.into()});Self::bump(e,provenance);Ok(sid)}
    pub fn register_evidence(&mut self,eid:&str,expected:u64,source_id:&str,class:ClaimClass,provenance:&str,g:&AuthorityGrant)->Result<String,DomainError>{self.begin_change(eid,expected,g,"REGISTER_EVIDENCE_REFERENCE")?;let e=self.engagements.get_mut(eid).unwrap();if !e.sources.iter().any(|s|s.source_id==source_id&&s.engagement_id()==eid){return Err(DomainError::EvidenceInvariant)}let x=id("evidence",&[eid,source_id,&format!("{class:?}"),provenance]);e.evidence.push(EvidenceReference{engagement_id:eid.into(),evidence_id:x.clone(),source_id:source_id.into(),classification:class,validation_status:"UNVALIDATED_REFERENCE".into(),provenance:provenance.into()});Self::bump(e,provenance);Ok(x)}
    pub fn add_requirement(&mut self,eid:&str,expected:u64,description:&str,source_ref:&str,provenance:&str,g:&AuthorityGrant)->Result<String,DomainError>{self.begin_change(eid,expected,g,"ADD_REQUIREMENT")?;let e=self.engagements.get_mut(eid).unwrap();let owned=e.sources.iter().any(|s|s.source_id==source_ref&&s.engagement_id()==eid)||e.evidence.iter().any(|x|x.evidence_id==source_ref&&x.engagement_id()==eid);if !owned{return Err(DomainError::EvidenceInvariant)}let x=id("requirement",&[eid,description,source_ref,provenance]);e.requirements.push(Requirement{engagement_id:eid.into(),requirement_id:x.clone(),description:description.into(),category:"DOMAIN".into(),source_ref:source_ref.into(),status:"OPEN".into(),version:1,predecessor:None,provenance:provenance.into()});Self::bump(e,provenance);Ok(x)}
    pub fn create_workstream(&mut self,eid:&str,expected:u64,kind:&str,provenance:&str,g:&AuthorityGrant)->Result<String,DomainError>{self.begin_change(eid,expected,g,"CREATE_WORKSTREAM")?;let e=self.engagements.get_mut(eid).unwrap();let x=id("workstream",&[eid,kind,provenance]);e.workstreams.push(Workstream{engagement_id:eid.into(),workstream_id:x.clone(),kind:kind.into(),provenance:provenance.into()});Self::bump(e,provenance);Ok(x)}
    pub fn request_decision(&mut self,eid:&str,expected:u64,kind:DecisionType,recommendation_ref:&str,provenance:&str,g:&AuthorityGrant)->Result<String,DomainError>{self.begin_change(eid,expected,g,"REQUEST_DECISION")?;let e=self.engagements.get_mut(eid).unwrap();let x=id("decision",&[eid,&format!("{kind:?}"),recommendation_ref,provenance]);e.decisions.push(Decision{engagement_id:eid.into(),decision_id:x.clone(),decision_type:kind,recommendation_ref:recommendation_ref.into(),human_decision:None,provenance:provenance.into()});Self::bump(e,provenance);Ok(x)}
    pub fn register_deliverable(&mut self,eid:&str,expected:u64,title:&str,source_refs:Vec<String>,as_original_evidence:bool,provenance:&str,g:&AuthorityGrant)->Result<String,DomainError>{self.begin_change(eid,expected,g,"REGISTER_DELIVERABLE")?;if as_original_evidence{return Err(DomainError::EvidenceInvariant)}let e=self.engagements.get_mut(eid).unwrap();let owned=|r:&String|e.sources.iter().any(|s|s.source_id==*r&&s.engagement_id()==eid)||e.evidence.iter().any(|x|x.evidence_id==*r&&x.engagement_id()==eid)||e.builds.iter().any(|b|b.build_id==*r&&b.engagement_id()==eid);if !source_refs.iter().all(owned){return Err(DomainError::EvidenceInvariant)}let x=id("deliverable",&[eid,title,provenance]);e.deliverables.push(Deliverable{engagement_id:eid.into(),deliverable_id:x.clone(),title:title.into(),kind:"REPORT".into(),version:1,output_language:e.working_language,source_refs,status:"DRAFT".into(),provenance:provenance.into(),audit_ref:None,validation_ref:None});Self::bump(e,provenance);Ok(x)}
    pub fn register_build(&mut self,eid:&str,expected:u64,artifact_ref:&str,provenance:&str,g:&AuthorityGrant)->Result<String,DomainError>{self.begin_change(eid,expected,g,"REGISTER_DELIVERABLE")?;let e=self.engagements.get_mut(eid).unwrap();let x=id("build",&[eid,artifact_ref,provenance]);e.builds.push(BuildReference{engagement_id:eid.into(),build_id:x.clone(),artifact_ref:artifact_ref.into(),test_ref:None,provenance:provenance.into()});Self::bump(e,provenance);Ok(x)}
    pub fn transition(&mut self,eid:&str,expected:u64,next:Lifecycle,provenance:&str,g:&AuthorityGrant)->Result<(),DomainError>{self.begin_change(eid,expected,g,"TRANSITION_ENGAGEMENT")?;let e=self.engagements.get_mut(eid).unwrap();let valid=matches!((e.lifecycle,next),(Lifecycle::Defined,Lifecycle::Active)|(Lifecycle::Draft,Lifecycle::Stopped)|(Lifecycle::Defined,Lifecycle::Stopped)|(Lifecycle::Active,Lifecycle::Stopped));if !valid{return Err(DomainError::InvalidTransition)}e.lifecycle=next;Self::bump(e,provenance);Ok(())}
    pub fn attach_stage1_gate(&mut self,eid:&str,expected:u64,result:&KernelResult,provenance:&str,g:&AuthorityGrant)->Result<(),DomainError>{self.begin_change(eid,expected,g,"REQUEST_DECISION")?;let h=result.validation_handoff.as_ref().ok_or(DomainError::ValidationRequired)?;if result.classification!=IntegrationClassification::AwaitHumanPrincipal||result.human_gate!=Some(HumanGateState::AwaitingHumanPrincipalDecision)||h.status!="INDEPENDENT_VALIDATION_COMPLETE"||h.validator_verdict.as_deref()!=Some("CERTIFIED_PASS")||h.validated_candidate_version.as_deref()!=Some(STAGE_I_CANDIDATE_VERSION)||h.validated_candidate_id.as_ref()!=result.result_id.as_ref(){return Err(DomainError::HumanPrincipalBypass)}let e=self.engagements.get_mut(eid).unwrap();if e.lifecycle!=Lifecycle::Active{return Err(DomainError::InvalidTransition)}e.stage1_gate=Some(StageIReference{candidate_version:h.validated_candidate_version.clone().unwrap(),result_id:result.result_id.clone().unwrap(),package_fingerprint:h.package_fingerprint.clone().unwrap(),validator_verdict:"CERTIFIED_PASS".into(),human_gate:HumanGateState::AwaitingHumanPrincipalDecision});e.validation_references.push(h.package_fingerprint.clone().unwrap());e.audit_references.push(result.audit_record_hash.clone().unwrap());e.lifecycle=Lifecycle::AwaitingHumanDecision;Self::bump(e,provenance);Ok(())}
    pub fn apply_human_decision(&mut self,eid:&str,expected:u64,actor:&str,validated_result_id:&str,approve:bool,provenance:&str,g:&AuthorityGrant)->Result<(),DomainError>{self.begin_change(eid,expected,g,"CLOSE_ENGAGEMENT")?;let e=self.engagements.get_mut(eid).unwrap();let bound=e.lifecycle==Lifecycle::AwaitingHumanDecision&&e.stage1_gate.as_ref().is_some_and(|s|s.human_gate==HumanGateState::AwaitingHumanPrincipalDecision&&s.validator_verdict=="CERTIFIED_PASS"&&s.result_id==validated_result_id);if actor!="human-principal:titus-lab"||!bound{return Err(DomainError::HumanPrincipalBypass)}if let Some(d)=e.decisions.last_mut(){d.human_decision=Some(approve)}e.lifecycle=if approve{Lifecycle::Closed}else{Lifecycle::Stopped};Self::bump(e,provenance);Ok(())}
    pub fn localized_artifact(&self,eid:&str,source_id:&str,target:WorkingLanguage,provenance:&str)->Result<DerivedLocalizedArtifact,DomainError>{let e=self.engagements.get(eid).ok_or(DomainError::InvalidEngagement)?;let s=e.sources.iter().find(|s|s.source_id==source_id).ok_or(DomainError::EvidenceInvariant)?;Ok(DerivedLocalizedArtifact{derivation_id:id("localized",&[eid,source_id,&format!("{target:?}"),provenance]),source_id:source_id.into(),original_language:s.original_language.clone(),target_language:target,provenance:provenance.into()})}
}

pub fn all_operations()->Vec<String>{["CREATE_ENGAGEMENT","UPDATE_OBJECTIVE","CHANGE_WORKING_LANGUAGE","ADD_REQUIREMENT","CREATE_WORKSTREAM","REGISTER_SOURCE","REGISTER_EVIDENCE_REFERENCE","REGISTER_DELIVERABLE","REQUEST_DECISION","TRANSITION_ENGAGEMENT","CLOSE_ENGAGEMENT"].iter().map(|s|s.to_string()).collect()}
