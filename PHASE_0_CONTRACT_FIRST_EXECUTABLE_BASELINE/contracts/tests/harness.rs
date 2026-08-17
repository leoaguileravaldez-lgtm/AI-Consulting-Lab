use std::{collections::HashSet, fs, path::PathBuf};

use ai_consulting_lab_phase0_contracts::CONFORMANCE_TEST_IDS;
use serde_json::Value;
use sha2::{Digest, Sha256};

const CERTIFIED: &str = "ccddb2f3ec0c21e7415e5c4866eda9b8a532a799";
const CONTRACT_FIELDS: [&str; 20] = [
    "contract_id",
    "certified_commit",
    "certified_source_locators",
    "requirement_identity",
    "subject_type",
    "preconditions",
    "operation",
    "authority_conditions",
    "currentness_conditions",
    "expected_valid_behavior",
    "prohibited_behavior",
    "assertion_semantics",
    "phase0_classification_rule",
    "expected_phase0_state",
    "future_pass_condition",
    "failure_classification",
    "test_id",
    "fixture_id",
    "interface",
    "independent_validation_requirement",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
fn repository_root() -> PathBuf {
    root().parent().unwrap().to_path_buf()
}
fn json(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(root().join(path)).expect("file must load"))
        .expect("valid JSON")
}
fn nonempty(value: &Value) -> bool {
    value.as_str().is_some_and(|s| !s.is_empty())
}
fn outcome(value: &Value) -> bool {
    matches!(value["disposition"].as_str(), Some("ACCEPT" | "DENY")) && nonempty(&value["evidence"])
}
fn pointer_identity(value: &Value) -> Option<&str> {
    value.get("id").and_then(Value::as_str)
}

