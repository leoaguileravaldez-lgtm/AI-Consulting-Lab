# Residual Risk Assessment Schema

## Required fields

- `residual_risk_assessment_id`, `schema_version`, `record_version`, `engagement_id`;
- `canonical_risk_record_ref`, `target_object_ref`, `target_version`, `assessment_as_of`;
- `canonical_category_refs`, `cause`, `likelihood_ref`, `impact_ref`, `inherent_risk_ref`;
- `control_test_refs`, `control_effectiveness`, `mitigation_refs`, `mitigation_status`;
- `residual_risk_ref`, `uncertainty`, `interaction_refs`, `affected_decision_refs`;
- `evidence_strength_ref`, `analytical_confidence_ref`, `challenge_status_ref`, `qa_status_ref`;
- `professional_review_requirement_ref`, `exception_ref`;
- `risk_owner_ref`, `mitigation_owner_ref`, `qa_verifier_ref`, `risk_acceptance_authority_ref`;
- `monitoring`, `triggers`, `limitations`, `created_at`, `audit_refs`.

## Rules

All ratings reference canonical definitions. Unknown control effectiveness receives no assumed credit. No aggregate may replace individual dimensions or conceal Material/Critical exposure. QA records risk; only the exact canonical authority may accept it.

