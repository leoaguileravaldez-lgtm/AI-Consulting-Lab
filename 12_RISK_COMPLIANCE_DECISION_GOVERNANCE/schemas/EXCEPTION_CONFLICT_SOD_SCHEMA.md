# Exception, Conflict, and Segregation Schemas

All five records incorporate every common field under the normative Schema Incorporation Contract in `MATERIAL_OBJECT_INVARIANT.md`. Those fields must be physically present and validated in each concrete record; this reference is not a substitute field or default.

## Exception Request

`exception_request_id`, `requirement_or_control_ref`, `requester_identity`, `requester_effective_actor_id`, `effective_actor_resolution_ref`, `requested_deviation`, `reason`, `alternatives`, `affected_scope`, `requested_start`, `requested_expiration`, `risk_refs`, `compensating_controls`, `monitoring_plan`, `dependencies`, `conflict_refs`, `reversibility`, `closure_plan`, `non_waivable_check`, `mandatory_SOD_rule_refs`, and `request_status`. A request cannot satisfy its own approval requirement or become decision approval.

## Exception Decision

`exception_decision_id`, `exception_request_id_version`, `disposition`, `decision_authority_identity`, `decision_authority_effective_actor_id`, `effective_actor_resolution_ref`, `decision_authority_ref`, `Human_Principal_decision_ref` where required, `qualified_review_refs`, `approved_scope`, `conditions`, `effective_at`, `expires_at`, `monitoring_requirements`, `revocation_triggers`, `residual_risk_ref`, `mandatory_SOD_rules_result`, `SOD_validation_ref`, `non_waivable_validation_ref`, and `closure_status`. The requester and decision authority must be distinct effective actors. Approval does not create precedent, satisfy another decision's approval chain, or create broader authority.

## Waiver Record

`waiver_record_id`, `waiver_requester_identity`, `waiver_requester_effective_actor_id`, `waiver_approval_identity`, `waiver_approval_effective_actor_id`, `requester_and_approver_resolution_refs`, `permitted_waiver_basis`, `waived_requirement_exact_ref`, `exception_decision_id_version`, `waiver_approval_requirement_ref`, `waiver_approval_record_ref`, `waiver_scope`, `beneficiaries`, `conditions`, `effective_at`, `expires_at`, `monitoring`, `reversibility`, `revocation_triggers`, `mandatory_SOD_rules_result`, `SOD_validation_ref`, `non_precedential_status`, and `non_waivable_validation_ref`. Requester and approver must be distinct effective actors. A waiver cannot approve itself, satisfy its own approval, enter another decision's approval set, or waive mandatory SOD, Human Principal authority, provenance, isolation, confidentiality, auditability, mandatory legal/contractual constraints, or evidence authority. Unauthorized or expanded waivers are invalid.

## Conflict Record

`conflict_record_id`, `conflict_type`, `position_refs_versions`, `source_authorities`, `affected_scope`, `materiality`, `conflict_status`, `detected_by_at`, `owner`, `conflict_subject_effective_actor_refs`, `resolution_authority_effective_actor_refs`, `effective_actor_resolution_refs`, `recusal_refs`, `containment`, `required_reviewers`, `mandatory_SOD_rules_result`, `SOD_validation_ref`, `escalation_ref`, `resolution_requirements`, and `canonical_resolution_ref`. A conflict subject cannot be the sole resolution authority. Material unresolved, stale, or SOD-invalid conflict blocks the affected gate.

## Segregation-of-Duties Constraint

`sod_constraint_id`, `mandatory_baseline_rule_set_version`, `mandatory_rule_ids`, `mandatory_rule_count`, `mandatory_rule_applicability_results`, `mandatory_rule_result`, `additional_incompatible_role_action_pairs`, `scope`, `materiality`, `minimum_independent_reviewers`, `effective_actor_refs`, `effective_actor_resolution_refs`, `identity_aliases`, `account_aliases`, `agent_model_delegation_refs`, `delegation_chain_refs`, `common_control_checks`, `recusal_rules`, `delegation_rules`, `substitute_authority_rules`, `validation_method`, `violation_status`, `violation_refs`, and `resolution_authority`. For Material work, `mandatory_baseline_rule_set_version` is `L12-MATERIAL-SOD-BASELINE-v1`, count is eight, and IDs equal exactly `MSOD-01` through `MSOD-08`, each with an applicability/result. The mandatory baseline cannot be omitted, disabled, configured empty, overridden, or waived; configurable rules only add restrictions. Self, reciprocal, circular, aliased, delegated, model/agent-mediated, or shared-control approval fails closed. Unknown effective identity is `SOD_BLOCKED`.
