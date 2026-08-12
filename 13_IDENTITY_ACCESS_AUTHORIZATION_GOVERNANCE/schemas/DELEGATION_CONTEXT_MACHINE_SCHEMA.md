# Delegation, Privilege, Constraint, Machine, and Context Schemas

All twelve objects incorporate and physically expand every common field under `MATERIAL_OBJECT_INVARIANT.md`; the fields below are additional mandatory fields.

## Delegation Record

`delegation_id`, `delegator_identity_principal_effective_actor`, `delegate_identity_principal_effective_actor`, `originating_principal_ref`, `immediate_predecessor_delegation_ref`, `chain_sequence`, `delegation_approver_identity_effective_actor`, `delegate_approver_effective_actor_comparison`, `source_authority_or_permission_ref`, `delegated_subset`, `object_action_scope`, `boundary_scope`, `purpose_scope`, `confidentiality_scope`, `delegation_depth`, `transitivity_state`, `conditions`, `mandatory_SOD_result`, `SOD_validation_ref`, `delegation_status`, and `chain_reconciliation_ref`. Every link is exact and cannot exceed the intersection inherited from all predecessors. Default transitivity is `NON_TRANSITIVE`; the delegate cannot be sole approver.

## Delegation Revocation

`delegation_revocation_id`, `delegation_id_version`, `revoker_identity_effective_actor_authority`, `revocation_reason`, `revoked_at`, `affected_descendant_delegation_refs`, `affected_authorization_refs`, `propagation_status`, and `reconciliation_ref`.

## Privilege Elevation Request

`privilege_elevation_request_id`, `requester_identity_principal_effective_actor`, `requested_privilege`, `purpose_ref`, `object_action_scope`, `boundary_scope`, `reason`, `requested_start`, `requested_expiry`, `alternatives`, `risk_refs`, `SOD_rule_refs`, and `request_status`.

## Privilege Elevation Decision

`privilege_elevation_decision_id`, `request_id_version`, `decision_authority_identity_effective_actor`, `authority_ref`, `decision_state`, `approved_scope`, `conditions`, `effective_at`, `expires_at`, `revocation_triggers`, `requester_approver_effective_actor_comparison`, `SOD_validation_ref`, `post_expiry_status`, and `audit_review_ref`. Self-approval or missing expiry denies elevation.

## Engagement Access Boundary

`engagement_access_boundary_id`, `client_boundary_ref`, `engagement_id`, `engagement_security_subdomain`, `engagement_lifecycle_ref`, `permitted_principal_refs`, `object_action_scope`, `purpose_scope`, `cross_engagement_rule`, and `boundary_status`. Cross-engagement defaults deny.

## Legal-Entity Access Boundary

`legal_entity_access_boundary_id`, `client_boundary_ref`, `legal_entity_id`, `entity_relationship_refs`, `entity_resolution_ref`, `object_action_scope`, `purpose_scope`, `related_party_rule`, and `boundary_status`. Relationship alone grants nothing.

## Jurisdiction Access Constraint

`jurisdiction_access_constraint_id`, `jurisdiction_codes`, `legal_entity_and_engagement_refs`, `applicability_ref`, `permitted_and_prohibited_operations`, `data_location_or_transfer_constraints`, `qualified_review_ref`, `conflict_refs`, and `constraint_status`.

## Confidentiality Access Constraint

`confidentiality_access_constraint_id`, `canonical_classification_ref`, `classification_level`, `object_scope`, `minimum_permission_authority_assurance`, `purpose_restrictions`, `boundary_restrictions`, `downgrade_authority_ref`, `conflict_refs`, and `constraint_status`. Implicit downgrade is prohibited.

## Segregation-of-Duties Access Constraint

`sod_access_constraint_id`, `layer12_baseline_version`, `layer12_rule_ids`, `layer13_baseline_version`, `layer13_rule_ids`, `mandatory_rule_counts`, `applicability_results`, `effective_actor_refs`, `identity_alias_common_control_refs`, `additional_rules`, `SOD_result`, and `validation_ref`. Material validation requires Layer 12 version `L12-MATERIAL-SOD-BASELINE-v1` with exactly `MSOD-01`–`MSOD-08`, and Layer 13 version `L13-MATERIAL-ACCESS-SOD-BASELINE-v1` with exactly `ASOD-01`–`ASOD-08`; both counts are eight and every rule has an applicability/result. Exact mandatory sets cannot be empty, partial, omitted, disabled, or waived.

## Service/Agent Identity Record

`machine_identity_id`, `machine_identity_class`, `owner_identity_ref`, `controller_effective_actor_id`, `authorizer_identity_effective_actor_ref`, `controller_authorizer_effective_actor_comparison`, `service_model_agent_reference`, `permitted_role_refs`, `object_action_scope`, `boundary_scope`, `purpose_scope`, `allowed_originating_principal_classes`, `acting_on_behalf_of_chain_required`, `caller_deputy_intersection_required`, `mandatory_SOD_result`, `human_impersonation_prohibited`, `self_authorization_prohibited`, and `machine_identity_status`. Every Material invocation preserves and authorizes the originator and all deputies; service authority alone is insufficient. The machine/controller cannot be its own authorizer. Contains no credential or runtime instance.

## Session/Context Reference

`session_context_ref_id`, `identity_principal_effective_actor_refs`, `authentication_assurance_ref`, `originating_authority_ref`, `context_purpose`, `object_action_scope`, `boundary_scope`, `issued_or_observed_at`, `expires_at`, `revocation_status`, `context_integrity_ref`, and `context_limitations`. Contains no session artifact.

## Authentication Assurance Reference

`authentication_assurance_ref_id`, `identity_principal_effective_actor_refs`, `assurance_state`, `method_reference`, `issuing_authority`, `verified_at`, `assessed_at`, `scope`, `expires_at`, `revocation_status`, `freshness_state`, and `assurance_limitations`. Assurance never grants authority.
