# Confidentiality, Conflict, Reuse, and Decision Schemas

## Confidentiality Profile

Required fields:

- `confidentiality_profile_id`, `schema_version`, `record_version`, `client_id`, `client_security_domain`;
- `legal_entity_scope_refs`, `engagement_scope_refs`, `classification_ref`, `restriction_refs`;
- `purpose_limits`, `distribution_limits`, `jurisdiction_limits`, `retention_ref`;
- `source_refs`, `effective_from`, `effective_until`, `freshness_status`, `verified_or_assessed_at`, `review_due`, `status`;
- `downgrade_from_ref`, `downgrade_basis_ref`, `human_decision_ref` where applicable;
- `contradiction_refs`, `limitations`, `schema_invariant_status`, `schema_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Unknown or conflicting classification retains the highest plausible restriction. The profile grants no access.

## Conflict Record

Required fields:

- `conflict_id`, `schema_version`, `record_version`, `conflict_type`, `conflict_status`;
- `party_refs`, `client_ids`, `legal_entity_ids`, `engagement_scope_refs`, `client_security_domains`, `affected_purpose`;
- `discovery_source_ref`, `discovered_at`, `factual_basis`, `uncertainties`;
- `classification_ref`, `information_barrier_ref`, `required_review_refs`, `disposition_ref`;
- `effective_from`, `freshness_status`, `verified_or_assessed_at`, `review_due`, `expires_at`;
- `contradiction_refs`, `limitations`, `schema_invariant_status`, `schema_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Layer 09 cannot assess, accept, waive, or close Material conflicts without the competent canonical disposition.

## Reuse Authorization Request

Required fields:

- `reuse_request_id`, `schema_version`, `record_version`, `client_security_domains`;
- `origin_client_id`, `origin_legal_entity_id`, `origin_engagement_id`, `origin_object_ref`, `origin_object_version`;
- `destination_client_id`, `destination_engagement_id`, `exact_purpose`, `proposed_transformation_ref`;
- `classification_ref`, `permission_basis_ref`, `deidentification_review_ref`, `generalization_review_ref`;
- `Layer_08_eligibility_ref`, `applicability_status`, `applicability_review_ref`, `conflict_clearance_ref`, `human_decision_ref`;
- `valid_from`, `valid_until`, `freshness_status`, `verified_or_assessed_at`, `review_due`, `conditions`, `revocation_refs`, `request_status`, `contradiction_refs`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Missing or unknown gate means `NOT_AUTHORIZED`.

## Client State Decision

Required fields:

- `client_state_decision_id`, `schema_version`, `record_version`, `decision_category`;
- `client_id`, `legal_entity_id`, `engagement_id`, `client_security_domain`, `classification_ref`, `purpose`;
- `request_ref`, `exact_object_refs_and_versions`, `permitted_options`, `decision`;
- `scope`, `conditions`, `contradictions_considered`, `risk_and_review_refs`, `rationale_ref`;
- `decision_maker_ref`, `Layer_01_authority_ref`, `decided_at`, `effective_from`, `expires_at`;
- `decision_status`, `freshness_status`, `verified_or_assessed_at`, `review_due`, `limitations`;
- `revocation_status`, `revocation_ref`, `proxy_prohibited`, `schema_invariant_status`, `schema_validation_ref`;
- `created_by`, `created_at`, `audit_refs`.

Silence, defaults, templates, models, agents, commercial roles, and prior decisions cannot populate a valid decision.
