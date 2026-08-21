use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::HashSet, env, fs, path::Path, process};

const COMMIT: &str = "8364cc3570cca11692e638b6cf3022a47fa9e752";
const SOURCE_HASH: &str = "295441a5f83f0ba6626c212a8950c8c0214a903709ade8617eddcdcca397051c";
const CONTRACT: &str = "BC_EMPIRICAL_CERTIFICATION";
const EVIDENCE: [(&str, &str, &str); 7] = [
    (
        "foundational-runtime",
        "PHASE_1_FOUNDATIONAL_RUNTIME/bindings.json",
        "750f1ea83d88d77f14329cf007a4e4034420019b43e6f31655171f24261fe3b4",
    ),
    (
        "transactional-persistence",
        "PHASE_1_TRANSACTIONAL_PERSISTENCE/LOCAL_IMPLEMENTATION_EVIDENCE.json",
        "26d6b6121c556998618ce0e281b555008562068ab87e123410c6f3d54c82be45",
    ),
    (
        "global-uniqueness-concurrency",
        "PHASE_1_GLOBAL_UNIQUENESS_CONCURRENCY/LOCAL_IMPLEMENTATION_EVIDENCE.json",
        "ed1c20d3327d98257e48a30d518fd470c0db75814b556326e1b65b5aa35db071",
    ),
    (
        "revocation-freshness",
        "PHASE_1_REVOCATION_FRESHNESS/LOCAL_IMPLEMENTATION_EVIDENCE.json",
        "bc85361f32741bb27cf3e3844930b8e548dbf708a13f8c206255e1174b719bec",
    ),
    (
        "runtime-isolation",
        "PHASE_1_RUNTIME_ISOLATION/LOCAL_IMPLEMENTATION_EVIDENCE.json",
        "0f1f7cb8833e5cd40deabf07cdf3e5be1690a13ad24c0ba451ff5edb66af4d8f",
    ),
    (
        "recovery-reconciliation",
        "PHASE_1_RECOVERY_RECONCILIATION/LOCAL_IMPLEMENTATION_EVIDENCE.json",
        "5846f5adfcbcedf004625872bc8632705c6f3c4aca166ade15c3a45b0862d7fd",
    ),
    (
        "audit-evidence",
        "PHASE_1_AUDIT_EVIDENCE/LOCAL_IMPLEMENTATION_EVIDENCE.json",
        "05752754c306d83cc7c741fece713c99e69d523d9e2c0e7edbb80ce690e05c3b",
    ),
];

fn sha(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}
fn deny(verdict: &str, reason: &str) -> Value {
    json!({"verdict":verdict,"reason":reason,"accepted":false,"material_failure_preserved":false,"deployment_authority_created":false,"certification_authority_laundering":0,"execution_authority_leakage":0,"retry_authority_leakage":0,"recovery_authority_leakage":0,"layer19_operational_authority_leakage":0,"deployment_authority_leakage":0})
}
fn text<'a>(v: &'a Value, k: &str) -> Option<&'a str> {
    v.get(k)?.as_str()
}

