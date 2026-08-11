# QA Assessment Schema

## Required fields

- `qa_assessment_id`, `schema_version`, `record_version`, `engagement_id`, `task_id`;
- `authorized_scope_ref`, `target_object_ref`, `target_version`, `target_hash` where available;
- `canonical_state_ref`, `canonical_blocker_refs`, `approval_refs`, `exception_refs`;
- `evidence_state_manifest_ref`, `challenge_state_manifest_ref`, `dissent_refs`;
- `control_manifest_ref`, `professional_review_manifest_ref`;
- `assessor_identity`, `role_assignment_ref`, `session_model_ref`, `independence_assessment_ref`;
- `methodology_id`, `methodology_version`, `assessment_as_of`, `created_at`;
- `control_test_refs`, `finding_refs`, `residual_risk_refs`, `limitations`;
- `veto_conditions`, `qa_result`, `dependency_manifest_ref`, `impact_notice_ref`, `audit_refs`;
- `superseded_by`, `revalidation_reason` where applicable.

## Validity rules

`qa_result` is an advisory record value only. It cannot change state, resolve blockers, approve, accept risk, or authorize release. `READY_FOR_CANONICAL_EVALUATION` requires no active Material/Critical veto and complete current bindings. `READY_WITH_NON_MATERIAL_LIMITATIONS` cannot contain a Material issue. Missing or inconsistent required fields produce `NOT_ASSESSABLE`.

Updates append versions. Material dependency change yields `SUPERSEDED` or `REVALIDATION_REQUIRED`; it cannot silently inherit the prior result.

