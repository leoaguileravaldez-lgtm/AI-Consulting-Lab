# Applicability Assessment Schema

## Required fields

- `applicability_assessment_id`, `schema_version`, `record_version`;
- `receiving_engagement_id`, `knowledge_id`, `knowledge_version`, `intended_use`;
- `jurisdiction_match`, `sector_subsector_match`, `scope_match`, `size_population_match`;
- `period_match`, `currency_basis_match`, `methodology_compatibility`;
- `freshness_status`, `source_status`, `client_boundary_status`;
- `professional_review_status`, `contradiction_status`, `supersession_status`;
- `dimension_differences`, `limitations`, `result`;
- `assessor_identity`, `independence_ref`, `assessed_at`, `audit_refs`.

Results are `APPLICABLE`, `APPLICABLE_WITH_NON_MATERIAL_LIMITATIONS`, `REVIEW_REQUIRED`, `NOT_APPLICABLE` or `UNKNOWN`. They are advisory. `UNKNOWN` and any unresolved Material difference prohibit clean reuse.

