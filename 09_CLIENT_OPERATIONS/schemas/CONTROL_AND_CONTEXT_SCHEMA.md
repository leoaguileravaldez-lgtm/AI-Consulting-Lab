# Control and Context Schemas

## Operational Context Packet

Required fields:

- `context_packet_id`, `schema_version`, `record_version`, `packet_status`;
- `client_id`, `legal_entity_id`, `engagement_id`, `client_security_domain`, `destination_boundary_ref`;
- `purpose`, `classification_ref`, `source_object_refs_and_versions`, `source_schema_invariant_statuses`, `source_validation_refs`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `contradiction_refs`, `limitations`, `prohibited_interpretations`;
- `authorization_ref` where required, `schema_invariant_status`, `schema_validation_ref`;
- `created_by`, `created_at`, `audit_refs`.

Only exact-version `VALIDATED_CURRENT` sources may enter a packet. A packet is advisory and cannot create evidence, authority, permission, approval, or action.

## Operational Contradiction Record

Required fields:

- `operational_contradiction_id`, `schema_version`, `record_version`, `contradiction_status`;
- `client_ids`, `legal_entity_ids`, `engagement_ids`, `client_security_domains`, `purpose`;
- `classification_ref`, `assertion_refs_and_versions`, `source_refs`, `difference_description`;
- `materiality_proposal`, `affected_dependency_refs`, `owner_ref`, `escalation_refs`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Unresolved Material contradiction is `CONTRADICTED_BLOCKED`; recency, rank, preference, CRM state, or commercial value cannot choose a winner.

## Pressure Event Record

Required fields:

- `pressure_event_id`, `schema_version`, `record_version`, `pressure_event_status`;
- `client_id`, `legal_entity_id`, `engagement_id`, `client_security_domain`, `purpose`;
- `classification_ref`, `actor_ref`, `occurred_at`, `request_or_faithful_reference`, `affected_object_refs_and_versions`;
- `pressure_type`, `response`, `escalation_refs`, `source_ref`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `contradiction_refs`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Pressure state cannot alter evidence, analysis, dissent, risk, QA, professional judgment, limitations, or upstream history.

## Change Impact Notice

Required fields:

- `change_impact_notice_id`, `schema_version`, `record_version`, `notice_status`;
- `client_id`, `legal_entity_id`, `engagement_ids`, `client_security_domain`, `purpose`;
- `classification_ref`, `changed_object_ref_and_version`, `change_type`, `source_ref`;
- `affected_Layer_09_refs`, `affected_certified_layer_refs`, `required_owner_refs`, `acknowledgement_refs`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `contradiction_refs`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `created_by`, `created_at`, `audit_refs`.

The notice routes potential impact only. Certified owners determine revalidation or invalidation; Layer 09 cannot mutate their records.

## Audit Event Record

Required fields:

- `audit_event_id`, `schema_version`, `record_version`, `audit_event_status`;
- `client_id`, `legal_entity_id`, `engagement_id`, `client_security_domain`, `purpose`;
- `classification_ref`, `actor_ref`, `actor_role`, `occurred_at`, `action`, `exact_object_ref_and_version`;
- `basis`, `outcome`, `reason`, `authorization_ref`, `correlation_ref`, `source_event_ref`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `contradiction_refs`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `created_by`, `created_at`, `audit_chain_refs`.

Audit events are append-only design records and contain no credentials. Missing or incomplete audit lineage blocks the affected Material object; Layer 09 cannot alter or delete audit history.

