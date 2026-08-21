# TITUS LAB Phase 1 Empirical Certification

This bounded non-layer workspace implements only `BC_EMPIRICAL_CERTIFICATION`. Its subject is the exact local Phase 1 candidate evidence package bound to repository `AI-Consulting-Lab`, commit `8364cc3570cca11692e638b6cf3022a47fa9e752`, frozen contract `FUTURE_EMPIRICAL_CERTIFICATION_CONTRACT+S`, certified predecessors, environment manifest, and fixed Material results.

`independent_validator` is a separate executable and process. It imports no Phase 1 producer crate or producer verdict function. It reconstructs the frozen contract from the certified Layer 16 source, checks exact evidence files and hashes, derives expectations independently, and treats any Material failure as `CERTIFIED_FAIL` regardless of aggregate score or implementation-declared expectation. The bounded trust root is the certified Layer 16 source hash, exact candidate commit, fixed predecessor evidence manifest, separate validator executable, and Human Principal control of closure.

The Phase 0 adapter computes `input_digest` as SHA-256 over the exact JSON serialization of frozen `scenario_data`, not over a debug representation of the complete behavioral input. For the frozen material-failure case, the evidence package and independent validator preserve the behavioral facts `accepted=false`, `material_failure_preserved=true`, and `deployment_authority_created=false`; `CERTIFIED_FAIL` is derived from those facts and does not substitute for them.

Verdicts are evidence only. They create no execution, retry, recovery, Layer 19 operational, deployment, release, regulatory, or production authority. This does not certify TITUS LAB generally, production readiness, distributed correctness, compliance, deployment, or Operational Beta readiness.
