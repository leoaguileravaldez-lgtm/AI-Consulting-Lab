# Deliverable Review Schema

## Required fields

- `deliverable_review_id`, `schema_version`, `record_version`, `engagement_id`;
- `presentation_artifact_id`, `artifact_hash`, `content_version_ref`, `presentation_version_ref`;
- `review_scope`, `reviewer_identity`, `role_assignment_ref`, `independence_ref`;
- `lineage_result`, `semantic_fidelity_result`, `material_omission_result`;
- `numerical_chart_result`, `citation_result`, `risk_dissent_professional_result`;
- `segregation_confidentiality_result`, `hidden_content_result`, `multiformat_result`;
- `claim_strength_result`, `external_action_boundary_result`;
- `material_veto_conditions`, `limitations`, `advisory_result`;
- `canonical_impact_notice_ref`, `created_at`, `audit_refs`.

Advisory results are `DELIVERABLE_PREPARED`, `DELIVERABLE_REVIEWED`, `DELIVERABLE_INTERNALLY_CONSISTENT`, `NOT_PRESENTATION_READY`, `SEMANTIC_REVIEW_REQUIRED`, `SECURITY_REVIEW_REQUIRED` or `REVALIDATION_REQUIRED`. They are never canonical state, approval, release, blocker resolution, risk acceptance, analytical validation or delivery authorization. Any active Material veto prohibits a clean result.

