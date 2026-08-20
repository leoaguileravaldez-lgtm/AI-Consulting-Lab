use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, ContractResult, FoundationalContracts, ObservedBehavior,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const SQLITE: &str = "/usr/bin/sqlite3";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationDisposition {
    AcceptedOwnContext,
    ExpectedForeignContextRejection,
    ExpectedAuthorityRejection,
    ExpectedIsolationRejection,
    UnexpectedOsError,
    UnexpectedDatabaseError,
    BehavioralFailure,
    HarnessFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationObservation {
    pub disposition: IsolationDisposition,
    pub accepted: bool,
    pub boundary_crossed: bool,
    pub effective_actor: String,
    pub resource_value: Option<String>,
    pub process_status: Option<i32>,
    pub stderr: String,
}

#[derive(Debug, Clone)]
pub struct RuntimeIsolationBoundary {
    worker: PathBuf,
    authority_db: PathBuf,
    resource_root: PathBuf,
}

impl RuntimeIsolationBoundary {
    pub fn new(
        worker: impl Into<PathBuf>,
        authority_db: impl Into<PathBuf>,
        resource_root: impl Into<PathBuf>,
    ) -> Self {
        Self {
            worker: worker.into(),
            authority_db: authority_db.into(),
            resource_root: resource_root.into(),
        }
    }

    pub fn access(
        &self,
        token: &str,
        workload_client: &str,
        actor: &str,
        attestation: &str,
        object_client: &str,
        object_name: &str,
    ) -> IsolationObservation {
        let output = Command::new(&self.worker)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("AILAB_AUTHORITY_DB", &self.authority_db)
            .env("AILAB_RESOURCE_ROOT", &self.resource_root)
            .args([
                token,
                workload_client,
                actor,
                attestation,
                object_client,
                object_name,
            ])
            .output();
        let Ok(output) = output else {
            return failed(
                IsolationDisposition::HarnessFailure,
                actor,
                None,
                "worker launch failed",
            );
        };
        let code = output.status.code();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        let disposition = match code {
            Some(0) => IsolationDisposition::AcceptedOwnContext,
            Some(10) => IsolationDisposition::ExpectedForeignContextRejection,
            Some(11) => IsolationDisposition::ExpectedAuthorityRejection,
            Some(12) => IsolationDisposition::ExpectedIsolationRejection,
            Some(20) => IsolationDisposition::UnexpectedOsError,
            Some(21) => IsolationDisposition::UnexpectedDatabaseError,
            _ => IsolationDisposition::HarnessFailure,
        };
        IsolationObservation {
            accepted: disposition == IsolationDisposition::AcceptedOwnContext,
            boundary_crossed: false,
            effective_actor: actor.to_owned(),
            resource_value: (disposition == IsolationDisposition::AcceptedOwnContext)
                .then_some(stdout),
            process_status: code,
            stderr,
            disposition,
        }
    }
}

fn failed(
    disposition: IsolationDisposition,
    actor: &str,
    code: Option<i32>,
    stderr: &str,
) -> IsolationObservation {
    IsolationObservation {
        disposition,
        accepted: false,
        boundary_crossed: false,
        effective_actor: actor.into(),
        resource_value: None,
        process_status: code,
        stderr: stderr.into(),
    }
}

pub fn bootstrap_authority(db: &Path, token_a: &str, token_b: &str) -> Result<(), String> {
    let sql = format!(
        "PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; CREATE TABLE capabilities(token TEXT PRIMARY KEY, workload_client TEXT NOT NULL, actor TEXT NOT NULL CHECK(actor='MACHINE'), attestation TEXT NOT NULL CHECK(attestation IN('CURRENT','REVOKED')), status TEXT NOT NULL CHECK(status IN('ACTIVE','REVOKED'))); INSERT INTO capabilities VALUES('{token_a}','client-A','MACHINE','CURRENT','ACTIVE'); INSERT INTO capabilities VALUES('{token_b}','client-B','MACHINE','CURRENT','ACTIVE');"
    );
    sqlite(db, &sql).map(|_| ())
}

pub fn set_revoked(db: &Path, token: &str) -> Result<(), String> {
    sqlite(db, &format!("UPDATE capabilities SET attestation='REVOKED',status='REVOKED' WHERE token='{token}'; SELECT changes();")).and_then(|s| if s.lines().any(|v| v == "1") { Ok(()) } else { Err("capability not found".into()) })
}

fn sqlite(db: &Path, sql: &str) -> Result<String, String> {
    let out = Command::new(SQLITE)
        .arg("-batch")
        .arg("-bail")
        .arg(db)
        .arg(sql)
        .output()
        .map_err(|e| format!("PROCESS:{e}"))?;
    if !out.status.success() {
        return Err(format!(
            "SQLITE:{:?}:{}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

pub fn opaque_token(label: &str, nonce: u64) -> String {
    format!(
        "{:x}",
        Sha256::digest(
            format!("runtime-isolation:{label}:{nonce}:{}", std::process::id()).as_bytes()
        )
    )
}
fn digest(input: &BehavioralInput) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&input.scenario_data).unwrap())
    )
}

#[derive(Debug, Clone)]
pub struct RuntimeIsolationAdapter {
    boundary: RuntimeIsolationBoundary,
    token: String,
}
impl RuntimeIsolationAdapter {
    pub fn new(boundary: RuntimeIsolationBoundary, token: impl Into<String>) -> Self {
        Self {
            boundary,
            token: token.into(),
        }
    }
}

impl FoundationalContracts for RuntimeIsolationAdapter {
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
    fn attest_runtime_isolation(&self, input: &BehavioralInput) -> ContractResult {
        let o = self.boundary.access(
            &self.token,
            input.scenario_data["workload_client"]
                .as_str()
                .unwrap_or(""),
            input.scenario_data["actor_type"].as_str().unwrap_or(""),
            input.scenario_data["attestation"].as_str().unwrap_or(""),
            input.scenario_data["object_client"].as_str().unwrap_or(""),
            "object.txt",
        );
        Ok(ObservedBehavior {
            input_digest: digest(input),
            facts: json!({"accepted":o.accepted,"boundary_crossed":o.boundary_crossed,"effective_actor":o.effective_actor,"classification":format!("{:?}",o.disposition),"separate_process":true,"database_bound_capability":true,"process_status":o.process_status,"stderr":o.stderr}),
        })
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

pub fn infrastructure_adequate() -> bool {
    Path::new(SQLITE).is_file()
}
