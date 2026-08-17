use std::{fs, path::PathBuf};

use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, FoundationalContracts, ObservedBehavior,
};
use ai_consulting_lab_phase1_foundational_runtime::{
    Phase1Adapter,
    canonical::canonical_cbor,
    identity::{CanonicalIdentity, IdentityError, IdentityKind},
    integrity::domain_separated_sha256,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SELECTED: [&str; 3] = [
    "P0_CANONICAL_REPRESENTATION",
    "P0_IDENTITY_VERSION_PROVENANCE",
    "P0_CRYPTOGRAPHIC_INTEGRITY",
];

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
fn frozen_json(path: &str) -> Value {
    serde_json::from_str(
        &fs::read_to_string(
            repository_root()
                .join("PHASE_0_CONTRACT_FIRST_EXECUTABLE_BASELINE")
                .join(path),
        )
        .unwrap(),
    )
    .unwrap()
}
fn input(test_id: &str) -> BehavioralInput {
    let document = frozen_json("fixtures/synthetic_contract_case.json");
    let case = document["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["test_id"] == test_id)
        .unwrap();
    BehavioralInput {
        fixture_id: case["fixture_id"].as_str().unwrap().into(),
        subject_type: case["subject_type"].as_str().unwrap().into(),
        operation: case["operation"].as_str().unwrap().into(),
        precondition: case["precondition"].as_str().unwrap().into(),
        authority_condition: case["authority_condition"].as_str().unwrap().into(),
        currentness_condition: case["currentness_condition"].as_str().unwrap().into(),
        candidate_behavior: case["candidate_behavior"].as_str().unwrap().into(),
        scenario_data: case["scenario_data"].clone(),
    }
}
fn digest(input: &BehavioralInput) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&input.scenario_data).unwrap())
    )
}
fn satisfies(test_id: &str, input: &BehavioralInput, actual: &ObservedBehavior) -> bool {
    if actual.input_digest != digest(input) {
        return false;
    }
    let f = &actual.facts;
    match test_id {
        "P0_CANONICAL_REPRESENTATION" => {
            f["left_canonical"]
                .as_array()
                .is_some_and(|v| !v.is_empty())
                && f["left_canonical"] == f["right_canonical"]
        }
        "P0_IDENTITY_VERSION_PROVENANCE" => {
            f["accepted"] == false
                && f["bound_domain"] == input.scenario_data["bound_domain"]
                && f["provenance_bound"] == true
        }
        "P0_CRYPTOGRAPHIC_INTEGRITY" => {
            f["accepted"] == false
                && f["domain_mismatch"] == true
                && f["checked_domain"] == input.scenario_data["required_domain"]
        }
        _ => false,
    }
}
fn invoke(test_id: &str, input: &BehavioralInput) -> Result<ObservedBehavior, ContractError> {
    match test_id {
        "P0_CANONICAL_REPRESENTATION" => Phase1Adapter.canonical_representation(input),
        "P0_IDENTITY_VERSION_PROVENANCE" => Phase1Adapter.bind_identity_version_provenance(input),
        "P0_CRYPTOGRAPHIC_INTEGRITY" => Phase1Adapter.verify_cryptographic_integrity(input),
        _ => unreachable!(),
    }
}
fn materially_bad(test_id: &str, input: &BehavioralInput) -> ObservedBehavior {
    let facts = match test_id {
        "P0_CANONICAL_REPRESENTATION" => json!({"left_canonical":[1,2],"right_canonical":[2,1]}),
        "P0_IDENTITY_VERSION_PROVENANCE" => {
            json!({"accepted":true,"bound_domain":"client-B","provenance_bound":false})
        }
        "P0_CRYPTOGRAPHIC_INTEGRITY" => {
            json!({"accepted":true,"domain_mismatch":false,"checked_domain":""})
        }
        _ => unreachable!(),
    };
    ObservedBehavior {
        input_digest: digest(input),
        facts,
    }
}

