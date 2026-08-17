use std::{fs, path::PathBuf};

use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractBaseline, ContractError, FoundationalContracts, ObservedBehavior,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn fixture(test_id: &str) -> BehavioralInput {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("fixtures/synthetic_contract_case.json");
    let document: Value = serde_json::from_str(&fs::read_to_string(path).expect("fixture loads"))
        .expect("fixture JSON");
    let case = document["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["test_id"] == test_id)
        .expect("mapped fixture");
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
fn observed(input: &BehavioralInput, facts: Value) -> ObservedBehavior {
    ObservedBehavior {
        input_digest: digest(input),
        facts,
    }
}

/// Test-only deterministic reference adapter. It has no persistence, I/O, authority, or production use.
fn known_good(test_id: &str, input: &BehavioralInput) -> ObservedBehavior {
    let facts = match test_id {
        "P0_CANONICAL_REPRESENTATION" => {
            json!({"left_canonical":[161,97,1,97,98,2],"right_canonical":[161,97,1,97,98,2]})
        }
        "P0_IDENTITY_VERSION_PROVENANCE" => {
            json!({"accepted":false,"bound_domain":"client-A","provenance_bound":true})
        }
        "P0_CRYPTOGRAPHIC_INTEGRITY" => {
            json!({"accepted":false,"domain_mismatch":true,"checked_domain":"authority-record"})
        }
        "P0_TRANSACTIONAL_PERSISTENCE" => {
            json!({"accepted":false,"state_committed":false,"audit_committed":false,"provenance_committed":false})
        }
        "P0_GLOBAL_UNIQUENESS_CONCURRENCY" => {
            json!({"accepted":false,"stored_sequence":8,"stored_fence":12,"successor_created":false})
        }
        "P0_REVOCATION_FRESHNESS" => {
            json!({"accepted":false,"evaluated_revocation_epoch":5,"cache_invalidated":true})
        }
        "P0_RUNTIME_ISOLATION" => {
            json!({"accepted":false,"boundary_crossed":false,"effective_actor":"MACHINE"})
        }
        "P0_RECOVERY_RECONCILIATION" => {
            json!({"retry_allowed":false,"predecessor_preserved":true,"reconciliation_required":true})
        }
        "P0_AUDIT_EVIDENCE" => {
            json!({"accepted":false,"journal_mutated":false,"tail_sequence":31,"predecessor_valid":false})
        }
        "P0_EMPIRICAL_CERTIFICATION" => {
            json!({"accepted":false,"material_failure_preserved":true,"deployment_authority_created":false})
        }
        _ => unreachable!(),
    };
    observed(input, facts)
}

/// Test-only materially wrong mutant for the certified failure family.
fn known_bad(test_id: &str, input: &BehavioralInput) -> ObservedBehavior {
    let facts = match test_id {
        "P0_CANONICAL_REPRESENTATION" => {
            json!({"left_canonical":[97,1,98,2],"right_canonical":[98,2,97,1]})
        }
        "P0_IDENTITY_VERSION_PROVENANCE" => {
            json!({"accepted":true,"bound_domain":"client-B","provenance_bound":false})
        }
        "P0_CRYPTOGRAPHIC_INTEGRITY" => {
            json!({"accepted":true,"domain_mismatch":false,"checked_domain":""})
        }
        "P0_TRANSACTIONAL_PERSISTENCE" => {
            json!({"accepted":true,"state_committed":true,"audit_committed":false,"provenance_committed":false})
        }
        "P0_GLOBAL_UNIQUENESS_CONCURRENCY" => {
            json!({"accepted":true,"stored_sequence":7,"stored_fence":11,"successor_created":true})
        }
        "P0_REVOCATION_FRESHNESS" => {
            json!({"accepted":true,"evaluated_revocation_epoch":4,"cache_invalidated":false})
        }
        "P0_RUNTIME_ISOLATION" => {
            json!({"accepted":true,"boundary_crossed":true,"effective_actor":"HUMAN"})
        }
        "P0_RECOVERY_RECONCILIATION" => {
            json!({"retry_allowed":true,"predecessor_preserved":false,"reconciliation_required":false})
        }
        "P0_AUDIT_EVIDENCE" => {
            json!({"accepted":true,"journal_mutated":true,"tail_sequence":32,"predecessor_valid":false})
        }
        "P0_EMPIRICAL_CERTIFICATION" => {
            json!({"accepted":true,"material_failure_preserved":false,"deployment_authority_created":true})
        }
        _ => unreachable!(),
    };
    observed(input, facts)
}

/// Executable future-PASS predicate. It inspects input-bound operation facts, never labels.
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
        "P0_TRANSACTIONAL_PERSISTENCE" => {
            f["accepted"] == false
                && f["state_committed"] == false
                && f["audit_committed"] == false
                && f["provenance_committed"] == false
        }
        "P0_GLOBAL_UNIQUENESS_CONCURRENCY" => {
            f["accepted"] == false
                && f["stored_sequence"] == input.scenario_data["expected_sequence"]
                && f["stored_fence"] == input.scenario_data["current_fence"]
                && f["successor_created"] == false
        }
        "P0_REVOCATION_FRESHNESS" => {
            f["accepted"] == false
                && f["evaluated_revocation_epoch"] == input.scenario_data["revocation_epoch"]
                && f["cache_invalidated"] == true
        }
        "P0_RUNTIME_ISOLATION" => {
            f["accepted"] == false
                && f["boundary_crossed"] == false
                && f["effective_actor"] == input.scenario_data["actor_type"]
        }
        "P0_RECOVERY_RECONCILIATION" => {
            f["retry_allowed"] == false
                && f["predecessor_preserved"] == true
                && f["reconciliation_required"] == true
        }
        "P0_AUDIT_EVIDENCE" => {
            f["accepted"] == false
                && f["journal_mutated"] == false
                && f["tail_sequence"] == input.scenario_data["tail_sequence"]
                && f["predecessor_valid"] == false
        }
        "P0_EMPIRICAL_CERTIFICATION" => {
            f["accepted"] == false
                && f["material_failure_preserved"] == true
                && f["deployment_authority_created"] == false
        }
        _ => false,
    }
}

