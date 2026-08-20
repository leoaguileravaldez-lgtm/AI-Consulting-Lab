use std::{
    env, fs,
    path::{Component, Path},
    process::{self, Command},
};

fn finish(code: i32, message: &str) -> ! {
    if code == 0 {
        println!("{message}");
    } else {
        eprintln!("{message}");
    }
    process::exit(code)
}
fn valid_atom(s: &str) -> bool {
    !s.is_empty()
        && Path::new(s)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}
fn quote(s: &str) -> String {
    s.replace('\'', "''")
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 6 {
        finish(12, "INVALID_REQUEST_SHAPE");
    }
    let db = match env::var("AILAB_AUTHORITY_DB") {
        Ok(v) => v,
        Err(_) => finish(20, "MISSING_TRUSTED_AUTHORITY_DB"),
    };
    let root = match env::var("AILAB_RESOURCE_ROOT") {
        Ok(v) => v,
        Err(_) => finish(20, "MISSING_TRUSTED_RESOURCE_ROOT"),
    };
    let (token, workload, actor, attestation, object_client, object_name) =
        (&args[0], &args[1], &args[2], &args[3], &args[4], &args[5]);
    if !valid_atom(workload) || !valid_atom(object_client) || !valid_atom(object_name) {
        finish(12, "RESOURCE_SUBSTITUTION_REJECTED");
    }
    let sql = format!(
        "SELECT workload_client||'|'||actor||'|'||attestation||'|'||status FROM capabilities WHERE token='{}';",
        quote(token)
    );
    let out = match Command::new("/usr/bin/sqlite3")
        .arg("-batch")
        .arg("-bail")
        .arg(&db)
        .arg(sql)
        .output()
    {
        Ok(v) => v,
        Err(_) => finish(21, "AUTHORITY_DATABASE_PROCESS_ERROR"),
    };
    if !out.status.success() {
        finish(21, "AUTHORITY_DATABASE_EXECUTION_ERROR");
    }
    let row = String::from_utf8_lossy(&out.stdout);
    let fields: Vec<&str> = row.trim().split('|').collect();
    if fields.len() != 4
        || fields[0] != workload
        || fields[1] != actor
        || fields[2] != attestation
        || fields[3] != "ACTIVE"
        || actor != "MACHINE"
        || attestation != "CURRENT"
    {
        finish(11, "AUTHORITY_OR_IDENTITY_REJECTED");
    }
    if workload != object_client {
        finish(10, "FOREIGN_CONTEXT_REJECTED");
    }
    let canonical_root = match fs::canonicalize(&root) {
        Ok(v) => v,
        Err(_) => finish(20, "RESOURCE_ROOT_ERROR"),
    };
    let domain_root = match fs::canonicalize(canonical_root.join(object_client)) {
        Ok(v) => v,
        Err(_) => finish(20, "DOMAIN_RESOURCE_ERROR"),
    };
    if !domain_root.starts_with(&canonical_root) {
        finish(12, "RESOURCE_BOUNDARY_REJECTED");
    }
    let manifest = match fs::read_to_string(domain_root.join(".domain")) {
        Ok(v) => v,
        Err(_) => finish(20, "DOMAIN_MANIFEST_ERROR"),
    };
    if manifest.trim() != object_client {
        finish(12, "RESOURCE_IDENTITY_MISMATCH");
    }
    let target = match fs::canonicalize(domain_root.join(object_name)) {
        Ok(v) => v,
        Err(_) => finish(20, "RESOURCE_ACCESS_ERROR"),
    };
    if !target.starts_with(&domain_root) {
        finish(12, "RESOURCE_SUBSTITUTION_REJECTED");
    }
    match fs::read_to_string(target) {
        Ok(v) => finish(0, v.trim()),
        Err(_) => finish(20, "RESOURCE_READ_ERROR"),
    }
}
