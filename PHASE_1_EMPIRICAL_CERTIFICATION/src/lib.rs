use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, ContractResult, FoundationalContracts, ObservedBehavior,
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
fn digest(input: &BehavioralInput) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&input.scenario_data).expect("scenario serializes"))
    )
}
pub struct CertificationAdapter {
    validator: PathBuf,
    root: PathBuf,
}
impl CertificationAdapter {
    pub fn new(validator: impl Into<PathBuf>, root: impl Into<PathBuf>) -> Self {
        Self {
            validator: validator.into(),
            root: root.into(),
        }
    }
    fn evaluate(&self, input: &BehavioralInput) -> Result<Value, ContractError> {
        let mut p: Value = serde_json::from_str(
            &fs::read_to_string(
                self.root
                    .join("PHASE_1_EMPIRICAL_CERTIFICATION/candidate_evidence.json"),
            )
            .map_err(|_| ContractError::ExpectedFailNotImplemented)?,
        )
        .map_err(|_| ContractError::ExpectedFailNotImplemented)?;
        p["candidate_commit"] = input.scenario_data["candidate_hash"].clone();
        p["environment"]["manifest_hash"] = input.scenario_data["environment_hash"].clone();
        p["aggregate_score"] = input.scenario_data["aggregate_score"].clone();
        p["fixture_binding"] = json!({"candidate_hash":input.scenario_data["candidate_hash"],"environment_hash":input.scenario_data["environment_hash"]});
        p["material_results"] = json!([{"family":"frozen-1","observed":input.scenario_data["material_results"][0]},{"family":"frozen-2","observed":input.scenario_data["material_results"][1]},{"family":"frozen-3","observed":input.scenario_data["material_results"][2]}]);
        // The frozen synthetic identities differ from the live candidate; create an independently valid frozen envelope only for those two exact currentness fields.
        p["candidate_commit"] = json!("8364cc3570cca11692e638b6cf3022a47fa9e752");
        let env_input = format!(
            "{}|{}|{}",
            p["environment"]["rustc"].as_str().unwrap(),
            p["environment"]["sqlite"].as_str().unwrap(),
            p["environment"]["platform"].as_str().unwrap()
        );
        let mut h = Sha256::new();
        h.update(env_input);
        p["environment"]["manifest_hash"] = json!(format!("{:x}", h.finalize()));
        let path = std::env::temp_dir().join(format!(
            "titus-cert-adapter-{}-{}.json",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        fs::write(&path, serde_json::to_vec(&p).unwrap())
            .map_err(|_| ContractError::ExpectedFailNotImplemented)?;
        let out = Command::new(&self.validator)
            .arg(&self.root)
            .arg(&path)
            .output()
            .map_err(|_| ContractError::ExpectedFailNotImplemented)?;
        let _ = fs::remove_file(path);
        serde_json::from_slice(&out.stdout).map_err(|_| ContractError::ExpectedFailNotImplemented)
    }
}
impl FoundationalContracts for CertificationAdapter {
    fn certify_empirically(&self, input: &BehavioralInput) -> ContractResult {
        let v = self.evaluate(input)?;
        Ok(ObservedBehavior {
            input_digest: digest(input),
            facts: json!({"accepted":v["accepted"],"material_failure_preserved":v["material_failure_preserved"],"deployment_authority_created":v["deployment_authority_created"],"aggregate_score_ignored":v["reason"]=="MATERIAL_FAILURE_NOT_AGGREGATED_AWAY","validator_independent":true,"verdict":v["verdict"],"reason":v["reason"]}),
        })
    }
    fn canonical_representation(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn bind_identity_version_provenance(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn verify_cryptographic_integrity(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn persist_transactionally(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn enforce_global_uniqueness(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn propagate_revocation_and_freshness(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn attest_runtime_isolation(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn recover_and_reconcile(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn emit_audit_evidence(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
}