fn validate(root: &Path, package_path: &Path) -> Value {
    let raw = match fs::read(package_path) {
        Ok(v) => v,
        Err(_) => return deny("INFRASTRUCTURE_FAILURE", "PACKAGE_UNREADABLE"),
    };
    let p: Value = match serde_json::from_slice(&raw) {
        Ok(v) => v,
        Err(_) => return deny("INVALID_EVIDENCE", "PACKAGE_PARSE"),
    };
    let source = match fs::read(
        root.join("16_OPERATIONAL_REALIZATION_CONFORMANCE_ARCHITECTURE/CANONICAL_MODEL.json"),
    ) {
        Ok(v) => v,
        Err(_) => return deny("INFRASTRUCTURE_FAILURE", "NORMATIVE_SOURCE_UNREADABLE"),
    };
    if sha(&source) != SOURCE_HASH {
        return deny("INVALID_EVIDENCE", "NORMATIVE_SOURCE_HASH_MISMATCH");
    }
    let model: Value = match serde_json::from_slice(&source) {
        Ok(v) => v,
        Err(_) => return deny("VALIDATOR_FAILURE", "NORMATIVE_SOURCE_PARSE"),
    };
    if model["object_types"][12]["id"] != "FUTURE_EMPIRICAL_CERTIFICATION_CONTRACT"
        || model["invariants"][18]["id"] != "S"
    {
        return deny("INVALID_EVIDENCE", "FROZEN_CONTRACT_MISMATCH");
    }
    if text(&p, "repository") != Some("AI-Consulting-Lab")
        || text(&p, "candidate_commit") != Some(COMMIT)
        || text(&p, "contract_id") != Some(CONTRACT)
        || text(&p, "context_id") != Some("phase1-local-evidence")
        || p["generation"] != 1
    {
        return deny(
            "INVALID_EVIDENCE",
            "CANDIDATE_PROVENANCE_OR_VERSION_MISMATCH",
        );
    }
    if !p["fixture_binding"].is_null() {
        let fixture_raw = match fs::read(root.join(
            "PHASE_0_CONTRACT_FIRST_EXECUTABLE_BASELINE/fixtures/synthetic_contract_case.json",
        )) {
            Ok(v) => v,
            Err(_) => return deny("INFRASTRUCTURE_FAILURE", "FROZEN_FIXTURE_UNREADABLE"),
        };
        let fixture: Value = match serde_json::from_slice(&fixture_raw) {
            Ok(v) => v,
            Err(_) => return deny("VALIDATOR_FAILURE", "FROZEN_FIXTURE_PARSE"),
        };
        let Some(case) = fixture["cases"].as_array().and_then(|cases| {
            cases
                .iter()
                .find(|c| c["test_id"] == "P0_EMPIRICAL_CERTIFICATION")
        }) else {
            return deny("INVALID_EVIDENCE", "FROZEN_FIXTURE_MISSING");
        };
        if p["fixture_binding"]["candidate_hash"] != case["scenario_data"]["candidate_hash"]
            || p["fixture_binding"]["environment_hash"] != case["scenario_data"]["environment_hash"]
        {
            return deny("INVALID_EVIDENCE", "FROZEN_FIXTURE_IDENTITY_MISMATCH");
        }
    }
    let env = &p["environment"];
    let env_input = format!(
        "{}|{}|{}",
        env["rustc"].as_str().unwrap_or(""),
        env["sqlite"].as_str().unwrap_or(""),
        env["platform"].as_str().unwrap_or("")
    );
    if sha(env_input.as_bytes()) != env["manifest_hash"].as_str().unwrap_or("") {
        return deny("INVALID_EVIDENCE", "ENVIRONMENT_MANIFEST_MISMATCH");
    }
    let input_id =
        sha(format!("{}|{}|{}|{}", COMMIT, CONTRACT, "phase1-local-evidence", 1).as_bytes());
    if text(&p, "certification_input_id") != Some(&input_id) {
        return deny("INVALID_EVIDENCE", "CERTIFICATION_INPUT_ID_MISMATCH");
    }
    let ind = &p["independence"];
    if ind["producer_id"] == ind["validator_id"]
        || ind["separate_process"] != true
        || ind["imports_producer_implementation"] != false
        || ind["uses_producer_verdict"] != false
    {
        return deny("INVALID_EVIDENCE", "VALIDATOR_INDEPENDENCE_FAILURE");
    }
    let Some(items) = p["evidence"].as_array() else {
        return deny("INCOMPLETE_EVIDENCE", "EVIDENCE_MISSING");
    };
    if items.len() != EVIDENCE.len() {
        return deny("INCOMPLETE_EVIDENCE", "EVIDENCE_COUNT_MISMATCH");
    }
    let mut ids = HashSet::new();
    for (id, path, expected_hash) in EVIDENCE {
        let Some(item) = items.iter().find(|x| x["id"] == id) else {
            return deny("INCOMPLETE_EVIDENCE", "REQUIRED_EVIDENCE_MISSING");
        };
        if !ids.insert(id)
            || item["path"] != path
            || item["sha256"] != expected_hash
            || item["status"] != "EMPIRICAL_PASS"
        {
            return deny(
                "INVALID_EVIDENCE",
                "EVIDENCE_IDENTITY_OR_PROVENANCE_MISMATCH",
            );
        }
        let bytes = match fs::read(root.join(path)) {
            Ok(v) => v,
            Err(_) => return deny("INFRASTRUCTURE_FAILURE", "EVIDENCE_UNREADABLE"),
        };
        if sha(&bytes) != expected_hash {
            return deny("INVALID_EVIDENCE", "EVIDENCE_CONTENT_HASH_MISMATCH");
        }
    }
    let Some(results) = p["material_results"].as_array() else {
        return deny("INCOMPLETE_EVIDENCE", "MATERIAL_RESULTS_MISSING");
    };
    if results.is_empty() {
        return deny("INCOMPLETE_EVIDENCE", "MATERIAL_RESULTS_EMPTY");
    }
    let mut families = HashSet::new();
    let mut any_fail = false;
    for r in results {
        let Some(f) = r["family"].as_str() else {
            return deny("INVALID_EVIDENCE", "MATERIAL_FAMILY_INVALID");
        };
        if !families.insert(f) {
            return deny("INVALID_EVIDENCE", "DUPLICATE_EVIDENCE_IDENTITY");
        };
        match r["observed"].as_str() {
            Some("PASS") => {}
            Some("FAIL") => any_fail = true,
            _ => return deny("INVALID_EVIDENCE", "MATERIAL_RESULT_INVALID"),
        }
    }
    let Some(deployment_authority_created) =
        p["authority_observations"]["deployment_authority_created"].as_bool()
    else {
        return deny("INCOMPLETE_EVIDENCE", "AUTHORITY_OBSERVATION_MISSING");
    };
    if deployment_authority_created {
        return deny("INVALID_EVIDENCE", "DEPLOYMENT_AUTHORITY_CREATED");
    }
    let verdict = if any_fail {
        "CERTIFIED_FAIL"
    } else {
        "CERTIFIED_PASS"
    };
    let reason = if any_fail {
        "MATERIAL_FAILURE_NOT_AGGREGATED_AWAY"
    } else {
        "ALL_BOUND_MATERIAL_REQUIREMENTS_PASS"
    };
    let fingerprint = sha(&raw);
    json!({"verdict":verdict,"reason":reason,"candidate_commit":COMMIT,"contract_id":CONTRACT,"evidence_package_fingerprint":fingerprint,"validator_id":ind["validator_id"],"independent_expectation":if any_fail{"DENY"}else{"ACCEPT"},"accepted":!any_fail,"material_failure_preserved":any_fail,"deployment_authority_created":deployment_authority_created,"certification_authority_laundering":0,"execution_authority_leakage":0,"retry_authority_leakage":0,"recovery_authority_leakage":0,"layer19_operational_authority_leakage":0,"deployment_authority_leakage":0})
}
fn main() {
    let a: Vec<_> = env::args_os().collect();
    if a.len() != 3 {
        println!("{}", deny("VALIDATOR_FAILURE", "ARGUMENTS"));
        process::exit(2)
    }
    let v = validate(Path::new(&a[1]), Path::new(&a[2]));
    let pass = v["verdict"] == "CERTIFIED_PASS";
    println!("{}", v);
    if !pass {
        process::exit(10)
    }
}
