# Engagement Association Schema

## Engagement Association Record

Required fields:

- `association_id`, `schema_version`, `record_version`;
- `client_id`, `legal_entity_id`, `engagement_id`, `client_security_domain`;
- `relationship_role`, `purpose`, `scope_ref`, `Layer_03_engagement_version_ref`;
- `classification_ref`, `confidentiality_profile_ref`, `conflict_clearance_ref`;
- `authorization_ref`, `effective_from`, `effective_until`, `association_status`;
- `scope_change_refs`, `jurisdiction_refs`, `stakeholder_refs`, `contradiction_refs`;
- `source_refs`, `freshness_status`, `verified_or_assessed_at`, `review_due`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

The association references but cannot create or change the Layer 03 engagement. Missing, foreign, stale, contradictory, or unauthorized identifiers block association and use. An association grants no access and transfers no evidence, conclusions, knowledge, or permissions.
