use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::HashSet, env, fs, path::Path, process};

const PARENT: &str = "6023c33ecafb093c55750d1e5e86ce77ba87cd57";
const SUBJECT: &str = "STAGE_I_INTEGRATION_KERNEL_ACCEPTANCE_V1";
const VERSION: &str = "working-tree-on-parent:6023c33ecafb093c55750d1e5e86ce77ba87cd57";
const SOURCE_PATHS:[&str;9]=["INTEGRATION_STAGE_1_KERNEL/Cargo.toml","INTEGRATION_STAGE_1_KERNEL/Cargo.lock","INTEGRATION_STAGE_1_KERNEL/bindings.json","INTEGRATION_STAGE_1_KERNEL/README.md","INTEGRATION_STAGE_1_KERNEL/src/lib.rs","INTEGRATION_STAGE_1_KERNEL/src/bin/integration_harness.rs","INTEGRATION_STAGE_1_KERNEL/src/bin/stage1_candidate_package.rs","INTEGRATION_STAGE_1_KERNEL/src/bin/stage1_independent_validator.rs","INTEGRATION_STAGE_1_KERNEL/tests/integration.rs"];
const PREDECESSORS: [(&str,&str,&str);8] = [
 ("foundational-runtime","PHASE_1_FOUNDATIONAL_RUNTIME/bindings.json","750f1ea83d88d77f14329cf007a4e4034420019b43e6f31655171f24261fe3b4"),
 ("transactional-persistence","PHASE_1_TRANSACTIONAL_PERSISTENCE/LOCAL_IMPLEMENTATION_EVIDENCE.json","26d6b6121c556998618ce0e281b555008562068ab87e123410c6f3d54c82be45"),
 ("global-uniqueness-concurrency","PHASE_1_GLOBAL_UNIQUENESS_CONCURRENCY/LOCAL_IMPLEMENTATION_EVIDENCE.json","ed1c20d3327d98257e48a30d518fd470c0db75814b556326e1b65b5aa35db071"),
 ("revocation-freshness","PHASE_1_REVOCATION_FRESHNESS/LOCAL_IMPLEMENTATION_EVIDENCE.json","bc85361f32741bb27cf3e3844930b8e548dbf708a13f8c206255e1174b719bec"),
 ("runtime-isolation","PHASE_1_RUNTIME_ISOLATION/LOCAL_IMPLEMENTATION_EVIDENCE.json","0f1f7cb8833e5cd40deabf07cdf3e5be1690a13ad24c0ba451ff5edb66af4d8f"),
 ("recovery-reconciliation","PHASE_1_RECOVERY_RECONCILIATION/LOCAL_IMPLEMENTATION_EVIDENCE.json","5846f5adfcbcedf004625872bc8632705c6f3c4aca166ade15c3a45b0862d7fd"),
 ("audit-evidence","PHASE_1_AUDIT_EVIDENCE/LOCAL_IMPLEMENTATION_EVIDENCE.json","05752754c306d83cc7c741fece713c99e69d523d9e2c0e7edbb80ce690e05c3b"),
 ("empirical-certification","PHASE_1_EMPIRICAL_CERTIFICATION/LOCAL_IMPLEMENTATION_EVIDENCE.json","99994260ccd7a0580ca3475aaef52758c2efcb6e069e20cf91642822403a71bf")];
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn deny(v:&str,r:&str)->Value{json!({"verdict":v,"reason":r,"accepted":false,"self_certification":0,"execution_leakage":0,"retry_leakage":0,"recovery_leakage":0,"deployment_leakage":0})}
fn validate(root:&Path,path:&Path)->Value{
 let raw=match fs::read(path){Ok(v)=>v,Err(_)=>return deny("INFRASTRUCTURE_FAILURE","PACKAGE_UNREADABLE")};
 let mut p:Value=match serde_json::from_slice(&raw){Ok(v)=>v,Err(_)=>return deny("INVALID_EVIDENCE","PACKAGE_PARSE")};
 let claimed=p.get("package_fingerprint").and_then(Value::as_str).unwrap_or("").to_owned();
 if let Some(o)=p.as_object_mut(){o.remove("package_fingerprint");}
 if claimed.is_empty()||sha(&serde_json::to_vec(&p).unwrap())!=claimed{return deny("INVALID_EVIDENCE","PACKAGE_FINGERPRINT_MISMATCH")}
 if p["repository"]!="AI-Consulting-Lab"||p["subject_id"]!=SUBJECT||p["candidate_version"]!=VERSION||p["certified_parent"]!=PARENT||p["context_id"]!="synthetic-client-A"||p["generation"]!=1{return deny("INVALID_EVIDENCE","CANDIDATE_PROVENANCE_OR_VERSION_MISMATCH")}
 let ind=&p["independence"];
 if ind["producer_id"]==ind["validator_id"]||ind["validator_id"]!="stage1-independent-validator-process-v1"||ind["separate_process"]!=true||ind["imports_producer_decision_logic"]!=false||ind["uses_producer_declared_pass"]!=false{return deny("INVALID_EVIDENCE","INDEPENDENCE_FAILURE")}
 let Some(manifest)=p["candidate_source_manifest"].as_array() else{return deny("INCOMPLETE_EVIDENCE","SOURCE_MANIFEST_MISSING")};
 if manifest.len()!=SOURCE_PATHS.len(){return deny("INCOMPLETE_EVIDENCE","SOURCE_MANIFEST_INCOMPLETE")}
 for path in SOURCE_PATHS {let Some(item)=manifest.iter().find(|x|x["path"]==path)else{return deny("INCOMPLETE_EVIDENCE","SOURCE_MANIFEST_PATH_MISSING")};let b=match fs::read(root.join(path)){Ok(v)=>v,Err(_)=>return deny("INFRASTRUCTURE_FAILURE","SOURCE_UNREADABLE")};if item["sha256"]!=sha(&b){return deny("INVALID_EVIDENCE","SOURCE_CONTENT_MISMATCH")}}
 if p["candidate_source_fingerprint"]!=sha(serde_json::to_string(manifest).unwrap().as_bytes()){return deny("INVALID_EVIDENCE","SOURCE_FINGERPRINT_MISMATCH")}
 let Some(preds)=p["predecessors"].as_array() else{return deny("INCOMPLETE_EVIDENCE","PREDECESSORS_MISSING")};if preds.len()!=PREDECESSORS.len(){return deny("INCOMPLETE_EVIDENCE","PREDECESSORS_INCOMPLETE")}
 let mut ids=HashSet::new();for(id,path,h)in PREDECESSORS{let Some(x)=preds.iter().find(|x|x["id"]==id)else{return deny("INCOMPLETE_EVIDENCE","PREDECESSOR_MISSING")};if !ids.insert(id)||x["path"]!=path||x["sha256"]!=h||x["status"]!="EMPIRICAL_PASS"{return deny("INVALID_EVIDENCE","PREDECESSOR_BINDING_MISMATCH")}let b=match fs::read(root.join(path)){Ok(v)=>v,Err(_)=>return deny("INFRASTRUCTURE_FAILURE","PREDECESSOR_UNREADABLE")};if sha(&b)!=h{return deny("INVALID_EVIDENCE","PREDECESSOR_CONTENT_MISMATCH")}}
 let e=&p["execution_evidence"];if !e["request_id"].is_string()||!e["execution_id"].is_string()||!e["result_id"].is_string()||e["stored_sequence"]!=2||!e["audit_record_hash"].is_string()||e["provenance"].as_array().map_or(true,|v|v.len()!=8){return deny("INCOMPLETE_EVIDENCE","EXECUTION_EVIDENCE_INCOMPLETE")}
 let m=&p["material_observations"];
 let fixed=[("integration_chain","ESTABLISHED"),("identity","PASS"),("authority","PASS"),("policy_contract_gate","PASS"),("execution_dispatch","PASS"),("state_transition","PASS"),("provenance","PASS"),("evidence","PASS"),("audit_handoff","PASS"),("human_principal_gate","AWAITING_HUMAN_PRINCIPAL_DECISION"),("wrong_context","REJECTED"),("missing_or_out_of_scope_authority","REJECTED"),("stale_or_revoked_authority","REJECTED"),("execution_failure","NON_SUCCESS"),("state_conflict","NON_SUCCESS"),("audit_or_validation_failure","NON_SUCCESS"),("human_principal_bypass","BLOCKED"),("infrastructure_failure","EXPLICIT_NON_SUCCESS")];
 if fixed.iter().any(|(k,v)|m[*k]!=*v)||m["workbench_requirement_preserved"]!=true||m["client_access_ip_requirement_preserved"]!=true{return deny("CERTIFIED_FAIL","MATERIAL_STAGE_I_REQUIREMENT_FAILED")}
 let a=&p["authority_observations"];if a["new_institutional_authority"]!=0||a["execution_leakage"]!=0||a["retry_leakage"]!=0||a["recovery_leakage"]!=0||a["deployment_leakage"]!=0||a["layer20"]!="ABSENT"||a["premature_frontend"]!=0||a["premature_deployment"]!=0{return deny("CERTIFIED_FAIL","AUTHORITY_OR_ARCHITECTURAL_BOUNDARY_FAILED")}
 json!({"verdict":"CERTIFIED_PASS","reason":"ALL_INDEPENDENT_STAGE_I_EXPECTATIONS_SATISFIED","accepted":true,"candidate_version":VERSION,"subject_id":SUBJECT,"evidence_package_fingerprint":claimed,"validator_id":"stage1-independent-validator-process-v1","independent_expectation":"ACCEPT","self_certification":0,"execution_leakage":0,"retry_leakage":0,"recovery_leakage":0,"deployment_leakage":0})
}
fn main(){let a:Vec<_>=env::args_os().collect();if a.len()!=3{println!("{}",deny("VALIDATOR_FAILURE","ARGUMENTS"));process::exit(2)}let v=validate(Path::new(&a[1]),Path::new(&a[2]));let pass=v["verdict"]=="CERTIFIED_PASS";println!("{v}");if !pass{process::exit(10)}}

