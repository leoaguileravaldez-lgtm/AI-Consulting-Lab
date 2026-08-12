# Context, Derivative, and Export Schemas

## Context Boundary Manifest

Required fields:

- `context_manifest_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `engagement_security_subdomain`;
- `purpose`, `classification_ref`, `confidentiality_state`, `context_status`;
- `allowed_object_types`, `allowed_object_refs_and_versions`, `excluded_object_refs`, `classification_ceiling`;
- `time_window`, `authorization_refs`, `access_policy_ref`, `prompt_context_restrictions`, `model_memory_status`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Foreign, mixed, stale, unclassified, or unlisted content is excluded and quarantined.

## Retrieval Result Manifest

Required fields:

- `retrieval_manifest_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `engagement_security_subdomain`;
- `purpose`, `classification_ref`, `confidentiality_state`, `retrieval_status`;
- `context_manifest_ref_and_version`, `query_purpose_ref`, `result_object_refs_and_versions`, `excluded_result_refs`;
- `boundary_check_status`, `ranking_non_authority`, `source_independence_status`, `authorization_refs`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Retrieval does not establish truth, validation, applicability, permission, or authority.

## Derived Artifact Manifest

Required fields:

- `derived_artifact_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `engagement_security_subdomain`;
- `purpose`, `classification_ref`, `confidentiality_state`, `artifact_status`, `artifact_type`;
- `origin_refs_and_versions`, `transformation_description`, `producer_ref`, `tool_description_ref`;
- `evidence_reference_status`, `invalidation_dependency_refs`, `deliverable_ref` where applicable;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Transformation cannot reduce confidentiality, improve evidence status, or authorize reuse.

## Export Manifest

Required fields:

- `export_manifest_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `engagement_security_subdomain`;
- `purpose`, `classification_ref`, `confidentiality_state`, `export_status`;
- `artifact_refs_and_versions`, `proposed_recipient_refs`, `proposed_destination_ref`, `minimum_necessary_review_ref`;
- `Layer_07_readiness_ref`, `Layer_01_approval_ref`, `external_action_status`, `authorization_refs`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `created_by`, `created_at`, `audit_refs`.

`external_action_status` is always `NOT_AUTHORIZED_BY_LAYER_10`; this manifest cannot export, transfer, release, deliver, or publish.

