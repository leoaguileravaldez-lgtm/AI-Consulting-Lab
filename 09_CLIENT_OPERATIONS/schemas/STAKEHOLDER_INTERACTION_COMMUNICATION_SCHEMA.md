# Stakeholder, Interaction, and Communication Schemas

## Contact/Stakeholder Record

Required fields:

- `stakeholder_id`, `schema_version`, `record_version`, `client_id`, `legal_entity_id`, `client_security_domain`;
- `purpose`, `stakeholder_status`;
- `engagement_scope_refs`, `display_name_ref`, `contact_endpoint_refs`, `role_types`;
- `declared_authority`, `authority_basis_ref`, `authority_scope`, `authority_valid_from`, `authority_valid_until`;
- `relationship_status`, `source_ref`, `freshness_status`, `verified_or_assessed_at`, `review_due`;
- `classification_ref`, `confidentiality_restrictions`, `conflict_refs`, `contradiction_refs`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Contact endpoints should be referenced from approved systems. Declared authority is not Layer 01 approval authority.

## Interaction Record

Required fields:

- `interaction_id`, `schema_version`, `record_version`, `client_id`, `legal_entity_id`, `engagement_id` where applicable, `client_security_domain`;
- `occurred_at`, `participant_refs`, `channel_descriptor`, `purpose`, `classification_ref`;
- `source_ref`, `factual_summary`, `claimed_commitments`, `follow_up_proposals`;
- `interaction_status`, `freshness_status`, `verified_or_assessed_at`, `review_due`;
- `correction_of`, `contradiction_refs`, `limitations`, `schema_invariant_status`, `schema_validation_ref`;
- `created_by`, `created_at`, `audit_refs`.

Claims and commitments remain unvalidated until processed by their canonical owners.

## Communication Record

Required fields:

- `communication_id`, `schema_version`, `record_version`, `client_id`, `legal_entity_id`, `engagement_id` where applicable, `client_security_domain`;
- `sender_ref`, `recipient_refs`, `recipient_authority_status`, `purpose`, `channel_descriptor`;
- `classification_ref`, `distribution_scope`, `content_ref`, `retention_rule_ref`;
- `sent_or_received_claim`, `delivery_status_ref`, `response_status`, `occurred_at`;
- `communication_status`, `freshness_status`, `verified_or_assessed_at`, `review_due`;
- `authorization_ref`, `source_ref`, `contradiction_refs`, `limitations`, `schema_invariant_status`, `schema_validation_ref`;
- `created_by`, `created_at`, `audit_refs`.

The record cannot send a communication or establish delivery, consent, approval, or acceptance.

## Preference Record

Required fields:

- `preference_id`, `schema_version`, `record_version`, `client_id`, `legal_entity_id`, `client_security_domain`, `owner_ref`;
- `purpose`, `engagement_scope_refs`, `preference_type`, `preference_value_ref`, `source_ref`;
- `effective_from`, `effective_until`, `priority`, `conflict_refs`, `freshness_status`, `verified_or_assessed_at`, `review_due`;
- `non_analytical_constraint`, `classification_ref`, `status`, `limitations`;
- `contradiction_refs`, `schema_invariant_status`, `schema_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

`non_analytical_constraint` must affirm that the preference cannot alter certified truth, assurance, or authority.