#[cfg(test)]
mod tests {
 use super::*;
 use std::{path::PathBuf, sync::atomic::{AtomicU64,Ordering}};
 static NEXT:AtomicU64=AtomicU64::new(0);
 fn root()->PathBuf{PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()}
 fn package(root:&Path)->Value{
  let manifest:Vec<Value>=SOURCE_PATHS.iter().map(|path|json!({"path":path,"sha256":sha(&fs::read(root.join(path)).unwrap())})).collect();
  let predecessors:Vec<Value>=PREDECESSORS.iter().map(|(id,path,h)|json!({"id":id,"path":path,"sha256":h,"status":"EMPIRICAL_PASS"})).collect();
  let mut p=json!({"schema_version":1,"repository":"AI-Consulting-Lab","subject_id":SUBJECT,"candidate_version":VERSION,"certified_parent":PARENT,"candidate_source_fingerprint":sha(serde_json::to_string(&manifest).unwrap().as_bytes()),"candidate_source_manifest":manifest,"context_id":"synthetic-client-A","generation":1,"environment":{"execution":"deterministic-local-synthetic","storage":"disposable-local-sqlite","production":false},"independence":{"producer_id":"integration-kernel-producer-v1","validator_id":"stage1-independent-validator-process-v1","separate_process":true,"imports_producer_decision_logic":false,"uses_producer_declared_pass":false},"predecessors":predecessors,"execution_evidence":{"request_id":"r","execution_id":"e","result_id":"o","stored_sequence":2,"provenance":["1","2","3","4","5","6","7","8"],"audit_record_id":"a","audit_record_hash":"h"},"material_observations":{"integration_chain":"ESTABLISHED","identity":"PASS","authority":"PASS","policy_contract_gate":"PASS","execution_dispatch":"PASS","state_transition":"PASS","provenance":"PASS","evidence":"PASS","audit_handoff":"PASS","human_principal_gate":"AWAITING_HUMAN_PRINCIPAL_DECISION","wrong_context":"REJECTED","missing_or_out_of_scope_authority":"REJECTED","stale_or_revoked_authority":"REJECTED","execution_failure":"NON_SUCCESS","state_conflict":"NON_SUCCESS","audit_or_validation_failure":"NON_SUCCESS","human_principal_bypass":"BLOCKED","infrastructure_failure":"EXPLICIT_NON_SUCCESS","workbench_requirement_preserved":true,"client_access_ip_requirement_preserved":true},"authority_observations":{"new_institutional_authority":0,"execution_leakage":0,"retry_leakage":0,"recovery_leakage":0,"deployment_leakage":0,"layer20":"ABSENT","premature_frontend":0,"premature_deployment":0},"producer_declared_pass":true});
  p["package_fingerprint"]=json!(sha(&serde_json::to_vec(&p).unwrap()));p
 }
 fn run(mut p:Value)->Value{let root=root();let n=NEXT.fetch_add(1,Ordering::SeqCst);let path=std::env::temp_dir().join(format!("titus-stage1-validator-test-{}-{n}.json",process::id()));fs::write(&path,serde_json::to_vec(&p).unwrap()).unwrap();let v=validate(&root,&path);fs::remove_file(path).unwrap();p=Value::Null;drop(p);v}
 fn refingerprint(p:&mut Value){p.as_object_mut().unwrap().remove("package_fingerprint");p["package_fingerprint"]=json!(sha(&serde_json::to_vec(&p).unwrap()));}
 #[test]fn known_good_passes(){assert_eq!(run(package(&root()))["verdict"],"CERTIFIED_PASS")}
 #[test]fn material_bad_fails_independent_of_producer_pass(){let mut p=package(&root());p["material_observations"]["authority"]=json!("FAIL");refingerprint(&mut p);assert_eq!(run(p)["verdict"],"CERTIFIED_FAIL")}
 #[test]fn incomplete_package_fails(){let mut p=package(&root());p.as_object_mut().unwrap().remove("execution_evidence");refingerprint(&mut p);assert_ne!(run(p)["verdict"],"CERTIFIED_PASS")}
 #[test]fn altered_package_fails(){let mut p=package(&root());p["context_id"]=json!("altered");assert_eq!(run(p)["reason"],"PACKAGE_FINGERPRINT_MISMATCH")}
 #[test]fn wrong_candidate_fails(){let mut p=package(&root());p["candidate_version"]=json!("wrong");refingerprint(&mut p);assert_ne!(run(p)["verdict"],"CERTIFIED_PASS")}
 #[test]fn wrong_context_fails(){let mut p=package(&root());p["context_id"]=json!("foreign");refingerprint(&mut p);assert_ne!(run(p)["verdict"],"CERTIFIED_PASS")}
 #[test]fn wrong_predecessor_fails(){let mut p=package(&root());p["predecessors"][0]["sha256"]=json!("wrong");refingerprint(&mut p);assert_ne!(run(p)["verdict"],"CERTIFIED_PASS")}
 #[test]fn producer_pass_without_support_fails(){let mut p=package(&root());p["material_observations"]["state_transition"]=json!("FAIL");p["producer_declared_pass"]=json!(true);refingerprint(&mut p);assert_ne!(run(p)["verdict"],"CERTIFIED_PASS")}
 #[test]fn validator_failure_is_not_pass(){assert_ne!(deny("VALIDATOR_FAILURE","BOUNDED_TEST")["verdict"],"CERTIFIED_PASS")}
}
