# Identity, Actor, Principal, and Role Schemas

All eight objects incorporate and physically expand every common field under `MATERIAL_OBJECT_INVARIANT.md`; the fields below are additional mandatory fields.

## Identity Record

`identity_id`, `identity_class`, `canonical_name`, `presented_identifier_refs`, `identity_owner`, `identity_lifecycle_state`, `employment_contract_client_relationship_refs`, `termination_suspension_compromise_status`, `resolution_status`, and `duplicate_candidate_refs`. Unknown or ambiguous identity is blocked.

## Identity Resolution Record

`identity_resolution_id`, `presented_identifiers`, `candidate_identity_refs_versions`, `resolved_identity_id_version`, `resolved_principal_id_version`, `effective_actor_id`, `resolution_method`, `verification_refs`, `alias_account_delegation_edges`, `common_control_refs`, `contradictions`, `resolution_status`, and `resolution_confidence_limitations`. Material use requires deterministic `RESOLVED_CURRENT`; confidence alone cannot resolve ambiguity.

## Identity Alias Record

`identity_alias_id`, `alias_type`, `alias_value_or_reference`, `canonical_identity_id_version`, `effective_actor_id`, `source`, `valid_from`, `valid_to`, `rename_predecessor_ref`, `shared_alias_status`, `collision_refs`, and `alias_status`. Alias possession grants nothing.

## Identity Verification Reference

`identity_verification_ref_id`, `identity_id_version`, `verification_method_ref`, `issuing_or_verifying_authority`, `verification_scope`, `verification_result`, `verified_at`, `expires_at`, `revocation_status`, `evidence_reference`, and `assurance_limitations`. This stores no credential or authentication artifact.

## Actor Record

`actor_id`, `identity_id_version`, `principal_id_version`, `effective_actor_id`, `actor_type`, `originating_actor_principal_ref`, `acting_on_behalf_of_ref`, `immediate_predecessor_actor_principal_ref`, `controller_ref`, `delegation_source_ref`, `chain_sequence`, `role_assignment_refs`, `actor_lifecycle_state`, and `attribution_status`. Material deputy use requires complete ordered-chain participation; an isolated acting-on-behalf-of label grants nothing.

## Principal Record

`principal_id`, `principal_type`, `identity_or_machine_identity_ref`, `effective_actor_id`, `principal_owner`, `principal_lifecycle_state`, `eligible_identity_classes`, `canonical_authority_refs`, and `principal_resolution_ref`.

## Role Definition

`role_definition_id`, `role_name`, `role_kind`, `canonical_owner_layer`, `role_purpose`, `eligible_identity_classes`, `permitted_assignment_scopes`, `prohibited_authority_inferences`, `required_qualifications_refs`, `SOD_rule_refs`, and `role_lifecycle_state`. Role definition confers no grant.

## Role Assignment

`role_assignment_id`, `role_definition_id_version`, `assignee_identity_id_version`, `assignee_principal_id_version`, `assignee_effective_actor_id`, `assigner_identity_ref`, `assigner_effective_actor_id`, `assigner_authority_ref`, `assignee_assigner_effective_actor_comparison`, `assignment_scope`, `assignment_purpose`, `qualification_refs`, `mandatory_SOD_result`, `assignment_status`, `permission_grant_refs`, and `authority_grant_refs`. Self-assignment cannot create privilege. Assignment alone grants neither permission nor authority.
