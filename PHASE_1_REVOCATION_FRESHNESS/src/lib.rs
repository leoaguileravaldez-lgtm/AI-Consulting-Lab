use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, ContractResult, FoundationalContracts, ObservedBehavior,
};
use ai_consulting_lab_phase1_global_uniqueness_concurrency::infrastructure_available;
use ai_consulting_lab_phase1_transactional_persistence::sqlite_available;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const SQLITE: &str = "/usr/bin/sqlite3";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityState {
    Current,
    Revoked,
    Superseded,
    Stale,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevocationDisposition {
    AcceptedCurrent,
    ExpectedRevokedRejection,
    ExpectedStaleRejection,
    ExpectedSupersededRejection,
    UnknownOrIndeterminate,
    RevocationCommitted,
    UnexpectedDatabaseError,
    BehavioralFailure,
    HarnessFailure,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observation {
    pub disposition: RevocationDisposition,
    pub authority_state: AuthorityState,
    pub authoritative_generation: i64,
    pub evaluated_epoch: i64,
    pub cache_invalidated: bool,
    pub database_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SqliteRevocationStore {
    path: PathBuf,
}
impl SqliteRevocationStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    fn execute(&self, sql: &str) -> Result<Execution, String> {
        let out = Command::new(SQLITE)
            .arg("-batch")
            .arg("-bail")
            .arg(&self.path)
            .arg(sql)
            .output()
            .map_err(|e| format!("PROCESS:{e}"))?;
        Ok(Execution {
            success: out.status.success(),
            code: out.status.code(),
            stdout: String::from_utf8(out.stdout).map_err(|_| "STDOUT")?,
            stderr: String::from_utf8(out.stderr).map_err(|_| "STDERR")?,
        })
    }
    pub fn bootstrap(&self) -> Result<(), String> {
        if !sqlite_available() || !infrastructure_available() {
            return Err("PREDECESSOR_INFRASTRUCTURE".into());
        }
        require(self.execute(SCHEMA)?, "BOOTSTRAP").map(|_| ())
    }
    pub fn seed_current(&self, generation: i64) -> Result<(), String> {
        let sql = format!(
            "BEGIN IMMEDIATE; INSERT INTO authorities(id,generation,status) VALUES('synthetic-authority',{generation},'CURRENT'); INSERT INTO projections(id,observed_generation,cached_status,valid) VALUES('synthetic-authority',{generation},'CURRENT',1); COMMIT;"
        );
        require(self.execute(&sql)?, "SEED").map(|_| ())
    }
    pub fn revoke(&self, expected: i64, epoch: i64) -> Observation {
        self.transition(expected, epoch, "REVOKED", true)
    }
    pub fn supersede(&self, expected: i64, generation: i64) -> Observation {
        self.transition(expected, generation, "SUPERSEDED", false)
    }
    fn transition(&self, expected: i64, next: i64, status: &str, revocation: bool) -> Observation {
        let append = if revocation {
            format!(
                "INSERT INTO revocations(authority_id,epoch,reason) SELECT 'synthetic-authority',{next},'synthetic-revocation' WHERE changes()=1;"
            )
        } else {
            String::new()
        };
        let sql = format!(
            "PRAGMA synchronous=FULL; BEGIN IMMEDIATE; UPDATE authorities SET generation={next},status='{status}' WHERE id='synthetic-authority' AND generation={expected} AND status='CURRENT'; SELECT changes(); {append} COMMIT;"
        );
        let ex = match self.execute(&sql) {
            Ok(v) => v,
            Err(_) => return failure(RevocationDisposition::HarnessFailure),
        };
        if !ex.success {
            return Observation {
                database_error: Some(ex.stderr),
                ..failure(RevocationDisposition::UnexpectedDatabaseError)
            };
        }
        let changed = ex.stdout.lines().any(|x| x == "1");
        if changed {
            let mut o = self.inspect();
            o.disposition = RevocationDisposition::RevocationCommitted;
            o
        } else {
            self.inspect_with(RevocationDisposition::ExpectedStaleRejection)
        }
    }
    pub fn authorize_cached(&self, cached_epoch: i64) -> Observation {
        let mut o = self.inspect();
        match o.authority_state {
            AuthorityState::Current if cached_epoch == o.authoritative_generation => {
                o.disposition = RevocationDisposition::AcceptedCurrent;
                o
            }
            AuthorityState::Revoked if cached_epoch < o.authoritative_generation => {
                self.invalidate_projection(o, RevocationDisposition::ExpectedRevokedRejection)
            }
            AuthorityState::Superseded if cached_epoch < o.authoritative_generation => {
                self.invalidate_projection(o, RevocationDisposition::ExpectedSupersededRejection)
            }
            AuthorityState::Unknown => {
                o.disposition = RevocationDisposition::UnknownOrIndeterminate;
                o
            }
            _ => {
                o.disposition = RevocationDisposition::ExpectedStaleRejection;
                o
            }
        }
    }
    fn invalidate_projection(
        &self,
        mut o: Observation,
        disposition: RevocationDisposition,
    ) -> Observation {
        let sql = format!(
            "BEGIN IMMEDIATE; UPDATE projections SET valid=0,observed_generation={} WHERE id='synthetic-authority'; COMMIT;",
            o.authoritative_generation
        );
        match self.execute(&sql) {
            Ok(ex) if ex.success => {
                o.disposition = disposition;
                o.cache_invalidated = true;
                o
            }
            Ok(ex) => Observation {
                database_error: Some(ex.stderr),
                ..failure(RevocationDisposition::UnexpectedDatabaseError)
            },
            Err(_) => failure(RevocationDisposition::HarnessFailure),
        }
    }
    pub fn attempt_resurrection(&self) -> Observation {
        let ex=match self.execute("UPDATE authorities SET status='CURRENT',generation=generation+1 WHERE id='synthetic-authority';"){Ok(v)=>v,Err(_)=>return failure(RevocationDisposition::HarnessFailure)};
        let mut o = self.inspect();
        if !ex.success && ex.code == Some(19) && ex.stderr.contains("AUTHORITY_RESURRECTION") {
            o.disposition = RevocationDisposition::ExpectedStaleRejection;
            o.database_error = Some(ex.stderr);
            o
        } else {
            o.disposition = RevocationDisposition::BehavioralFailure;
            o
        }
    }
    fn inspect_with(&self, d: RevocationDisposition) -> Observation {
        let mut o = self.inspect();
        o.disposition = d;
        o
    }
    fn inspect(&self) -> Observation {
        let ex=match self.execute("SELECT generation||'|'||status||'|'||COALESCE((SELECT max(epoch) FROM revocations),generation) FROM authorities WHERE id='synthetic-authority';"){Ok(v)=>v,Err(_)=>return failure(RevocationDisposition::HarnessFailure)};
        if !ex.success {
            return failure(RevocationDisposition::HarnessFailure);
        }
        if ex.stdout.trim().is_empty() {
            return failure(RevocationDisposition::UnknownOrIndeterminate);
        }
        let p: Vec<_> = ex.stdout.trim().split('|').collect();
        if p.len() != 3 {
            return failure(RevocationDisposition::HarnessFailure);
        }
        let generation = p[0].parse().unwrap_or(-1);
        let epoch = p[2].parse().unwrap_or(-1);
        let state = match p[1] {
            "CURRENT" => AuthorityState::Current,
            "REVOKED" => AuthorityState::Revoked,
            "SUPERSEDED" => AuthorityState::Superseded,
            _ => AuthorityState::Unknown,
        };
        Observation {
            disposition: RevocationDisposition::BehavioralFailure,
            authority_state: state,
            authoritative_generation: generation,
            evaluated_epoch: epoch,
            cache_invalidated: false,
            database_error: None,
        }
    }
}

#[derive(Debug)]
struct Execution {
    success: bool,
    code: Option<i32>,
    stdout: String,
    stderr: String,
}
fn require(ex: Execution, label: &str) -> Result<Execution, String> {
    if ex.success {
        Ok(ex)
    } else {
        Err(format!("{label}:{:?}:{}", ex.code, ex.stderr))
    }
}
fn failure(d: RevocationDisposition) -> Observation {
    Observation {
        disposition: d,
        authority_state: AuthorityState::Unknown,
        authoritative_generation: -1,
        evaluated_epoch: -1,
        cache_invalidated: false,
        database_error: None,
    }
}
fn digest(i: &BehavioralInput) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&i.scenario_data).unwrap())
    )
}

