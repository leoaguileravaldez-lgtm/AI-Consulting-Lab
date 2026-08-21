use std::{fs, path::PathBuf};
use titus_lab_integration_stage_1_kernel::{
    DeterministicLocalExecutor, IntegrationKernel, construct_stage1_candidate_package,
    synthetic_request, synthetic_root,
};

fn main() {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root")
        .to_path_buf();
    let runtime_root = synthetic_root("candidate-package");
    let _ = fs::remove_dir_all(&runtime_root);
    let kernel = IntegrationKernel::create(
        &runtime_root,
        "synthetic-client-A",
        1,
        DeterministicLocalExecutor,
    )
    .expect("kernel setup");
    let result = kernel.execute(&synthetic_request());
    let package = construct_stage1_candidate_package(&repository_root, &result)
        .expect("candidate package");
    println!("{}", serde_json::to_string(&package).expect("package serialization"));
    fs::remove_dir_all(runtime_root).expect("cleanup");
}
