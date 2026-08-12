# Engagement and Lifecycle Schemas

## Engagement Instance Binding

Required fields:

- `engagement_binding_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`, `legal_entity_id`;
- `engagement_id`, `Layer_03_engagement_ref_and_version`, `Layer_09_association_ref_and_version`;
- `client_security_domain`, `engagement_security_subdomain`, `purpose`, `scope_ref`;
- `classification_ref`, `confidentiality_state`, `binding_status`, `engagement_status_reference`;
- `jurisdiction_refs`, `conflict_review_refs`, `access_policy_ref`, `context_manifest_refs`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `effective_from`, `effective_until`;
- `source_refs`, `provenance_refs`, `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

The binding grants no access and cannot create or change Layer 03 scope, lifecycle, gates, blockers, or approval.

## Client Lifecycle Transition

Required fields:

- `lifecycle_transition_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id` where applicable, `client_security_domain`, `purpose`;
- `classification_ref`, `confidentiality_state`, `prior_lifecycle_status`, `proposed_lifecycle_status`, `transition_status`;
- `exact_prior_record_ref_and_version`, `transition_reason`, `dependency_refs`, `canonical_decision_refs`;
- `effective_at`, `freshness_status`, `verified_or_assessed_at`, `review_due`;
- `source_refs`, `provenance_refs`, `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Unknown, invalid, or dependency-incomplete transitions are `LIFECYCLE_BLOCKED`. The transition does not change Layer 03 or Layer 09 state.

