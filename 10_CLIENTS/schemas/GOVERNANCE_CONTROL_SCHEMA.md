# Governance Control Schemas

## Retention State Record

Required fields:

- `retention_state_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `purpose`;
- `classification_ref`, `confidentiality_state`, `retention_status`, `lifecycle_status`;
- `governing_rule_refs`, `jurisdiction_refs`, `legal_hold_refs`, `contractual_hold_refs`, `dependency_inventory_ref`;
- `retention_start`, `review_due`, `proposed_end`, `freshness_status`, `verified_or_assessed_at`;
- `source_refs`, `provenance_refs`, `contradiction_refs`, `limitations`, `reuse_eligibility`;
- `invariant_status`, `invariant_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Missing or conflicting retention authority blocks disposition. Retention status never executes deletion.

## Disposition Request

Required fields:

- `disposition_request_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `purpose`;
- `classification_ref`, `confidentiality_state`, `disposition_status`, `lifecycle_status`;
- `exact_object_refs_and_versions`, `copy_derivative_export_cache_index_embedding_backup_inventory_ref`;
- `retention_state_refs`, `hold_clearance_refs`, `governing_rule_refs`, `jurisdiction_review_refs`;
- `proposed_method`, `reversibility_status`, `required_approval_refs`, `proposed_executor_ref`, `verification_plan_ref`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `invariant_status`, `invariant_validation_ref`;
- `created_by`, `created_at`, `audit_refs`.

The request is non-executing and cannot prove disposition. Unknown copy/hold scope is `LIFECYCLE_BLOCKED`.

## Knowledge Extraction Request

Required fields:

- `knowledge_extraction_request_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `purpose`;
- `classification_ref`, `confidentiality_state`, `request_status`, `lifecycle_status`;
- `origin_object_refs_and_versions`, `proposed_transformation`, `proposed_destination_client_and_engagement_refs`;
- `ownership_permission_refs`, `deidentification_review_ref`, `reidentification_risk_status`, `generalization_review_ref`;
- `Layer_08_applicability_ref`, `Layer_08_reuse_eligibility_ref`, `conflict_clearance_ref`, `Human_Principal_decision_ref`;
- `Layer_04_destination_validation_ref`, `valid_from`, `valid_until`, `revocation_refs`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Missing or unknown gate means `REUSE_BLOCKED`. De-identification alone never changes that result.

## Conflict and Related-Party Review Record

Required fields:

- `instance_conflict_review_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_ids`, `engagement_ids`, `client_security_domain`, `purpose`;
- `classification_ref`, `confidentiality_state`, `review_status`, `lifecycle_status`;
- `Layer_09_conflict_refs_and_versions`, `Layer_06_disposition_refs_and_versions`, `related_party_edge_refs`;
- `party_refs`, `affected_activity_types`, `information_barrier_refs`, `jurisdiction_refs`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `expires_at`;
- `source_refs`, `provenance_refs`, `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Layer 10 cannot assess, accept, waive, remediate, or close the canonical conflict.

## Client Instance Decision Record

Required fields:

- `client_instance_decision_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `purpose`;
- `classification_ref`, `confidentiality_state`, `decision_status`, `lifecycle_status`;
- `decision_category`, `request_ref`, `exact_object_refs_and_versions`, `permitted_options`, `decision`;
- `conditions`, `risk_conflict_review_refs`, `contradictions_considered`, `rationale_ref`;
- `decision_maker_ref`, `Layer_01_authority_ref`, `decided_at`, `effective_from`, `expires_at`, `revocation_ref`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `limitations`, `reuse_eligibility`, `retention_state_ref`, `proxy_prohibited`;
- `invariant_status`, `invariant_validation_ref`, `created_by`, `created_at`, `audit_refs`.

Silence, defaults, models, agents, client instructions, CRM roles, timeouts, and prior decisions cannot populate a valid decision.

## Client Instance Audit Event

Required fields:

- `client_instance_audit_event_id`, `schema_version`, `record_version`, `client_instance_id`, `client_id`;
- `legal_entity_id`, `engagement_id`, `client_security_domain`, `purpose`;
- `classification_ref`, `confidentiality_state`, `audit_event_status`, `lifecycle_status`;
- `actor_ref`, `actor_role`, `occurred_at`, `action`, `exact_object_ref_and_version`;
- `basis`, `authorization_ref`, `outcome`, `reason`, `correlation_ref`, `source_event_ref`;
- `freshness_status`, `verified_or_assessed_at`, `review_due`, `source_refs`, `provenance_refs`;
- `contradiction_refs`, `limitations`, `reuse_eligibility`, `retention_state_ref`;
- `invariant_status`, `invariant_validation_ref`, `created_by`, `created_at`, `audit_chain_refs`.

Audit events are append-only design records and contain no credentials. Incomplete audit lineage blocks the affected Material object.