fn prove_effectiveness(test_id: &str, input: &BehavioralInput) {
    assert!(
        satisfies(test_id, input, &known_good(test_id, input)),
        "known-good vector rejected"
    );
    assert!(
        !satisfies(test_id, input, &known_bad(test_id, input)),
        "known-bad mutant accepted"
    );
    let constant = ObservedBehavior {
        input_digest: "constant".into(),
        facts: json!({}),
    };
    assert!(
        !satisfies(test_id, input, &constant),
        "constant/label-only false pass accepted"
    );
    let echo = ObservedBehavior {
        input_digest: digest(input),
        facts: input.scenario_data.clone(),
    };
    assert!(
        !satisfies(test_id, input, &echo),
        "fixture-metadata echo false pass accepted"
    );
    let hardcoded_fixture_result = known_good(test_id, input);
    let mut varied = input.clone();
    varied.scenario_data["independent_probe_nonce"] = 1.into();
    assert!(
        !satisfies(test_id, &varied, &hardcoded_fixture_result),
        "hard-coded fixture result accepted for a varied deterministic scenario"
    );
}

macro_rules! behavioral_red {
    ($name:ident, $id:literal, $call:ident) => {
        #[test]
        fn $name() {
            let input = fixture($id);
            prove_effectiveness($id, &input);
            match ContractBaseline.$call(&input) {
                Err(ContractError::ExpectedFailNotImplemented) => {
                    panic!("{}: EXPECTED_FAIL_NOT_IMPLEMENTED", $id)
                }
                Ok(actual) => assert!(
                    satisfies($id, &input, &actual),
                    "{}: BEHAVIORAL_CONTRACT_FAIL",
                    $id
                ),
            }
        }
    };
}

behavioral_red!(
    p0_canonical_representation,
    "P0_CANONICAL_REPRESENTATION",
    canonical_representation
);
behavioral_red!(
    p0_identity_version_provenance,
    "P0_IDENTITY_VERSION_PROVENANCE",
    bind_identity_version_provenance
);
behavioral_red!(
    p0_cryptographic_integrity,
    "P0_CRYPTOGRAPHIC_INTEGRITY",
    verify_cryptographic_integrity
);
behavioral_red!(
    p0_transactional_persistence,
    "P0_TRANSACTIONAL_PERSISTENCE",
    persist_transactionally
);
behavioral_red!(
    p0_global_uniqueness_concurrency,
    "P0_GLOBAL_UNIQUENESS_CONCURRENCY",
    enforce_global_uniqueness
);
behavioral_red!(
    p0_revocation_freshness,
    "P0_REVOCATION_FRESHNESS",
    propagate_revocation_and_freshness
);
behavioral_red!(
    p0_runtime_isolation,
    "P0_RUNTIME_ISOLATION",
    attest_runtime_isolation
);
behavioral_red!(
    p0_recovery_reconciliation,
    "P0_RECOVERY_RECONCILIATION",
    recover_and_reconcile
);
behavioral_red!(p0_audit_evidence, "P0_AUDIT_EVIDENCE", emit_audit_evidence);
behavioral_red!(
    p0_empirical_certification,
    "P0_EMPIRICAL_CERTIFICATION",
    certify_empirically
);
