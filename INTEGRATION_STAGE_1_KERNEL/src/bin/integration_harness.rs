use std::fs;
use titus_lab_integration_stage_1_kernel::{
    DeterministicLocalExecutor, IntegrationKernel, synthetic_request, synthetic_root,
};

fn main() {
    let root = synthetic_root("harness");
    let _ = fs::remove_dir_all(&root);
    let kernel =
        IntegrationKernel::create(&root, "synthetic-client-A", 1, DeterministicLocalExecutor)
            .expect("kernel setup");
    let result = kernel.execute(&synthetic_request());
    println!("{result:?}");
    let _ = fs::remove_dir_all(root);
}
