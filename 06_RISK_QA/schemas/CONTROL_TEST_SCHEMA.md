# Control Test Schema

## Required fields

- `control_test_id`, `schema_version`, `record_version`, `engagement_id`, `qa_assessment_id`;
- `canonical_control_ref`, `control_requirement_version`, `control_owner`, `control_operator`;
- `applicability`, `applicability_rationale`, `materiality`;
- `target_object_ref`, `target_version`, `test_as_of`;
- `design_criteria`, `operation_criteria`, `evidence_requirements`;
- `test_method`, `sample_scope`, `inputs`, `procedures`, `calculations`, `exclusions`;
- `evidence_refs`, `exception_ref`, `result`, `limitations`, `residual_exposure`;
- `tester_identity`, `independence_ref`, `created_at`, `audit_refs`.

## Rules

Permitted results are `SATISFIED`, `PARTIALLY_SATISFIED`, `FAILED`, `NOT_TESTED`, `NOT_APPLICABLE`, and `UNKNOWN`. `NOT_APPLICABLE` requires an authoritative applicability rule and exact-scope rationale. Missing evidence cannot produce `SATISFIED`. A test result is control evidence, not canonical readiness, approval, risk acceptance, or state.

