//! Bounded local implementation of BC_TRANSACTIONAL_PERSISTENCE only.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, ContractResult, FoundationalContracts, ObservedBehavior,
};
use serde_json::json;
use sha2::{Digest, Sha256};

const SQLITE: &str = "/usr/bin/sqlite3";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceDisposition {
    Committed,
    RolledBackExplicitly,
    RolledBackInjectedFailure,
    RejectedInvalidPredecessor,
    RejectedInvalidProvenance,
    RejectedReplay,
    UnexpectedDatabaseError,
    BehavioralFailure,
    InfrastructureFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceClass {
    SuccessfulCommit,
    ExplicitRollback,
    ExpectedDatabaseRejection,
    UnexpectedDatabaseError,
    BehavioralFailure,
    HarnessFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseErrorEvidence {
    pub exit_code: Option<i32>,
    pub stderr: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceObservation {
    pub disposition: PersistenceDisposition,
    pub evidence_class: EvidenceClass,
    pub state_committed: bool,
    pub audit_committed: bool,
    pub provenance_committed: bool,
    pub database_error: Option<DatabaseErrorEvidence>,
}

/// Replaceable persistence boundary; SQLite is an empirical adapter, not a contract dependency.
pub trait TransactionalStore {
    fn bootstrap(&self) -> Result<(), String>;
    fn transact(&self, input: &BehavioralInput) -> PersistenceObservation;
}

#[derive(Debug, Clone)]
pub struct SqliteCliStore {
    database_path: PathBuf,
}

impl SqliteCliStore {
    pub fn new(database_path: impl Into<PathBuf>) -> Self {
        Self {
            database_path: database_path.into(),
        }
    }

    fn execute(&self, sql: &str) -> Result<SqliteExecution, String> {
        let output = Command::new(SQLITE)
            .arg("-batch")
            .arg("-bail")
            .arg(&self.database_path)
            .arg(sql)
            .output()
            .map_err(|error| format!("SQLITE_PROCESS:{error}"))?;
        Ok(SqliteExecution {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8(output.stdout)
                .map_err(|_| "SQLITE_STDOUT_ENCODING".to_string())?,
            stderr: String::from_utf8(output.stderr)
                .map_err(|_| "SQLITE_STDERR_ENCODING".to_string())?,
        })
    }

    fn counts(&self, operation_id: &str) -> Result<(bool, bool, bool), String> {
        let sql = format!(
            "SELECT (SELECT count(*) FROM authoritative_objects WHERE operation_id='{0}') || '|' || (SELECT count(*) FROM audit_events WHERE operation_id='{0}') || '|' || (SELECT count(*) FROM provenance_records WHERE operation_id='{0}');",
            sql_literal(operation_id)
        );
        let output = self.execute(&sql)?;
        if !output.success {
            return Err(format!(
                "SQLITE_COUNT_QUERY:exit={:?}:stderr={}",
                output.exit_code, output.stderr
            ));
        }
        let values: Vec<_> = output.stdout.trim().split('|').collect();
        if values.len() != 3 {
            return Err("SQLITE_COUNT_SHAPE".into());
        }
        Ok((values[0] == "1", values[1] == "1", values[2] == "1"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SqliteExecution {
    success: bool,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl TransactionalStore for SqliteCliStore {
    fn bootstrap(&self) -> Result<(), String> {
        let output = self.execute(SCHEMA)?;
        if output.success {
            Ok(())
        } else {
            Err(format!(
                "SQLITE_BOOTSTRAP:exit={:?}:stderr={}",
                output.exit_code, output.stderr
            ))
        }
    }

    fn transact(&self, input: &BehavioralInput) -> PersistenceObservation {
        let operation_id = input_digest(input);
        let Some(predecessor) = input.scenario_data["predecessor_version"].as_i64() else {
            return empty(PersistenceDisposition::RejectedInvalidPredecessor);
        };
        let state = input.scenario_data["state_prepare"].as_bool() == Some(true);
        let audit = input.scenario_data["audit_prepare"].as_bool() == Some(true);
        let provenance = input.scenario_data["provenance_prepare"].as_bool() == Some(true);
        if !state || !audit {
            return empty(PersistenceDisposition::RejectedInvalidProvenance);
        }

        let provenance_value = if provenance {
            "'synthetic-provenance'"
        } else {
            "NULL"
        };
        let final_statement = if input.scenario_data["explicit_rollback"].as_bool() == Some(true) {
            "ROLLBACK;"
        } else {
            "COMMIT;"
        };
        let op = sql_literal(&operation_id);
        let sql = format!(
            "PRAGMA synchronous=FULL;
             PRAGMA foreign_keys=ON;
             BEGIN IMMEDIATE;
             INSERT INTO operation_guard(operation_id) VALUES('{op}');
             INSERT INTO authoritative_objects(operation_id, predecessor_version, payload) VALUES('{op}', {predecessor}, 'synthetic-state');
             INSERT INTO audit_events(operation_id, predecessor_version, event_payload) VALUES('{op}', {predecessor}, 'synthetic-audit');
             INSERT INTO provenance_records(operation_id, predecessor_version, provenance) VALUES('{op}', {predecessor}, {provenance_value});
             {final_statement}"
        );
        let execution = match self.execute(&sql) {
            Ok(output) => output,
            Err(_) => return empty(PersistenceDisposition::InfrastructureFailure),
        };
        let observed = match self.counts(&operation_id) {
            Ok(values) => values,
            Err(_) => return empty(PersistenceDisposition::InfrastructureFailure),
        };
        let error = (!execution.success).then(|| DatabaseErrorEvidence {
            exit_code: execution.exit_code,
            stderr: execution.stderr.clone(),
        });
        match (execution.success, observed) {
            (true, (true, true, true)) => PersistenceObservation {
                disposition: PersistenceDisposition::Committed,
                evidence_class: EvidenceClass::SuccessfulCommit,
                state_committed: true,
                audit_committed: true,
                provenance_committed: true,
                database_error: None,
            },
            (true, (false, false, false))
                if input.scenario_data["explicit_rollback"].as_bool() == Some(true) =>
            {
                empty_with_class(
                    PersistenceDisposition::RolledBackExplicitly,
                    EvidenceClass::ExplicitRollback,
                    None,
                )
            }
            (false, (false, false, false))
                if predecessor != 7
                    && expected_constraint(
                        &execution,
                        "CHECK constraint failed: predecessor_version = 7",
                    ) =>
            {
                empty_with_class(
                    PersistenceDisposition::RejectedInvalidPredecessor,
                    EvidenceClass::ExpectedDatabaseRejection,
                    error,
                )
            }
            (false, (false, false, false))
                if !provenance
                    && expected_constraint(
                        &execution,
                        "NOT NULL constraint failed: provenance_records.provenance",
                    ) =>
            {
                empty_with_class(
                    PersistenceDisposition::RolledBackInjectedFailure,
                    EvidenceClass::ExpectedDatabaseRejection,
                    error,
                )
            }
            (false, (true, true, true))
                if provenance
                    && expected_constraint(
                        &execution,
                        "UNIQUE constraint failed: operation_guard.operation_id",
                    ) =>
            {
                PersistenceObservation {
                    disposition: PersistenceDisposition::RejectedReplay,
                    evidence_class: EvidenceClass::ExpectedDatabaseRejection,
                    state_committed: true,
                    audit_committed: true,
                    provenance_committed: true,
                    database_error: error,
                }
            }
            (false, (false, false, false)) => empty_with_class(
                PersistenceDisposition::UnexpectedDatabaseError,
                EvidenceClass::UnexpectedDatabaseError,
                error,
            ),
            _ => PersistenceObservation {
                disposition: PersistenceDisposition::BehavioralFailure,
                evidence_class: EvidenceClass::BehavioralFailure,
                state_committed: observed.0,
                audit_committed: observed.1,
                provenance_committed: observed.2,
                database_error: error,
            },
        }
    }
}

fn expected_constraint(execution: &SqliteExecution, constraint_identity: &str) -> bool {
    execution.exit_code == Some(19) && execution.stderr.contains(constraint_identity)
}

fn empty(disposition: PersistenceDisposition) -> PersistenceObservation {
    empty_with_class(disposition, EvidenceClass::HarnessFailure, None)
}

fn empty_with_class(
    disposition: PersistenceDisposition,
    evidence_class: EvidenceClass,
    database_error: Option<DatabaseErrorEvidence>,
) -> PersistenceObservation {
    PersistenceObservation {
        disposition,
        evidence_class,
        state_committed: false,
        audit_committed: false,
        provenance_committed: false,
        database_error,
    }
}

fn sql_literal(value: &str) -> String {
    value.replace('\'', "''")
}

fn input_digest(input: &BehavioralInput) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&input.scenario_data).unwrap())
    )
}

const SCHEMA: &str = "
PRAGMA journal_mode=WAL;
PRAGMA synchronous=FULL;
PRAGMA foreign_keys=ON;
CREATE TABLE IF NOT EXISTS operation_guard(operation_id TEXT PRIMARY KEY);
CREATE TABLE IF NOT EXISTS authoritative_objects(
  operation_id TEXT PRIMARY KEY REFERENCES operation_guard(operation_id),
  predecessor_version INTEGER NOT NULL CHECK(predecessor_version = 7),
  payload TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS audit_events(
  sequence INTEGER PRIMARY KEY AUTOINCREMENT,
  operation_id TEXT NOT NULL UNIQUE REFERENCES operation_guard(operation_id),
  predecessor_version INTEGER NOT NULL CHECK(predecessor_version = 7),
  event_payload TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS provenance_records(
  sequence INTEGER PRIMARY KEY AUTOINCREMENT,
  operation_id TEXT NOT NULL UNIQUE REFERENCES operation_guard(operation_id),
  predecessor_version INTEGER NOT NULL CHECK(predecessor_version = 7),
  provenance TEXT NOT NULL
);
CREATE TRIGGER IF NOT EXISTS audit_no_update BEFORE UPDATE ON audit_events BEGIN SELECT RAISE(ABORT, 'APPEND_ONLY_AUDIT'); END;
CREATE TRIGGER IF NOT EXISTS audit_no_delete BEFORE DELETE ON audit_events BEGIN SELECT RAISE(ABORT, 'APPEND_ONLY_AUDIT'); END;
CREATE TRIGGER IF NOT EXISTS provenance_no_update BEFORE UPDATE ON provenance_records BEGIN SELECT RAISE(ABORT, 'APPEND_ONLY_PROVENANCE'); END;
CREATE TRIGGER IF NOT EXISTS provenance_no_delete BEFORE DELETE ON provenance_records BEGIN SELECT RAISE(ABORT, 'APPEND_ONLY_PROVENANCE'); END;
";

#[derive(Debug, Clone)]
pub struct TransactionalPersistenceAdapter<S> {
    store: S,
}

impl<S> TransactionalPersistenceAdapter<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl FoundationalContracts for TransactionalPersistenceAdapter<SqliteCliStore> {
    fn canonical_representation(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn bind_identity_version_provenance(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }
    fn verify_cryptographic_integrity(&self, _: &BehavioralInput) -> ContractResult {
        Err(ContractError::ExpectedFailNotImplemented)
    }

    fn persist_transactionally(&self, input: &BehavioralInput) -> ContractResult {
        let observed = self.store.transact(input);
        Ok(ObservedBehavior {
            input_digest: input_digest(input),
            facts: json!({
                "accepted": observed.disposition == PersistenceDisposition::Committed,
                "state_committed": observed.state_committed,
                "audit_committed": observed.audit_committed,
                "provenance_committed": observed.provenance_committed,
                "classification": format!("{:?}", observed.disposition),
                "evidence_class": format!("{:?}", observed.evidence_class),
                "database_atomic_boundary": true,
                "database_engine": "sqlite",
                "database_observed": observed.disposition != PersistenceDisposition::InfrastructureFailure,
                "database_exit_code": observed.database_error.as_ref().and_then(|error| error.exit_code),
                "database_stderr": observed.database_error.as_ref().map(|error| error.stderr.as_str()),
            }),
        })
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

pub fn sqlite_available() -> bool {
    Path::new(SQLITE).is_file()
}
