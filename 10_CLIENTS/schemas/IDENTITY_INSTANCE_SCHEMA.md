# Identity and Instance Schemas

## Client Instance Record

Required fields:

- `client_instance_id`, `schema_version`, `record_version`, `client_id`, `Layer_09_client_record_ref_and_version`;
- `client_security_domain`, `purpose`, `classification_ref`, `confidentiality_state`, `confidentiality_profile_ref`;
- `instance_lifecycle_status`, `identity_status`, `legal_entity_binding_refs`, `engagement_binding_refs`;
- `jurisdiction_refs`, `ownership_refs`, `conflict_review_refs`, `access_policy_ref`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `valid_from`, `valid_until`;
- `source_refs`, `provenance_refs`, `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Missing, ambiguous, duplicated, foreign, or stale Layer 09 identity prevents `VALIDATED_CURRENT`.

## Legal Entity Binding

Required fields:

- `entity_binding_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`, `legal_entity_id`;
- `Layer_09_entity_record_ref_and_version`, `client_security_domain`, `purpose`, `classification_ref`, `confidentiality_state`;
- `binding_status`, `entity_status`, `relationship_role`, `jurisdiction_refs`, `ownership_refs`, `related_party_edge_refs`;
- `engagement_scope_refs`, `conflict_review_refs`, `effective_from`, `effective_until`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Layer 10 cannot resolve entity identity or infer a binding from aliases, ownership, or CRM hierarchy.

## Related-Party Edge

Required fields:

- `related_party_edge_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `source_legal_entity_id`, `target_legal_entity_id`, `client_security_domain`, `purpose`;
- `relationship_type`, `relationship_status`, `classification_ref`, `confidentiality_state`;
- `Layer_09_relationship_ref_and_version`, `jurisdiction_refs`, `conflict_review_refs`, `authorization_effect`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `valid_from`, `valid_until`;
- `source_refs`, `provenance_refs`, `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

`authorization_effect` must be `NONE`; relationship never transfers access, confidentiality, engagement, evidence, or reuse authority.

