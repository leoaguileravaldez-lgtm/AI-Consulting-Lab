use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, ContractResult, FoundationalContracts, ObservedBehavior,
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{
    canonical::canonical_cbor,
    identity::{CanonicalIdentity, IdentityKind},
    integrity::{domain_separated_sha256, verify_domain},
};

#[derive(Debug, Default)]
pub struct Phase1Adapter;

fn input_digest(input: &BehavioralInput) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&input.scenario_data).unwrap())
    )
}
fn observation(input: &BehavioralInput, facts: serde_json::Value) -> ContractResult {
    Ok(ObservedBehavior {
        input_digest: input_digest(input),
        facts,
    })
}

impl FoundationalContracts for Phase1Adapter {
    fn canonical_representation(&self, input: &BehavioralInput) -> ContractResult {
        let left = canonical_cbor(&input.scenario_data["left"])
            .map_err(|_| ContractError::ExpectedFailNotImplemented)?;
        let right = canonical_cbor(&input.scenario_data["right"])
            .map_err(|_| ContractError::ExpectedFailNotImplemented)?;
        observation(
            input,
            json!({"left_canonical":left,"right_canonical":right}),
        )
    }

    fn bind_identity_version_provenance(&self, input: &BehavioralInput) -> ContractResult {
        let data = &input.scenario_data;
        let identity = CanonicalIdentity::new(
            data["bound_domain"].as_str().unwrap(),
            &input.subject_type,
            IdentityKind::Version,
            data["raw_id"].as_str().unwrap(),
            data["version_hash"].as_str().unwrap(),
            &input.fixture_id,
        )
        .map_err(|_| ContractError::ExpectedFailNotImplemented)?;
        let accepted = identity
            .bind_presented_domain(data["presented_domain"].as_str().unwrap())
            .is_ok();
        observation(
            input,
            json!({"accepted":accepted,"bound_domain":identity.domain,"provenance_bound":!identity.provenance_ref.is_empty()}),
        )
    }

    fn verify_cryptographic_integrity(&self, input: &BehavioralInput) -> ContractResult {
        let data = &input.scenario_data;
        let proof_domain = data["proof_domain"].as_str().unwrap();
        let required = data["required_domain"].as_str().unwrap();
        let payload = canonical_cbor(&json!({"payload":data["payload"].clone()}))
            .map_err(|_| ContractError::ExpectedFailNotImplemented)?;
        let proof_hash = domain_separated_sha256(proof_domain, "v1", &payload);
        let required_hash = domain_separated_sha256(required, "v1", &payload);
        let domain_mismatch = !verify_domain(proof_domain, required) && proof_hash != required_hash;
        observation(
            input,
            json!({"accepted":!domain_mismatch,"domain_mismatch":domain_mismatch,"checked_domain":required}),
        )
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
    fn certify_empirically(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
}
