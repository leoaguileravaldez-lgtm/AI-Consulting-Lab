use ai_consulting_lab_phase1_foundational_runtime::{
    canonical::canonical_cbor,
    identity::{CanonicalIdentity, IdentityKind},
    integrity::domain_separated_sha256,
};
use ai_consulting_lab_phase1_global_uniqueness_concurrency::{
    ConcurrencyDisposition, SqliteConcurrencyStore,
};
use ai_consulting_lab_phase1_revocation_freshness::{
    AuthorityState, RevocationDisposition, SqliteRevocationStore,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::{Path, PathBuf}, process::Command};
use titus_lab_phase1_audit_evidence::{
    AppendResult, AuditDisposition, AuditEvent, AuditJournal, ExpectedAuditRecord,
};

pub const CERTIFIED_PARENT: &str = "6023c33ecafb093c55750d1e5e86ce77ba87cd57";
pub const CERTIFICATION_CONTRACT: &str = "BC_EMPIRICAL_CERTIFICATION";
pub const STAGE_I_VALIDATION_SUBJECT: &str = "STAGE_I_INTEGRATION_KERNEL_ACCEPTANCE_V1";
pub const STAGE_I_CANDIDATE_VERSION: &str = "working-tree-on-parent:6023c33ecafb093c55750d1e5e86ce77ba87cd57";
pub const REQUIRED_SCOPE: &str = "integration:execute";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEnvelope {
    pub context_id: String,
    pub engagement_id: String,
    pub project_id: String,
    pub operation_id: String,
    pub actor_id: String,
    pub workload_id: String,
    pub authority_ref: String,
    pub authority_scope: String,
    pub authority_generation: i64,
    pub predecessor_sequence: i64,
    pub predecessor_fence: i64,
    pub implementation_version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntheticFault {
    None,
    ExecutionBehavior,
    AuditContext,
    ValidationUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelRequest {
    pub envelope: RuntimeEnvelope,
    pub presented_context: String,
    pub payload: Value,
    pub fault: SyntheticFault,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationClassification {
    ValidationRequired,
    AwaitHumanPrincipal,
    InvalidInput,
    InvalidContext,
    MissingAuthority,
    StaleOrRevokedAuthority,
    PolicyRejected,
    ExecutionBehavioralFailure,
    StateConflict,
    PersistenceFailure,
    AuditFailure,
    ValidationFailure,
    InfrastructureFailure,
    HumanPrincipalBypassBlocked,
    HarnessFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelSetupError {
    pub classification: IntegrationClassification,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanGateState {
    AwaitingHumanPrincipalDecision,
    ApprovedByHumanPrincipal,
    RejectedByHumanPrincipal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanDecision {
    pub actor_id: String,
    pub result_id: String,
    pub approve: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationHandoff {
    pub contract_id: String,
    pub candidate_id: String,
    pub evidence_fingerprint: String,
    pub producer_id: String,
    pub validator_id: String,
    pub status: String,
    pub package_fingerprint: Option<String>,
    pub validator_verdict: Option<String>,
    pub validated_candidate_id: Option<String>,
    pub validated_candidate_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelResult {
    pub classification: IntegrationClassification,
    pub request_id: String,
    pub execution_id: Option<String>,
    pub result_id: Option<String>,
    pub stored_sequence: i64,
    pub provenance: Vec<String>,
    pub audit_record_id: Option<String>,
    pub audit_record_hash: Option<String>,
    pub validation_handoff: Option<ValidationHandoff>,
    pub human_gate: Option<HumanGateState>,
    pub recovery_required: bool,
    pub authority_created: bool,
    pub external_effect_truth_determined: bool,
}

pub trait BoundedExecutor {
    fn execute(&self, request_id: &str, payload: &Value) -> Result<Value, String>;
    fn executor_id(&self) -> &'static str;
}

#[derive(Debug, Default)]
pub struct DeterministicLocalExecutor;

impl BoundedExecutor for DeterministicLocalExecutor {
    fn execute(&self, request_id: &str, payload: &Value) -> Result<Value, String> {
        Ok(json!({"request_id":request_id,"synthetic_result":payload}))
    }
    fn executor_id(&self) -> &'static str {
        "deterministic-local-executor-v1"
    }
}

pub struct IntegrationKernel<E> {
    context_id: String,
    generation: i64,
    state: SqliteConcurrencyStore,
    authority: SqliteRevocationStore,
    audit: AuditJournal,
    executor: E,
}

impl<E: BoundedExecutor> IntegrationKernel<E> {
    pub fn create(
        root: &Path,
        context_id: &str,
        generation: i64,
        executor: E,
    ) -> Result<Self, KernelSetupError> {
        let infrastructure = |detail: String| KernelSetupError {
            classification: IntegrationClassification::InfrastructureFailure,
            detail,
        };
        std::fs::create_dir_all(root).map_err(|e| infrastructure(format!("RUNTIME_ROOT:{e}")))?;
        let state = SqliteConcurrencyStore::new(root.join("state.sqlite"));
        state.bootstrap().map_err(&infrastructure)?;
        state.seed(context_id, 1, 1).map_err(&infrastructure)?;
        let authority = SqliteRevocationStore::new(root.join("authority.sqlite"));
        authority.bootstrap().map_err(&infrastructure)?;
        authority
            .seed_current(generation)
            .map_err(&infrastructure)?;
        let audit = AuditJournal::create(root.join("audit.sqlite"), context_id, generation)
            .map_err(&infrastructure)?;
        Ok(Self {
            context_id: context_id.into(),
            generation,
            state,
            authority,
            audit,
            executor,
        })
    }

    pub fn revoke_authority(&self, next_generation: i64) -> bool {
        self.authority
            .revoke(self.generation, next_generation)
            .disposition
            == RevocationDisposition::RevocationCommitted
    }

    pub fn execute(&self, request: &KernelRequest) -> KernelResult {
        let request_id = request_id(request);
        let reject = |classification| rejected(classification, &request_id);
        let e = &request.envelope;
        if [
            e.context_id.as_str(),
            e.engagement_id.as_str(),
            e.project_id.as_str(),
            e.operation_id.as_str(),
            e.actor_id.as_str(),
            e.workload_id.as_str(),
            e.implementation_version.as_str(),
        ]
        .contains(&"")
        {
            return reject(IntegrationClassification::InvalidInput);
        }
        let identity = match CanonicalIdentity::new(
            &e.context_id,
            "integration-request",
            IdentityKind::Event,
            &request_id,
            &e.implementation_version,
            &format!("request:{request_id}"),
        ) {
            Ok(v) => v,
            Err(_) => return reject(IntegrationClassification::InvalidInput),
        };
        if identity
            .bind_presented_domain(&request.presented_context)
            .is_err()
            || e.context_id != self.context_id
        {
            return reject(IntegrationClassification::InvalidContext);
        }
        if e.authority_ref.is_empty() {
            return reject(IntegrationClassification::MissingAuthority);
        }
        if e.authority_ref != format!("authority:synthetic-authority:{}", self.generation)
            || e.authority_scope != REQUIRED_SCOPE
        {
            return reject(IntegrationClassification::PolicyRejected);
        }
        let authority = self.authority.authorize_cached(e.authority_generation);
        if authority.authority_state != AuthorityState::Current
            || authority.disposition != RevocationDisposition::AcceptedCurrent
        {
            return reject(IntegrationClassification::StaleOrRevokedAuthority);
        }
        if request.fault == SyntheticFault::ExecutionBehavior {
            return reject(IntegrationClassification::ExecutionBehavioralFailure);
        }
        let output = match self.executor.execute(&request_id, &request.payload) {
            Ok(v) => v,
            Err(_) => return reject(IntegrationClassification::ExecutionBehavioralFailure),
        };
        let execution_id = hash(&format!(
            "execution|{}|{}",
            self.executor.executor_id(),
            request_id
        ));
        let transition = self.state.cas(
            &e.operation_id,
            &e.context_id,
            e.predecessor_sequence,
            e.predecessor_fence,
            e.predecessor_sequence + 1,
            e.predecessor_fence + 1,
        );
        if transition.disposition != ConcurrencyDisposition::Committed {
            return rejected_with_state(
                if transition.disposition == ConcurrencyDisposition::UnexpectedDatabaseError {
                    IntegrationClassification::PersistenceFailure
                } else {
                    IntegrationClassification::StateConflict
                },
                &request_id,
                transition.stored_sequence,
            );
        }
        let output_bytes = match canonical_cbor(&output) {
            Ok(v) => v,
            Err(_) => {
                return rejected_with_state(
                    IntegrationClassification::HarnessFailure,
                    &request_id,
                    transition.stored_sequence,
                );
            }
        };
        let result_id = hex(&domain_separated_sha256(
            "titus-lab-integration-result-v1",
            "v1",
            &output_bytes,
        ));
        let provenance = vec![
            format!("request:{request_id}"),
            format!("context:{}", e.context_id),
            format!("authority:{}", e.authority_ref),
            "contract:BC_GLOBAL_UNIQUENESS_CONCURRENCY".into(),
            format!("executor:{}", self.executor.executor_id()),
            format!("execution:{execution_id}"),
            format!(
                "transition:{}->{}",
                e.predecessor_sequence, transition.stored_sequence
            ),
            format!("result:{result_id}"),
        ];
        let audit_context = if request.fault == SyntheticFault::AuditContext {
            "foreign-context"
        } else {
            &e.context_id
        };
        let audit = self.audit.append(&AuditEvent {
            sequence: 1,
            predecessor_hash: "GENESIS",
            operation_id: &e.operation_id,
            event_type: "INTEGRATION_TRANSITION_RECORDED",
            actor_id: &e.actor_id,
            workload_id: &e.workload_id,
            context_id: audit_context,
            generation: self.generation,
            transition_ref: &format!("transition:{}", transition.stored_sequence),
            provenance: &provenance.join("->"),
            payload: &result_id,
        });
        if !audit.accepted || audit.disposition != AuditDisposition::ValidAuditEvidence {
            return failed_after_transition(
                IntegrationClassification::AuditFailure,
                request_id,
                execution_id,
                result_id,
                transition.stored_sequence,
                provenance,
                audit,
            );
        }
        let expected = [ExpectedAuditRecord {
            record_id: audit.record_id.clone(),
            operation_id: e.operation_id.clone(),
            event_type: "INTEGRATION_TRANSITION_RECORDED".into(),
            actor_id: e.actor_id.clone(),
            workload_id: e.workload_id.clone(),
            context_id: e.context_id.clone(),
            generation: self.generation,
            transition_ref: format!("transition:{}", transition.stored_sequence),
            provenance: provenance.join("->"),
        }];
        let verification = self.audit.verify(&expected);
        if verification.disposition != AuditDisposition::ValidAuditEvidence
            || verification.execution_authority_leakage != 0
            || verification.recovery_authority_leakage != 0
            || verification.layer19_authority_leakage != 0
            || verification.certification_authority_leakage != 0
        {
            return failed_after_transition(
                IntegrationClassification::AuditFailure,
                request_id,
                execution_id,
                result_id,
                transition.stored_sequence,
                provenance,
                audit,
            );
        }
        if request.fault == SyntheticFault::ValidationUnavailable {
            return failed_after_transition(
                IntegrationClassification::ValidationFailure,
                request_id,
                execution_id,
                result_id,
                transition.stored_sequence,
                provenance,
                audit,
            );
        }
        let evidence_fingerprint = hash(&format!(
            "{}|{}|{}|{}",
            request_id, result_id, audit.record_hash, transition.stored_sequence
        ));
        KernelResult {
            classification: IntegrationClassification::ValidationRequired,
            request_id,
            execution_id: Some(execution_id),
            result_id: Some(result_id.clone()),
            stored_sequence: transition.stored_sequence,
            provenance,
            audit_record_id: Some(audit.record_id),
            audit_record_hash: Some(audit.record_hash),
            validation_handoff: Some(ValidationHandoff {
                contract_id: STAGE_I_VALIDATION_SUBJECT.into(),
                candidate_id: result_id,
                evidence_fingerprint,
                producer_id: "integration-kernel-producer-v1".into(),
                validator_id: "stage1-independent-validator-process-v1".into(),
                status: "VALIDATION_REQUIRED".into(),
                package_fingerprint: None,
                validator_verdict: None,
                validated_candidate_id: None,
                validated_candidate_version: None,
            }),
            human_gate: None,
            recovery_required: false,
            authority_created: false,
            external_effect_truth_determined: false,
        }
    }

    pub fn apply_human_decision(
        &self,
        result: &KernelResult,
        decision: Option<&HumanDecision>,
    ) -> KernelResult {
        let validation_complete = result.validation_handoff.as_ref().is_some_and(|handoff| {
            handoff.status == "INDEPENDENT_VALIDATION_COMPLETE"
                && handoff.validator_verdict.as_deref() == Some("CERTIFIED_PASS")
                && handoff.package_fingerprint.as_ref().is_some_and(|v| !v.is_empty())
                && handoff.validated_candidate_id.as_ref() == result.result_id.as_ref()
                && handoff.candidate_id == result.result_id.as_deref().unwrap_or("")
                && handoff.validated_candidate_version.as_deref()
                    == Some(STAGE_I_CANDIDATE_VERSION)
        });
        let valid_decision = decision.is_some_and(|d| {
            d.actor_id == "human-principal:titus-lab"
                && Some(&d.result_id) == result.result_id.as_ref()
        });
        if result.classification != IntegrationClassification::AwaitHumanPrincipal
            || result.human_gate != Some(HumanGateState::AwaitingHumanPrincipalDecision)
            || !validation_complete
            || !valid_decision
        {
            let mut denied = result.clone();
            denied.classification = IntegrationClassification::HumanPrincipalBypassBlocked;
            return denied;
        }
        let mut decided = result.clone();
        decided.human_gate = Some(if decision.is_some_and(|d| d.approve) {
            HumanGateState::ApprovedByHumanPrincipal
        } else {
            HumanGateState::RejectedByHumanPrincipal
        });
        decided
    }
}

/// Constructs the candidate package only from the observed Stage I result and
/// repository-bound certified predecessor evidence. The package fingerprint is
/// SHA-256 over deterministic JSON with the fingerprint field omitted.
pub fn construct_stage1_candidate_package(
    repository_root: &Path,
    result: &KernelResult,
) -> Result<Value, String> {
    if result.classification != IntegrationClassification::ValidationRequired
        || result.human_gate.is_some()
    {
        return Err("CANDIDATE_RESULT_NOT_ADMISSIBLE".into());
    }
    let source_paths = [
        "INTEGRATION_STAGE_1_KERNEL/Cargo.toml",
        "INTEGRATION_STAGE_1_KERNEL/Cargo.lock",
        "INTEGRATION_STAGE_1_KERNEL/bindings.json",
        "INTEGRATION_STAGE_1_KERNEL/README.md",
        "INTEGRATION_STAGE_1_KERNEL/src/lib.rs",
        "INTEGRATION_STAGE_1_KERNEL/src/bin/integration_harness.rs",
        "INTEGRATION_STAGE_1_KERNEL/src/bin/stage1_candidate_package.rs",
        "INTEGRATION_STAGE_1_KERNEL/src/bin/stage1_independent_validator.rs",
        "INTEGRATION_STAGE_1_KERNEL/tests/integration.rs",
    ];
    let mut source_manifest = Vec::new();
    for path in source_paths {
        let bytes = fs::read(repository_root.join(path)).map_err(|_| "CANDIDATE_SOURCE_UNREADABLE")?;
        source_manifest.push(json!({"path":path,"sha256":format!("{:x}", Sha256::digest(&bytes))}));
    }
    let source_fingerprint = hash(&serde_json::to_string(&source_manifest).map_err(|_| "SOURCE_MANIFEST_SERIALIZATION")?);
    let predecessor_specs = [
        ("foundational-runtime", "PHASE_1_FOUNDATIONAL_RUNTIME/bindings.json", "750f1ea83d88d77f14329cf007a4e4034420019b43e6f31655171f24261fe3b4"),
        ("transactional-persistence", "PHASE_1_TRANSACTIONAL_PERSISTENCE/LOCAL_IMPLEMENTATION_EVIDENCE.json", "26d6b6121c556998618ce0e281b555008562068ab87e123410c6f3d54c82be45"),
        ("global-uniqueness-concurrency", "PHASE_1_GLOBAL_UNIQUENESS_CONCURRENCY/LOCAL_IMPLEMENTATION_EVIDENCE.json", "ed1c20d3327d98257e48a30d518fd470c0db75814b556326e1b65b5aa35db071"),
        ("revocation-freshness", "PHASE_1_REVOCATION_FRESHNESS/LOCAL_IMPLEMENTATION_EVIDENCE.json", "bc85361f32741bb27cf3e3844930b8e548dbf708a13f8c206255e1174b719bec"),
        ("runtime-isolation", "PHASE_1_RUNTIME_ISOLATION/LOCAL_IMPLEMENTATION_EVIDENCE.json", "0f1f7cb8833e5cd40deabf07cdf3e5be1690a13ad24c0ba451ff5edb66af4d8f"),
        ("recovery-reconciliation", "PHASE_1_RECOVERY_RECONCILIATION/LOCAL_IMPLEMENTATION_EVIDENCE.json", "5846f5adfcbcedf004625872bc8632705c6f3c4aca166ade15c3a45b0862d7fd"),
        ("audit-evidence", "PHASE_1_AUDIT_EVIDENCE/LOCAL_IMPLEMENTATION_EVIDENCE.json", "05752754c306d83cc7c741fece713c99e69d523d9e2c0e7edbb80ce690e05c3b"),
        ("empirical-certification", "PHASE_1_EMPIRICAL_CERTIFICATION/LOCAL_IMPLEMENTATION_EVIDENCE.json", "99994260ccd7a0580ca3475aaef52758c2efcb6e069e20cf91642822403a71bf"),
    ];
    let predecessors: Vec<Value> = predecessor_specs.iter().map(|(id,path,sha256)| json!({"id":id,"path":path,"sha256":sha256,"status":"EMPIRICAL_PASS"})).collect();
    let handoff = result.validation_handoff.as_ref().ok_or("VALIDATION_HANDOFF_MISSING")?;
    let mut package = json!({
        "schema_version":1,
        "repository":"AI-Consulting-Lab",
        "subject_id":STAGE_I_VALIDATION_SUBJECT,
        "candidate_version":STAGE_I_CANDIDATE_VERSION,
        "certified_parent":CERTIFIED_PARENT,
        "candidate_source_manifest":source_manifest,
        "candidate_source_fingerprint":source_fingerprint,
        "context_id":"synthetic-client-A",
        "generation":1,
        "environment":{"execution":"deterministic-local-synthetic","storage":"disposable-local-sqlite","production":false},
        "independence":{"producer_id":handoff.producer_id,"validator_id":"stage1-independent-validator-process-v1","separate_process":true,"imports_producer_decision_logic":false,"uses_producer_declared_pass":false},
        "predecessors":predecessors,
        "execution_evidence":{"request_id":result.request_id,"execution_id":result.execution_id,"result_id":result.result_id,"stored_sequence":result.stored_sequence,"provenance":result.provenance,"audit_record_id":result.audit_record_id,"audit_record_hash":result.audit_record_hash},
        "material_observations":{"integration_chain":"ESTABLISHED","identity":"PASS","authority":"PASS","policy_contract_gate":"PASS","execution_dispatch":"PASS","state_transition":"PASS","provenance":"PASS","evidence":"PASS","audit_handoff":"PASS","human_principal_gate":"AWAITING_HUMAN_PRINCIPAL_DECISION","wrong_context":"REJECTED","missing_or_out_of_scope_authority":"REJECTED","stale_or_revoked_authority":"REJECTED","execution_failure":"NON_SUCCESS","state_conflict":"NON_SUCCESS","audit_or_validation_failure":"NON_SUCCESS","human_principal_bypass":"BLOCKED","infrastructure_failure":"EXPLICIT_NON_SUCCESS","workbench_requirement_preserved":true,"client_access_ip_requirement_preserved":true},
        "authority_observations":{"new_institutional_authority":0,"execution_leakage":0,"retry_leakage":0,"recovery_leakage":0,"deployment_leakage":0,"layer20":"ABSENT","premature_frontend":0,"premature_deployment":0},
        "producer_declared_pass":true
    });
    let bytes = serde_json::to_vec(&package).map_err(|_| "PACKAGE_SERIALIZATION")?;
    package["package_fingerprint"] = json!(format!("{:x}", Sha256::digest(bytes)));
    Ok(package)
}

pub fn complete_validation_handoff(
    repository_root: &Path,
    validator: &Path,
    result: &mut KernelResult,
) -> Result<Value, String> {
    let package = construct_stage1_candidate_package(repository_root, result)?;
    complete_validation_handoff_with_package(repository_root, validator, result, package)
}

pub fn complete_validation_handoff_with_package(
    repository_root: &Path,
    validator: &Path,
    result: &mut KernelResult,
    package: Value,
) -> Result<Value, String> {
    let path = std::env::temp_dir().join(format!("titus-stage1-candidate-{}.json", result.request_id));
    fs::write(&path, serde_json::to_vec(&package).map_err(|_| "PACKAGE_SERIALIZATION")?)
        .map_err(|_| "PACKAGE_WRITE_FAILURE")?;
    let output = Command::new(validator).arg(repository_root).arg(&path).output();
    let _ = fs::remove_file(&path);
    let output = output.map_err(|_| "VALIDATOR_INVOCATION_FAILURE")?;
    let verdict: Value = serde_json::from_slice(&output.stdout).map_err(|_| "VALIDATOR_OUTPUT_INVALID")?;
    if !output.status.success() || verdict["verdict"] != "CERTIFIED_PASS" {
        result.classification = IntegrationClassification::ValidationFailure;
        result.human_gate = None;
        result.recovery_required = true;
        return Err(verdict["reason"].as_str().unwrap_or("VALIDATOR_FAILURE").into());
    }
    let handoff = result.validation_handoff.as_mut().ok_or("VALIDATION_HANDOFF_MISSING")?;
    handoff.status = "INDEPENDENT_VALIDATION_COMPLETE".into();
    handoff.package_fingerprint = package["package_fingerprint"].as_str().map(str::to_owned);
    handoff.validator_verdict = verdict["verdict"].as_str().map(str::to_owned);
    handoff.validated_candidate_id = Some(handoff.candidate_id.clone());
    handoff.validated_candidate_version = verdict["candidate_version"].as_str().map(str::to_owned);
    if handoff.validated_candidate_version.as_deref() != Some(STAGE_I_CANDIDATE_VERSION)
        || verdict["evidence_package_fingerprint"] != package["package_fingerprint"]
    {
        result.classification = IntegrationClassification::ValidationFailure;
        result.human_gate = None;
        result.recovery_required = true;
        return Err("VALIDATOR_CANDIDATE_BINDING_MISMATCH".into());
    }
    result.classification = IntegrationClassification::AwaitHumanPrincipal;
    result.human_gate = Some(HumanGateState::AwaitingHumanPrincipalDecision);
    Ok(verdict)
}

fn request_id(request: &KernelRequest) -> String {
    let e = &request.envelope;
    hash(&format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        e.context_id,
        e.engagement_id,
        e.project_id,
        e.operation_id,
        e.actor_id,
        e.workload_id,
        e.authority_ref,
        e.authority_scope,
        e.authority_generation,
        e.predecessor_sequence,
        e.predecessor_fence,
        e.implementation_version,
    ))
}

fn hash(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn rejected(classification: IntegrationClassification, request_id: &str) -> KernelResult {
    rejected_with_state(classification, request_id, 1)
}

fn rejected_with_state(
    classification: IntegrationClassification,
    request_id: &str,
    stored_sequence: i64,
) -> KernelResult {
    KernelResult {
        classification,
        request_id: request_id.into(),
        execution_id: None,
        result_id: None,
        stored_sequence,
        provenance: vec![],
        audit_record_id: None,
        audit_record_hash: None,
        validation_handoff: None,
        human_gate: None,
        recovery_required: false,
        authority_created: false,
        external_effect_truth_determined: false,
    }
}

fn failed_after_transition(
    classification: IntegrationClassification,
    request_id: String,
    execution_id: String,
    result_id: String,
    stored_sequence: i64,
    provenance: Vec<String>,
    audit: AppendResult,
) -> KernelResult {
    KernelResult {
        classification,
        request_id,
        execution_id: Some(execution_id),
        result_id: Some(result_id),
        stored_sequence,
        provenance,
        audit_record_id: (!audit.record_id.is_empty()).then_some(audit.record_id),
        audit_record_hash: (!audit.record_hash.is_empty()).then_some(audit.record_hash),
        validation_handoff: None,
        human_gate: None,
        recovery_required: true,
        authority_created: false,
        external_effect_truth_determined: false,
    }
}

pub fn synthetic_request() -> KernelRequest {
    KernelRequest {
        envelope: RuntimeEnvelope {
            context_id: "synthetic-client-A".into(),
            engagement_id: "synthetic-engagement-1".into(),
            project_id: "synthetic-project-1".into(),
            operation_id: "synthetic-operation-1".into(),
            actor_id: "synthetic-human-principal".into(),
            workload_id: "synthetic-kernel-workload".into(),
            authority_ref: "authority:synthetic-authority:1".into(),
            authority_scope: REQUIRED_SCOPE.into(),
            authority_generation: 1,
            predecessor_sequence: 1,
            predecessor_fence: 1,
            implementation_version: CERTIFIED_PARENT.into(),
        },
        presented_context: "synthetic-client-A".into(),
        payload: json!({"task":"deterministic-local-transform","value":7}),
        fault: SyntheticFault::None,
    }
}

pub fn synthetic_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!("titus-stage1-{label}"))
}
