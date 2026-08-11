# QA Finding Schema

## Required fields

- `qa_finding_id`, `schema_version`, `record_version`, `engagement_id`, `qa_assessment_id`;
- `canonical_control_ref`, `target_object_ref`, `target_version`, `materiality`;
- `expected_condition`, `observed_condition`, `cause`, `impact`, `uncertainty`;
- `supporting_control_evidence_refs`, `affected_dependency_refs`;
- `risk_owner_ref`, `remediation_owner_ref`, `qa_verifier_ref`;
- `recommended_response`, `status`, `remediation_evidence_refs`, `verification_result`;
- `exception_ref`, `recurrence_refs`, `impact_notice_ref`;
- `created_by`, `created_at`, `independence_ref`, `audit_refs`, `superseded_by`.

## Rules

Finding statuses are internal assurance-record labels, never canonical blocker states. `VERIFIED` means only that remediation evidence passed the specified independent QA test. `ACCEPTED_EXCEPTION` requires a current exact canonical exception and does not imply risk acceptance. Findings, failed remediation, pressure, and adverse outcomes remain immutable and versioned.