fn validate(manifest: &Value, fixtures: &Value) -> Result<(), String> {
    if manifest["manifest_version"] != "2.0.0"
        || manifest["certified_commit"] != CERTIFIED
        || manifest["workspace_kind"] != "NON_LAYER_PHASE_0"
    {
        return Err("root binding".into());
    }
    if manifest["independent_validation"]["id"] != "IV_PHASE0_BLACK_BOX_V1"
        || !nonempty(&manifest["independent_validation"]["requirement"])
    {
        return Err("independent validation linkage".into());
    }
    let contracts = manifest["contracts"].as_array().ok_or("contracts array")?;
    let cases = fixtures["cases"].as_array().ok_or("fixture cases")?;
    if contracts.is_empty()
        || contracts.len() != cases.len()
        || contracts.len() != CONFORMANCE_TEST_IDS.len()
    {
        return Err("orphan cardinality".into());
    }
    let mut contract_ids = HashSet::new();
    let mut test_ids = HashSet::new();
    let mut fixture_ids = HashSet::new();
    let conformance_source = fs::read_to_string(root().join("contracts/tests/conformance.rs"))
        .map_err(|e| e.to_string())?;
    let interface_source =
        fs::read_to_string(root().join("contracts/src/lib.rs")).map_err(|e| e.to_string())?;
    for contract in contracts {
        let object = contract.as_object().ok_or("contract object")?;
        if object.len() != CONTRACT_FIELDS.len()
            || CONTRACT_FIELDS.iter().any(|f| !object.contains_key(*f))
        {
            return Err("incomplete or additional contract field".into());
        }
        if contract["certified_commit"] != CERTIFIED
            || contract["expected_phase0_state"] != "EXPECTED_FAIL_NOT_IMPLEMENTED"
            || contract["independent_validation_requirement"] != "IV_PHASE0_BLACK_BOX_V1"
        {
            return Err("contract binding".into());
        }
        for field in [
            "contract_id",
            "requirement_identity",
            "subject_type",
            "operation",
            "future_pass_condition",
            "failure_classification",
            "test_id",
            "fixture_id",
            "interface",
            "assertion_semantics",
            "phase0_classification_rule",
        ] {
            if !nonempty(&contract[field]) {
                return Err(format!("missing {field}"));
            }
        }
        for field in [
            "preconditions",
            "authority_conditions",
            "currentness_conditions",
        ] {
            if contract[field]
                .as_array()
                .is_none_or(|a| a.is_empty() || a.iter().any(|v| !nonempty(v)))
            {
                return Err(format!("invalid {field}"));
            }
        }
        if !outcome(&contract["expected_valid_behavior"])
            || !outcome(&contract["prohibited_behavior"])
            || contract["expected_valid_behavior"] == contract["prohibited_behavior"]
        {
            return Err("behavioral oracle".into());
        }
        let cid = contract["contract_id"].as_str().unwrap();
        let tid = contract["test_id"].as_str().unwrap();
        let fid = contract["fixture_id"].as_str().unwrap();
        if !contract_ids.insert(cid) || !test_ids.insert(tid) || !fixture_ids.insert(fid) {
            return Err("duplicate identity".into());
        }
        if !CONFORMANCE_TEST_IDS.contains(&tid) || !conformance_source.contains(tid) {
            return Err("orphan contract/test".into());
        }
        let interface = contract["interface"].as_str().unwrap();
        if !interface_source.contains(&format!("fn {interface}("))
            || !conformance_source.contains(interface)
        {
            return Err("orphan interface/test".into());
        }
        let case = cases
            .iter()
            .find(|v| v["fixture_id"] == fid && v["test_id"] == tid)
            .ok_or("orphan fixture")?;
        if case["scenario_data"]
            .as_object()
            .is_none_or(|o| o.is_empty())
        {
            return Err("missing concrete scenario data".into());
        }
        for (contract_field, fixture_field) in
            [("subject_type", "subject_type"), ("operation", "operation")]
        {
            if contract[contract_field] != case[fixture_field] {
                return Err("contract fixture mismatch".into());
            }
        }
        if contract["preconditions"][0] != case["precondition"]
            || contract["authority_conditions"][0] != case["authority_condition"]
            || contract["currentness_conditions"][0] != case["currentness_condition"]
            || contract["expected_valid_behavior"] != case["expected"]
            || contract["prohibited_behavior"] != case["false_pass"]
        {
            return Err("behavior fixture mismatch".into());
        }
        let locators = contract["certified_source_locators"]
            .as_array()
            .ok_or("source locators")?;
        if locators.is_empty() {
            return Err("missing source locator".into());
        }
        for locator in locators {
            let layer = locator["layer"].as_u64().ok_or("source layer")?;
            if layer > 19 || layer == 20 {
                return Err("future source".into());
            }
            let path = locator["path"].as_str().ok_or("source path")?;
            if path.starts_with("20_") || path.to_ascii_lowercase().contains("future") {
                return Err("future source path".into());
            }
            let bytes = fs::read(repository_root().join(path)).map_err(|_| "source absent")?;
            if format!("{:x}", Sha256::digest(&bytes))
                != locator["sha256"].as_str().ok_or("source hash")?
            {
                return Err("source hash mismatch".into());
            }
            let source: Value = serde_json::from_slice(&bytes).map_err(|_| "source JSON")?;
            let pointer = locator["json_pointer"].as_str().ok_or("JSON pointer")?;
            let resolved = source.pointer(pointer).ok_or("unresolved JSON pointer")?;
            if pointer_identity(resolved) != locator["identity"].as_str() {
                return Err("source identity mismatch".into());
            }
        }
    }
    let fixture_test_ids: HashSet<_> = cases.iter().filter_map(|v| v["test_id"].as_str()).collect();
    if fixture_test_ids != test_ids
        || CONFORMANCE_TEST_IDS.iter().copied().collect::<HashSet<_>>() != test_ids
    {
        return Err("orphan set mismatch".into());
    }
    Ok(())
}

#[test]
fn harness_fixture_loads_and_is_deterministic() {
    let fixture = json("fixtures/synthetic_contract_case.json");
    assert_eq!(fixture["synthetic"], true);
    assert_eq!(fixture["deterministic_seed"], 0);
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 10);
}

#[test]
fn harness_semantic_traceability_validates() {
    validate(
        &json("traceability/manifest.json"),
        &json("fixtures/synthetic_contract_case.json"),
    )
    .expect("complete semantic chains");
}

