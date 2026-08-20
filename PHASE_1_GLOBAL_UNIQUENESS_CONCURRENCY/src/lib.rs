//! Bounded local implementation of BC_GLOBAL_UNIQUENESS_CONCURRENCY only.

use std::{
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio},
};

use ai_consulting_lab_phase0_contracts::{
    BehavioralInput, ContractError, ContractResult, FoundationalContracts, ObservedBehavior,
};
use ai_consulting_lab_phase1_transactional_persistence::sqlite_available;
use serde_json::json;
use sha2::{Digest, Sha256};

const SQLITE: &str = "/usr/bin/sqlite3";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcurrencyDisposition {
    Committed,
    ExpectedStaleWriteRejection,
    ExpectedUniquenessRejection,
    ExpectedReplayRejection,
    ExpectedConcurrencyRejection,
    UnexpectedDatabaseError,
    BehavioralFailure,
    HarnessFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseEvidence {
    pub exit_code: Option<i32>,
    pub stderr: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasObservation {
    pub disposition: ConcurrencyDisposition,
    pub stored_sequence: i64,
    pub stored_fence: i64,
    pub successor_created: bool,
    pub history_count: i64,
    pub database_error: Option<DatabaseEvidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirstWriter {
    A,
    B,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaceObservation {
    pub independent_connections: usize,
    pub overlapping_snapshots: bool,
    pub snapshot_a: (i64, i64),
    pub snapshot_b: (i64, i64),
    pub winner: String,
    pub loser: String,
    pub loser_disposition: ConcurrencyDisposition,
    pub final_sequence: i64,
    pub final_fence: i64,
    pub history_count: i64,
    pub database_enforced: bool,
    pub loser_error: DatabaseEvidence,
}

#[derive(Debug, Clone)]
pub struct SqliteConcurrencyStore {
    database_path: PathBuf,
}

impl SqliteConcurrencyStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            database_path: path.into(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.database_path
    }

    pub fn bootstrap(&self) -> Result<(), String> {
        if !sqlite_available() {
            return Err("CERTIFIED_PREDECESSOR_SQLITE_UNAVAILABLE".into());
        }
        let result = self.execute(SCHEMA)?;
        require_success("BOOTSTRAP", result).map(|_| ())
    }

    pub fn seed(&self, domain: &str, sequence: i64, fence: i64) -> Result<(), String> {
        let sql = format!(
            "BEGIN IMMEDIATE;
             INSERT INTO uniqueness_domains(domain,sequence,fence,winner_operation) VALUES('{0}',{sequence},{fence},'seed');
             INSERT INTO successor_history(operation_id,domain,predecessor_sequence,new_sequence,new_fence)
             SELECT 'seed-history','{0}',{sequence}-1,{sequence},{fence} WHERE {sequence}>7;
             COMMIT;",
            sql_literal(domain),
        );
        require_success("SEED", self.execute(&sql)?).map(|_| ())
    }

    pub fn cas(
        &self,
        operation_id: &str,
        domain: &str,
        expected_sequence: i64,
        expected_fence: i64,
        new_sequence: i64,
        new_fence: i64,
    ) -> CasObservation {
        let op = sql_literal(operation_id);
        let domain = sql_literal(domain);
        let sql = format!(
            "PRAGMA synchronous=FULL;
             BEGIN IMMEDIATE;
             UPDATE uniqueness_domains SET sequence={new_sequence},fence={new_fence},winner_operation='{op}'
             WHERE domain='{domain}' AND sequence={expected_sequence} AND fence={expected_fence};
             SELECT changes();
             INSERT INTO successor_history(operation_id,domain,predecessor_sequence,new_sequence,new_fence)
             SELECT '{op}','{domain}',{expected_sequence},{new_sequence},{new_fence} WHERE changes()=1;
             COMMIT;"
        );
        let execution = match self.execute(&sql) {
            Ok(value) => value,
            Err(_) => return harness_failure(),
        };
        let state = match self.state(&domain) {
            Ok(value) => value,
            Err(_) => return harness_failure(),
        };
        let error = (!execution.success).then(|| execution.evidence());
        let updated = execution
            .stdout
            .lines()
            .find_map(|line| line.parse::<i64>().ok());
        if execution.success
            && updated == Some(1)
            && state.0 == new_sequence
            && state.1 == new_fence
        {
            return CasObservation {
                disposition: ConcurrencyDisposition::Committed,
                stored_sequence: state.0,
                stored_fence: state.1,
                successor_created: true,
                history_count: state.2,
                database_error: None,
            };
        }
        if execution.success
            && updated == Some(0)
            && state.0 != expected_sequence
            && state.1 != expected_fence
            && state.2 >= 1
        {
            return CasObservation {
                disposition: ConcurrencyDisposition::ExpectedStaleWriteRejection,
                stored_sequence: state.0,
                stored_fence: state.1,
                successor_created: false,
                history_count: state.2,
                database_error: None,
            };
        }
        if !execution.success
            && expected_error(
                &execution,
                19,
                "UNIQUE constraint failed: successor_history.operation_id",
            )
            && state.0 == expected_sequence
            && state.1 == expected_fence
        {
            return CasObservation {
                disposition: ConcurrencyDisposition::ExpectedReplayRejection,
                stored_sequence: state.0,
                stored_fence: state.1,
                successor_created: false,
                history_count: state.2,
                database_error: error,
            };
        }
        CasObservation {
            disposition: if execution.success {
                ConcurrencyDisposition::BehavioralFailure
            } else {
                ConcurrencyDisposition::UnexpectedDatabaseError
            },
            stored_sequence: state.0,
            stored_fence: state.1,
            successor_created: false,
            history_count: state.2,
            database_error: error,
        }
    }

    pub fn attempt_duplicate_domain(&self, domain: &str) -> CasObservation {
        let sql = format!(
            "INSERT INTO uniqueness_domains(domain,sequence,fence,winner_operation) VALUES('{}',1,1,'duplicate');",
            sql_literal(domain)
        );
        let execution = match self.execute(&sql) {
            Ok(value) => value,
            Err(_) => return harness_failure(),
        };
        let state = match self.state(domain) {
            Ok(value) => value,
            Err(_) => return harness_failure(),
        };
        CasObservation {
            disposition: if expected_error(
                &execution,
                19,
                "UNIQUE constraint failed: uniqueness_domains.domain",
            ) {
                ConcurrencyDisposition::ExpectedUniquenessRejection
            } else {
                ConcurrencyDisposition::UnexpectedDatabaseError
            },
            stored_sequence: state.0,
            stored_fence: state.1,
            successor_created: false,
            history_count: state.2,
            database_error: Some(execution.evidence()),
        }
    }

    pub fn race_two_contenders(&self, first: FirstWriter) -> Result<RaceObservation, String> {
        let mut a = Interactive::spawn(&self.database_path)?;
        let mut b = Interactive::spawn(&self.database_path)?;
        let snapshot_a = a.begin_and_snapshot("READY_A")?;
        let snapshot_b = b.begin_and_snapshot("READY_B")?;
        let (winner_result, loser_result, winner_name, loser_name) = match first {
            FirstWriter::A => {
                a.commit_winner("operation-a")?;
                b.reject_loser()?;
                (a.finish()?, b.finish()?, "A", "B")
            }
            FirstWriter::B => {
                b.commit_winner("operation-b")?;
                a.reject_loser()?;
                (b.finish()?, a.finish()?, "B", "A")
            }
        };
        let state = self.state("synthetic-successor")?;
        let loser_evidence = loser_result.evidence();
        let loser_disposition = if expected_error(&loser_result, 1, "database is locked (5)") {
            ConcurrencyDisposition::ExpectedConcurrencyRejection
        } else {
            ConcurrencyDisposition::UnexpectedDatabaseError
        };
        if !winner_result.success || state != (8, 12, 1) {
            return Err("RACE_BEHAVIORAL_FAILURE".into());
        }
        Ok(RaceObservation {
            independent_connections: 2,
            overlapping_snapshots: true,
            snapshot_a,
            snapshot_b,
            winner: winner_name.into(),
            loser: loser_name.into(),
            loser_disposition,
            final_sequence: state.0,
            final_fence: state.1,
            history_count: state.2,
            database_enforced: true,
            loser_error: loser_evidence,
        })
    }

    fn state(&self, domain: &str) -> Result<(i64, i64, i64), String> {
        let sql = format!(
            "SELECT sequence||'|'||fence||'|'||(SELECT count(*) FROM successor_history WHERE domain=d.domain) FROM uniqueness_domains d WHERE domain='{}';",
            sql_literal(domain)
        );
        let output = require_success("STATE", self.execute(&sql)?)?;
        parse_state(output.stdout.trim())
    }

    fn execute(&self, sql: &str) -> Result<Execution, String> {
        let output = Command::new(SQLITE)
            .arg("-batch")
            .arg("-bail")
            .arg(&self.database_path)
            .arg(sql)
            .output()
            .map_err(|error| format!("SQLITE_PROCESS:{error}"))?;
        Ok(Execution {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8(output.stdout).map_err(|_| "SQLITE_STDOUT_ENCODING")?,
            stderr: String::from_utf8(output.stderr).map_err(|_| "SQLITE_STDERR_ENCODING")?,
        })
    }
}

#[derive(Debug)]
struct Interactive {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    stderr: Option<ChildStderr>,
}

impl Interactive {
    fn spawn(path: &Path) -> Result<Self, String> {
        let mut child = Command::new(SQLITE)
            .arg("-batch")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("CONTENDER_SPAWN:{error}"))?;
        let stdin = child.stdin.take().ok_or("CONTENDER_STDIN")?;
        let stdout = BufReader::new(child.stdout.take().ok_or("CONTENDER_STDOUT")?);
        let stderr = Some(child.stderr.take().ok_or("CONTENDER_STDERR")?);
        Ok(Self {
            child,
            stdin,
            stdout,
            stderr,
        })
    }

    fn send(&mut self, sql: &str) -> Result<(), String> {
        writeln!(self.stdin, "{sql}").map_err(|error| format!("CONTENDER_WRITE:{error}"))?;
        self.stdin
            .flush()
            .map_err(|error| format!("CONTENDER_FLUSH:{error}"))
    }

    fn read_until(&mut self, marker: &str) -> Result<Vec<String>, String> {
        let mut lines = Vec::new();
        loop {
            let mut line = String::new();
            if self
                .stdout
                .read_line(&mut line)
                .map_err(|error| format!("CONTENDER_READ:{error}"))?
                == 0
            {
                return Err(format!("CONTENDER_MARKER_MISSING:{marker}"));
            }
            let line = line.trim().to_string();
            let done = line == marker;
            lines.push(line);
            if done {
                return Ok(lines);
            }
        }
    }

    fn begin_and_snapshot(&mut self, marker: &str) -> Result<(i64, i64), String> {
        self.send("PRAGMA busy_timeout=0;")?;
        self.send("BEGIN DEFERRED;")?;
        self.send("SELECT sequence||'|'||fence FROM uniqueness_domains WHERE domain='synthetic-successor';")?;
        self.send(&format!(".print {marker}"))?;
        let lines = self.read_until(marker)?;
        lines
            .iter()
            .find_map(|line| parse_pair(line).ok())
            .ok_or("SNAPSHOT_MISSING".into())
    }

    fn commit_winner(&mut self, operation: &str) -> Result<(), String> {
        self.send("UPDATE uniqueness_domains SET sequence=8,fence=12,winner_operation='winner' WHERE domain='synthetic-successor' AND sequence=7 AND fence=11;")?;
        self.send(&format!("INSERT INTO successor_history(operation_id,domain,predecessor_sequence,new_sequence,new_fence) SELECT '{operation}','synthetic-successor',7,8,12 WHERE changes()=1;"))?;
        self.send("COMMIT;")?;
        self.send(".print DONE_WINNER")?;
        self.read_until("DONE_WINNER").map(|_| ())
    }

    fn reject_loser(&mut self) -> Result<(), String> {
        self.send("UPDATE uniqueness_domains SET sequence=8,fence=12,winner_operation='loser' WHERE domain='synthetic-successor' AND sequence=7 AND fence=11;")?;
        self.send("ROLLBACK;")?;
        self.send(".print DONE_LOSER")?;
        self.read_until("DONE_LOSER").map(|_| ())
    }

    fn finish(mut self) -> Result<Execution, String> {
        self.stdin
            .flush()
            .map_err(|error| format!("CONTENDER_FLUSH:{error}"))?;
        drop(self.stdin);
        let mut stderr = String::new();
        self.stderr
            .take()
            .ok_or("CONTENDER_STDERR")?
            .read_to_string(&mut stderr)
            .map_err(|error| format!("CONTENDER_STDERR_READ:{error}"))?;
        let status = self
            .child
            .wait()
            .map_err(|error| format!("CONTENDER_WAIT:{error}"))?;
        Ok(Execution {
            success: status.success(),
            exit_code: status.code(),
            stdout: String::new(),
            stderr,
        })
    }
}

use std::io::Read;

#[derive(Debug, Clone)]
struct Execution {
    success: bool,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl Execution {
    fn evidence(&self) -> DatabaseEvidence {
        DatabaseEvidence {
            exit_code: self.exit_code,
            stderr: self.stderr.clone(),
        }
    }
}

fn require_success(label: &str, result: Execution) -> Result<Execution, String> {
    if result.success {
        Ok(result)
    } else {
        Err(format!(
            "{label}:exit={:?}:stderr={}",
            result.exit_code, result.stderr
        ))
    }
}
fn expected_error(result: &Execution, code: i32, identity: &str) -> bool {
    result.exit_code == Some(code) && result.stderr.contains(identity)
}
fn parse_pair(value: &str) -> Result<(i64, i64), String> {
    let mut fields = value.split('|');
    let a = fields
        .next()
        .ok_or("PAIR_A")?
        .parse()
        .map_err(|_| "PAIR_A_PARSE")?;
    let b = fields
        .next()
        .ok_or("PAIR_B")?
        .parse()
        .map_err(|_| "PAIR_B_PARSE")?;
    if fields.next().is_some() {
        return Err("PAIR_EXTRA".into());
    }
    Ok((a, b))
}
fn parse_state(value: &str) -> Result<(i64, i64, i64), String> {
    let mut fields = value.split('|');
    let a = fields
        .next()
        .ok_or("STATE_A")?
        .parse()
        .map_err(|_| "STATE_A_PARSE")?;
    let b = fields
        .next()
        .ok_or("STATE_B")?
        .parse()
        .map_err(|_| "STATE_B_PARSE")?;
    let c = fields
        .next()
        .ok_or("STATE_C")?
        .parse()
        .map_err(|_| "STATE_C_PARSE")?;
    if fields.next().is_some() {
        return Err("STATE_EXTRA".into());
    }
    Ok((a, b, c))
}
fn sql_literal(value: &str) -> String {
    value.replace('\'', "''")
}
fn harness_failure() -> CasObservation {
    CasObservation {
        disposition: ConcurrencyDisposition::HarnessFailure,
        stored_sequence: -1,
        stored_fence: -1,
        successor_created: false,
        history_count: -1,
        database_error: None,
    }
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
CREATE TABLE IF NOT EXISTS uniqueness_domains(
  domain TEXT PRIMARY KEY,
  sequence INTEGER NOT NULL,
  fence INTEGER NOT NULL,
  winner_operation TEXT NOT NULL,
  CHECK(sequence >= 0), CHECK(fence >= 0)
);
CREATE TABLE IF NOT EXISTS successor_history(
  operation_id TEXT PRIMARY KEY,
  domain TEXT NOT NULL REFERENCES uniqueness_domains(domain),
  predecessor_sequence INTEGER NOT NULL,
  new_sequence INTEGER NOT NULL,
  new_fence INTEGER NOT NULL,
  UNIQUE(domain,new_sequence),
  CHECK(new_sequence = predecessor_sequence + 1)
);
CREATE TRIGGER IF NOT EXISTS history_no_update BEFORE UPDATE ON successor_history BEGIN SELECT RAISE(ABORT,'APPEND_ONLY_HISTORY'); END;
CREATE TRIGGER IF NOT EXISTS history_no_delete BEFORE DELETE ON successor_history BEGIN SELECT RAISE(ABORT,'APPEND_ONLY_HISTORY'); END;
";

#[derive(Debug, Clone)]
pub struct GlobalUniquenessAdapter {
    store: SqliteConcurrencyStore,
}
impl GlobalUniquenessAdapter {
    pub fn new(store: SqliteConcurrencyStore) -> Self {
        Self { store }
    }
}

impl FoundationalContracts for GlobalUniquenessAdapter {
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
    fn enforce_global_uniqueness(&self, input: &BehavioralInput) -> ContractResult {
        let data = &input.scenario_data;
        let result = self.store.cas(
            &input_digest(input),
            data["uniqueness_domain"].as_str().unwrap_or(""),
            data["writer_sequence"].as_i64().unwrap_or(-1),
            data["writer_fence"].as_i64().unwrap_or(-1),
            data["expected_sequence"].as_i64().unwrap_or(-1),
            data["current_fence"].as_i64().unwrap_or(-1),
        );
        Ok(ObservedBehavior {
            input_digest: input_digest(input),
            facts: json!({
                "accepted": result.disposition == ConcurrencyDisposition::Committed,
                "stored_sequence": result.stored_sequence,
                "stored_fence": result.stored_fence,
                "successor_created": result.successor_created,
                "classification": format!("{:?}",result.disposition),
                "database_cas": true,
                "database_engine": "sqlite",
                "database_error_code": result.database_error.as_ref().and_then(|e|e.exit_code),
                "database_stderr": result.database_error.as_ref().map(|e|e.stderr.as_str()),
            }),
        })
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

pub fn infrastructure_available() -> bool {
    Path::new(SQLITE).is_file() && sqlite_available()
}
