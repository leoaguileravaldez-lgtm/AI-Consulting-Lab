# Deliverable Content Lineage Schema

## Required fields

- `lineage_entry_id`, `schema_version`, `record_version`, `engagement_id`;
- `presentation_artifact_id`, `content_version_ref`;
- `statement_id`, `statement_text`, `rendered_location`, `materiality`;
- `transformation_type`, `canonical_claim_or_recommendation_ref`;
- `analysis_refs`, `evidence_relation_refs`, `source_refs`;
- `challenge_refs`, `dissent_refs`, `qa_refs`, `risk_refs`;
- `professional_review_refs`, `human_disposition_refs`;
- `quantitative_object_ref`, `field_cell_series_ref`, `units`, `currency`, `basis`, `period`, `scenario`, `transformation`, `rounding_rule` where applicable;
- `limitations`, `semantic_comparison_ref`, `created_at`, `created_by`, `audit_refs`.

Every Material statement requires complete applicable lineage. A narrower upstream proposition cannot support broader rendered language. The artifact is never a source of truth.

