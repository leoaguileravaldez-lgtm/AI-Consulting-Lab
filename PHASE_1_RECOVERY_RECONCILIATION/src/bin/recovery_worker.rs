use std::{
    env,
    process::{self, Command},
};
fn end(code: i32, s: &str) -> ! {
    if code == 0 {
        println!("{s}")
    } else {
        eprintln!("{s}")
    }
    process::exit(code)
}
fn main() {
    let Some(db) = env::args().nth(1) else {
        end(22, "MISSING_DB")
    };
    let q = "SELECT c.sequence||'|'||j.sequence||'|'||c.state_hash||'|'||j.predecessor_hash||'|'||j.outcome||'|'||j.retry_authorized||'|'||COALESCE(t.operation_id,'')||'|'||COALESCE(t.predecessor_sequence,-1)||'|'||COALESCE(t.new_sequence,-1)||'|'||COALESCE(t.provenance,'') FROM checkpoints c JOIN journal j ON j.sequence=c.sequence+1 LEFT JOIN transitions t ON j.outcome='COMMITTED' ORDER BY j.sequence DESC LIMIT 1;";
    let o = match Command::new("/usr/bin/sqlite3")
        .arg("-batch")
        .arg("-bail")
        .arg(db)
        .arg(q)
        .output()
    {
        Ok(v) => v,
        Err(_) => end(22, "DATABASE_PROCESS"),
    };
    if !o.status.success() {
        end(21, "DATABASE_EXECUTION")
    };
    let row = String::from_utf8_lossy(&o.stdout);
    let p: Vec<_> = row.trim().split('|').collect();
    if p.len() != 10 {
        end(22, "MISSING_DURABLE_LINEAGE")
    };
    let checkpoint = p[0];
    let tail = p[1];
    let predecessor = p[2] == p[3];
    let unknown = p[4] == "UNKNOWN";
    let retry = p[5] == "1";
    let exact_transition = p[6] == "committed-op"
        && p[7].parse::<i64>().ok() == p[1].parse::<i64>().ok()
        && p[7].parse::<i64>().ok() == Some(21)
        && p[8].parse::<i64>().ok() == p[1].parse::<i64>().ok().map(|v| v + 1)
        && p[8].parse::<i64>().ok() == Some(22)
        && p[9] == "synthetic-hash-21";
    if unknown && !retry && predecessor {
        end(
            0,
            &format!("RECONCILIATION_REQUIRED|0|1|1|{checkpoint}|{tail}|UNKNOWN||||"),
        )
    }
    if p[4] == "COMMITTED" && predecessor && exact_transition {
        end(
            0,
            &format!(
                "RECOVERED_COMMITTED_STATE|0|1|0|{checkpoint}|{tail}|COMMITTED|{}|{}|{}|{}",
                p[6], p[7], p[8], p[9]
            ),
        )
    }
    if p[4] == "COMMITTED" {
        end(23, "COMMITTED_TRANSITION_MISMATCH")
    }
    end(22, "INDETERMINATE_INTERNAL_STATE")
}
