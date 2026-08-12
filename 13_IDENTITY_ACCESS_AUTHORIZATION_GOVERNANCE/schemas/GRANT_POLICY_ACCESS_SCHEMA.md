# Grant, Policy, Access, Purpose, and Boundary Schemas

All eight objects incorporate and physically expand every common field under `MATERIAL_OBJECT_INVARIANT.md`; the fields below are additional mandatory fields.

## Authority Grant

`authority_grant_id`, `authority_category`, `beneficiary_identity_principal_effective_actor`, `source_authority_object_id_version`, `grantor_identity_ref`, `grantor_effective_actor_id`, `grantor_beneficiary_effective_actor_comparison`, `governed_determination_scope`, `object_action_scope`, `delegability`, `conditions`, `mandatory_SOD_result`, `SOD_validation_ref`, `grant_status`, and `source_authority_reconciliation_ref`. Grantor and beneficiary cannot collapse where Material. It cannot exceed or change the source category.

## Permission Grant

`permission_grant_id`, `beneficiary_identity_principal_effective_actor`, `grantor_identity_ref`, `grantor_effective_actor_id`, `grantor_beneficiary_effective_actor_comparison`, `source_permission_authority_ref`, `permitted_operation_classes`, `object_refs_versions_or_set`, `boundary_refs`, `purpose_refs`, `confidentiality_constraints`, `conditions`, `mandatory_SOD_result`, `SOD_validation_ref`, `grant_status`, and `attempt_only_limitation`. Grantor and beneficiary cannot collapse where Material. Permission is not determination authority.

## Access Policy

`access_policy_id`, `policy_owner_layer`, `policy_type`, `subject_rules`, `effective_actor_rules`, `object_version_rules`, `operation_rules`, `purpose_rules`, `boundary_rules`, `confidentiality_rules`, `required_role_permission_authority_refs`, `SOD_rules`, `time_rules`, `decision_logic`, `priority_conflict_rule`, and `default_result`. `default_result` must be `DENY`.

## Access Request

`access_request_id`, `access_request_version`, `access_request_hash`, `requester_actor_id_version_hash`, `requester_principal_id_version_hash`, `requester_effective_actor_id_version_hash`, `originating_actor_id_version_hash`, `originating_principal_id_version_hash`, `originating_effective_actor_id_version_hash`, `originating_authority_ref`, `declared_deputy_chain_entries`, `declared_deputy_count`, `declared_deputy_chain_manifest_hash`, `validated_empty_deputy_chain_ref`, `declared_final_executing_actor_id_version_hash`, `declared_final_executing_principal_id_version_hash`, `declared_final_executing_effective_actor_id_version_hash`, `zero_deputy_identity_equality_manifest`, `requested_object_refs_versions_hashes`, `requested_operations`, `requested_purpose_ref`, `requested_jurisdiction_refs_versions_hashes`, `requested_boundaries`, `requested_confidentiality_scope`, `requested_duration`, `role_grant_context_refs`, `applicable_constraint_refs_versions_hashes`, `reason`, and `request_state`.

Every declared chain entry contains its ordinal, predecessor and successor, exact identity/principal/effective-actor ID/version/hash, roles, permissions, authorities, delegation source, object/action scope, purpose, jurisdiction, client/entity/engagement boundaries, confidentiality, lifecycle, freshness, validity, and applicable constraints. For direct access, `declared_deputy_count` is exactly zero, `declared_deputy_chain_entries` is explicitly empty, and `validated_empty_deputy_chain_ref` plus `zero_deputy_identity_equality_manifest` prove exact equality among requester, originator, final executor, effective actor, and the actor submitted for authoritative authorization. A service, agent, model, machine, human, alias, proxy, shared account, or intermediary mismatch invalidates direct access. Omission never means zero. A request grants no access.

## Access Decision

`access_decision_id`, `access_request_id_version_hash`, `authoritative_authorization_decision_id`, `authoritative_authorization_decision_version`, `authoritative_authorization_decision_hash`, `request_authorization_comparison_manifest_hash`, `authorization_state_snapshot`, `authorization_authority_snapshot_ref`, `authorization_actor_principal_effective_actor_snapshot`, `authorization_deputy_chain_manifest_hash`, `authorization_input_manifest_hash`, `authorization_scope_snapshot`, `authorization_client_entity_engagement_snapshot`, `authorization_jurisdiction_snapshot`, `authorization_purpose_snapshot`, `authorization_confidentiality_snapshot`, `authorization_lifecycle_freshness_snapshot`, `authorization_validity_window_snapshot`, `authorization_revocation_supersession_snapshot`, `zero_deputy_identity_equality_manifest_hash`, `access_state_projection`, `access_scope_projection`, `conditions_projection`, `denial_reasons_projection`, `effective_at_projection`, `expires_at_projection`, `revocation_status_projection`, `authorization_access_reconciliation_status`, `authorization_access_reconciliation_ref`, `authorization_access_reconciled_at`, and `enforcement_limitation`.

Access Decision is a non-authoritative immutable projection and cannot independently choose a state. Every projection field must equal the same-version authoritative Authorization Decision or be a provably narrower non-permissive projection. `APPROVED`, `CONDITIONALLY_APPROVED`, `DENIED`, `REVOKED`, `EXPIRED`, `SUPERSEDED`, and `REVALIDATION_REQUIRED` must match exactly; no mapping may become more permissive. Missing, stale, hash-mismatched, scope/boundary/purpose/confidentiality-mismatched, revoked, expired, superseded, or unreconciled Authorization Decision makes the Access Decision `REVALIDATION_REQUIRED` and non-actionable. It represents but does not enforce access.

## Purpose-of-Use Record

`purpose_of_use_id`, `purpose_category`, `purpose_description`, `permitted_object_action_scope`, `prohibited_secondary_uses`, `boundary_scope`, `confidentiality_scope`, `source_authority_ref`, `purpose_status`, and `downstream_revalidation_required`. Purpose A never authorizes Purpose B.

## Access Context

`access_context_id`, `access_request_id_version_hash`, `authorization_decision_id_version_hash`, `request_authorization_comparison_manifest_hash`, `originating_actor_principal_effective_actor_refs_versions_hashes`, `ordered_deputy_principal_effective_actor_refs_versions_hashes`, `final_executing_principal_effective_actor_refs_versions_hashes`, `deputy_chain_count`, `deputy_chain_manifest_hash`, `validated_empty_deputy_chain_ref`, `zero_deputy_identity_equality_manifest_hash`, `identity_resolution_refs_for_every_chain_member`, `authentication_assurance_refs_for_every_chain_member`, `session_context_refs`, `role_assignment_refs`, `permission_grant_refs`, `authority_grant_refs`, `policy_refs`, `boundary_refs`, `purpose_ref`, `confidentiality_constraint_refs`, `SOD_ref`, `object_action_manifest`, `effective_authorization_intersection_ref`, `context_status`, and `context_reconciliation_ref`. Direct contexts require an explicitly validated-empty deputy chain and exact canonical equality of requester, originator, final executor, effective actor, and Authorization Decision actor. Any mismatch invalidates the context.

## Client Access Boundary

`client_access_boundary_id`, `client_instance_id`, `client_id`, `client_security_domain`, `permitted_principal_refs`, `object_scope`, `operation_scope`, `purpose_scope`, `cross_client_rule`, and `boundary_status`. `cross_client_rule` defaults to deny.