#[test]
fn frozen_bindings_reconcile_exactly() {
    let phase0 = frozen_json("traceability/manifest.json");
    let phase1: Value = serde_json::from_str(
        &fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bindings.json"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        phase0["certified_commit"],
        "ccddb2f3ec0c21e7415e5c4866eda9b8a532a799"
    );
    assert_eq!(
        phase1["frozen_phase0_commit"],
        "8b0b5f342fcf95b34dc26051edb09a8394267a8d"
    );
    let phase0_ids: Vec<_> = phase0["contracts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["contract_id"].clone())
        .collect();
    let phase1_ids: Vec<_> = phase1["dependency_order"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["contract_id"].clone())
        .collect();
    assert_eq!(phase0_ids, phase1_ids);
    for binding in &phase1["dependency_order"].as_array().unwrap()[..3] {
        let frozen = phase0
            .pointer(binding["phase0_manifest_pointer"].as_str().unwrap())
            .unwrap();
        assert_eq!(frozen["contract_id"], binding["contract_id"]);
        assert_eq!(
            frozen["requirement_identity"],
            binding["certified_requirement"]
        );
        assert!(
            !frozen["certified_source_locators"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        assert!(
            binding["frozen_predicate"]
                .as_str()
                .unwrap()
                .starts_with("satisfies/P0_")
        );
        assert_eq!(
            binding["empirical_test"],
            "selected_contracts_are_genuine_empirical_passes"
        );
    }
}

#[test]
fn selected_contracts_are_genuine_empirical_passes() {
    for id in SELECTED {
        let base = input(id);
        let actual = invoke(id, &base).expect("selected implementation exists");
        assert!(satisfies(id, &base, &actual));
        assert!(!satisfies(id, &base, &materially_bad(id, &base)));
        assert!(!satisfies(
            id,
            &base,
            &ObservedBehavior {
                input_digest: "constant".into(),
                facts: json!({})
            }
        ));
        assert!(!satisfies(
            id,
            &base,
            &ObservedBehavior {
                input_digest: digest(&base),
                facts: base.scenario_data.clone()
            }
        ));
        let hardcoded = actual;
        let mut varied = base.clone();
        varied.scenario_data["independent_probe_nonce"] = 1.into();
        assert!(!satisfies(id, &varied, &hardcoded));
        let varied_actual = invoke(id, &varied).expect("varied scenario executes");
        assert!(satisfies(id, &varied, &varied_actual));
    }
}

#[test]
fn dependent_contracts_remain_explicitly_absent() {
    let input = input("P0_TRANSACTIONAL_PERSISTENCE");
    let results = [
        Phase1Adapter.persist_transactionally(&input),
        Phase1Adapter.enforce_global_uniqueness(&input),
        Phase1Adapter.propagate_revocation_and_freshness(&input),
        Phase1Adapter.attest_runtime_isolation(&input),
        Phase1Adapter.recover_and_reconcile(&input),
        Phase1Adapter.emit_audit_evidence(&input),
        Phase1Adapter.certify_empirically(&input),
    ];
    assert!(
        results
            .into_iter()
            .all(|r| r == Err(ContractError::ExpectedFailNotImplemented))
    );
}

#[test]
fn canonical_serialization_is_stable_and_order_independent() {
    let a = json!({"z":null,"a":[1,true,"é"]});
    let b: Value = serde_json::from_str("{\"a\":[1,true,\"é\"],\"z\":null}").unwrap();
    let first = canonical_cbor(&a).unwrap();
    assert_eq!(first, canonical_cbor(&a).unwrap());
    assert_eq!(first, canonical_cbor(&b).unwrap());
}

#[test]
fn identity_rejects_cross_domain_substitution() {
    let identity = CanonicalIdentity::new(
        "client-A",
        "Material",
        IdentityKind::Version,
        "synthetic-1",
        "v1",
        "prov-1",
    )
    .unwrap();
    assert_eq!(
        identity.bind_presented_domain("client-B"),
        Err(IdentityError::DomainSubstitution)
    );
    assert!(identity.bind_presented_domain("client-A").is_ok());
}

#[test]
fn integrity_hash_binds_domain_version_and_canonical_payload() {
    let payload = canonical_cbor(&json!({"a":1,"b":2})).unwrap();
    let base = domain_separated_sha256("authority-record", "v1", &payload);
    assert_ne!(base, domain_separated_sha256("audit-event", "v1", &payload));
    assert_ne!(
        base,
        domain_separated_sha256("authority-record", "v2", &payload)
    );
    assert_eq!(
        base,
        domain_separated_sha256(
            "authority-record",
            "v1",
            &canonical_cbor(&json!({"b":2,"a":1})).unwrap()
        )
    );
}
