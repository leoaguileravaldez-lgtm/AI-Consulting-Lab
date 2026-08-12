# Information and Material Schemas

## Client Information Asset Record

Required fields:

- `information_asset_id`, `schema_version`, `record_version`, `information_class`;
- `client_instance_id`, `client_id`, `legal_entity_id`, `engagement_id`, `client_security_domain`, `engagement_security_subdomain`;
- `purpose`, `classification_ref`, `confidentiality_state`, `ownership_custody_status`, `asset_status`;
- `authoritative_location_ref`, `original_identifier_or_hash_ref`, `provider_ref`, `provider_authority_ref`;
- `received_or_observed_at`, `license_contract_restriction_refs`, `jurisdiction_refs`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `valid_from`, `valid_until`;
- `source_refs`, `provenance_refs`, `transformation_refs`, `contradiction_refs`, `limitations`;
- `reuse_eligibility`, `retention_state_ref`, `invariant_status`, `invariant_validation_ref`;
- `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Raw client content and credentials are excluded. Asset completeness does not establish factual or evidence validity.

## Client Assertion Record

Required fields:

- `client_assertion_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `engagement_security_subdomain`;
- `purpose`, `classification_ref`, `confidentiality_state`, `assertion_status`, `assertion_type`;
- `assertion_ref`, `speaker_or_provider_ref`, `authority_claim_ref`, `received_or_observed_at`;
- `Layer_04_intake_ref`, `evidence_status_ref`, `factual_reliance_status`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Until Layer 04 acts, `factual_reliance_status` is `UNVERIFIED_NOT_EVIDENCE`.

## Client Material Manifest

Required fields:

- `material_manifest_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `engagement_security_subdomain`;
- `purpose`, `classification_ref`, `confidentiality_state`, `material_status`, `material_type`;
- `asset_refs_and_versions`, `provider_ref`, `provider_authority_ref`, `authoritative_location_refs`;
- `ownership_custody_refs`, `license_contract_restriction_refs`, `jurisdiction_refs`, `content_instruction_trust_status`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Embedded instructions are always `UNTRUSTED_FOR_AUTHORITY`.

