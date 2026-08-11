# Knowledge Record Schema

## Required fields

- `knowledge_id`, `schema_version`, `record_version`, `knowledge_type`, `title`, `summary`;
- `domain`, `sector`, `jurisdiction`, `applicability_scope`;
- `origin_type`, `originating_engagement_ref`, `client_boundary`, `classification_ref`;
- `deidentification_ref`, `generalization_ref`, `reuse_policy`, `reuse_authorization_ref`;
- `methodology_id`, `methodology_version` where applicable;
- `source_lineage_refs`, `evidence_refs`, `challenge_refs`, `qa_refs`, `professional_review_refs`;
- `freshness_status`, `freshness_as_of`, `review_due`, `valid_from`, `valid_until`;
- `limitations`, `assumptions`, `known_exceptions`, `contradiction_refs`;
- `knowledge_record_status`, `supersedes`, `superseded_by`, `distribution_class_ref`;
- `created_by`, `reviewed_by`, `independence_ref`, `created_at`, `audit_refs`.

Status and distribution fields are metadata only. Missing reuse authorization or unknown eligibility means non-reusable. Source/evidence, confidence, permission and canonical state remain owned upstream.