const SCHEMA:&str="
PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;
CREATE TABLE IF NOT EXISTS authorities(id TEXT PRIMARY KEY,generation INTEGER NOT NULL,status TEXT NOT NULL CHECK(status IN('CURRENT','REVOKED','SUPERSEDED')));
CREATE TABLE IF NOT EXISTS revocations(authority_id TEXT NOT NULL REFERENCES authorities(id),epoch INTEGER NOT NULL UNIQUE,reason TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS projections(id TEXT PRIMARY KEY,observed_generation INTEGER NOT NULL,cached_status TEXT NOT NULL,valid INTEGER NOT NULL CHECK(valid IN(0,1)));
CREATE TRIGGER IF NOT EXISTS no_resurrection BEFORE UPDATE ON authorities WHEN OLD.status IN('REVOKED','SUPERSEDED') AND NEW.status='CURRENT' BEGIN SELECT RAISE(ABORT,'AUTHORITY_RESURRECTION'); END;
CREATE TRIGGER IF NOT EXISTS monotonic_generation BEFORE UPDATE ON authorities WHEN NEW.generation<=OLD.generation BEGIN SELECT RAISE(ABORT,'NON_MONOTONIC_GENERATION'); END;
";

#[derive(Debug, Clone)]
pub struct RevocationFreshnessAdapter {
    store: SqliteRevocationStore,
}
impl RevocationFreshnessAdapter {
    pub fn new(store: SqliteRevocationStore) -> Self {
        Self { store }
    }
}
impl FoundationalContracts for RevocationFreshnessAdapter {
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
    fn propagate_revocation_and_freshness(&self, input: &BehavioralInput) -> ContractResult {
        let o = self.store.authorize_cached(
            input.scenario_data["cached_grant_epoch"]
                .as_i64()
                .unwrap_or(-1),
        );
        Ok(ObservedBehavior {
            input_digest: digest(input),
            facts: json!({"accepted":o.disposition==RevocationDisposition::AcceptedCurrent,"evaluated_revocation_epoch":o.evaluated_epoch,"cache_invalidated":o.cache_invalidated,"classification":format!("{:?}",o.disposition),"authority_state":format!("{:?}",o.authority_state),"authoritative_database":true,"database_engine":"sqlite","database_stderr":o.database_error}),
        })
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
pub fn infrastructure_adequate() -> bool {
    Path::new(SQLITE).is_file() && sqlite_available() && infrastructure_available()
}