#[test]
fn harness_validator_rejects_all_nine_fixed_invalid_chain_classes() {
    let fixtures = json("fixtures/synthetic_contract_case.json");
    let manifest = json("traceability/manifest.json");
    let mut incomplete = manifest.clone();
    incomplete["contracts"][0]
        .as_object_mut()
        .unwrap()
        .remove("requirement_identity");
    assert!(
        validate(&incomplete, &fixtures).is_err(),
        "incomplete chain"
    );
    let mut duplicate_contract = manifest.clone();
    duplicate_contract["contracts"][1]["contract_id"] =
        duplicate_contract["contracts"][0]["contract_id"].clone();
    assert!(
        validate(&duplicate_contract, &fixtures).is_err(),
        "duplicate contract ID"
    );
    let mut duplicate_test = manifest.clone();
    duplicate_test["contracts"][1]["test_id"] = duplicate_test["contracts"][0]["test_id"].clone();
    assert!(
        validate(&duplicate_test, &fixtures).is_err(),
        "duplicate test ID"
    );
    let mut future = manifest.clone();
    future["contracts"][0]["certified_source_locators"][0]["layer"] = 20.into();
    assert!(
        validate(&future, &fixtures).is_err(),
        "Layer 20/future source"
    );
    let mut orphan = manifest.clone();
    orphan["contracts"][0]["fixture_id"] = "FX_ORPHAN".into();
    assert!(validate(&orphan, &fixtures).is_err(), "orphan fixture");
    let mut missing_source = manifest.clone();
    missing_source["contracts"][0]["certified_source_locators"] = serde_json::json!([]);
    assert!(
        validate(&missing_source, &fixtures).is_err(),
        "missing source locator"
    );
    let mut missing_expectation = manifest.clone();
    missing_expectation["contracts"][0]
        .as_object_mut()
        .unwrap()
        .remove("expected_valid_behavior");
    assert!(
        validate(&missing_expectation, &fixtures).is_err(),
        "missing behavioral expectation"
    );
    let mut missing_future_pass = manifest.clone();
    missing_future_pass["contracts"][0]
        .as_object_mut()
        .unwrap()
        .remove("future_pass_condition");
    assert!(
        validate(&missing_future_pass, &fixtures).is_err(),
        "missing future PASS condition"
    );
    let mut missing_prohibited = manifest.clone();
    missing_prohibited["contracts"][0]
        .as_object_mut()
        .unwrap()
        .remove("prohibited_behavior");
    assert!(
        validate(&missing_prohibited, &fixtures).is_err(),
        "missing prohibited behavior"
    );
}

#[test]
fn harness_schema_declares_semantic_enforcement() {
    let schema = json("traceability/manifest.schema.json");
    assert_eq!(schema["properties"]["certified_commit"]["const"], CERTIFIED);
    let required = schema["$defs"]["contract"]["required"].as_array().unwrap();
    for field in CONTRACT_FIELDS {
        assert!(
            required.iter().any(|v| v == field),
            "schema missing {field}"
        );
    }
    assert!(
        schema["x-cross-record-constraints"]
            .as_array()
            .is_some_and(|v| v.len() >= 5)
    );
}

#[test]
fn harness_test_identifiers_are_unique_and_reconciled() {
    assert_eq!(
        CONFORMANCE_TEST_IDS
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .len(),
        CONFORMANCE_TEST_IDS.len()
    );
}

#[test]
fn harness_prohibited_artifacts_absent() {
    let prohibited = [
        "layer20",
        "layer_20",
        "database",
        "connector",
        "credential",
        "secret",
        "deployment",
        "worker",
        "queue",
        "scheduler",
        "webhook",
        "daemon",
    ];
    for entry in fs::read_dir(root()).unwrap() {
        let name = entry.unwrap().file_name().to_string_lossy().to_lowercase();
        assert!(
            !prohibited.iter().any(|p| name.contains(p)),
            "prohibited artifact: {name}"
        );
    }
}
