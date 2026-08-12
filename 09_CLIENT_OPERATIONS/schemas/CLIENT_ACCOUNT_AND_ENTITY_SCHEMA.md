# Client Account and Entity Schema

## Client Account Record

Required fields:

- `client_id`, `schema_version`, `record_version`, `display_name`, `identity_status`;
- `client_security_domain`, `purpose`, `classification_ref`, `confidentiality_profile_ref`;
- `canonical_name_source_ref`, `aliases`, `external_identifier_refs`;
- `parent_relationship_refs`, `legal_entity_refs`, `duplicate_case_refs`;
- `operational_status`, `status_basis_ref`, `valid_from`, `valid_until`, `freshness_status`, `verified_or_assessed_at`, `review_due`;
- `conflict_refs`, `restriction_refs`, `contradiction_refs`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

`external_identifier_refs` are aliases only. Missing or ambiguous identity yields `IDENTITY_REVIEW` or `QUARANTINED`, never an inferred match.

## Legal Entity Record

Required fields:

- `legal_entity_id`, `client_id`, `client_security_domain`, `schema_version`, `record_version`;
- `purpose`, `classification_ref`, `confidentiality_profile_ref`;
- `legal_name`, `entity_type`, `formation_jurisdiction`, `operating_jurisdictions`;
- `authoritative_identifier_refs`, `registered_address_ref`, `entity_status`, `freshness_status`;
- `effective_from`, `effective_until`, `verification_source_refs`, `verified_by`, `verified_or_assessed_at`, `review_due`;
- `predecessor_refs`, `successor_refs`, `relationship_refs`, `engagement_association_refs`;
- `conflict_refs`, `contradiction_refs`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Sensitive identifiers are referenced from approved systems rather than copied into Markdown or ordinary audit content.

## Identity Resolution Case

Required fields:

- `identity_case_id`, `schema_version`, `record_version`, `case_type`, `purpose`;
- `client_security_domain`, `classification_ref`, `confidentiality_profile_ref`;
- `candidate_client_ids`, `candidate_legal_entity_ids`, `match_indicators`, `contradictory_indicators`;
- `quarantine_scope`, `affected_engagement_refs`, `confidentiality_impact`, `conflict_impact`;
- `resolution_status`, `resolution_outcome`, `freshness_status`, `verified_or_assessed_at`, `review_due`;
- `source_refs`, `human_decision_ref`, `field_provenance_map_ref`, `limitations`;
- `survivor_or_new_id_refs`, `edge_remapping_ref`, `rollback_reconstruction_ref`;
- `schema_invariant_status`, `schema_validation_ref`, `opened_by`, `opened_at`, `resolved_by`, `resolved_at`, `audit_refs`.

Automatic merge or split is prohibited. `UNRESOLVED` blocks Material use.
