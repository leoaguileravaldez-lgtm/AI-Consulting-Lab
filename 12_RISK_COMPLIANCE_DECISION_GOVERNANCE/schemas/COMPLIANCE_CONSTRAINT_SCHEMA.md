# Compliance and Constraint Schemas

All five records incorporate every common field under the normative Schema Incorporation Contract in `MATERIAL_OBJECT_INVARIANT.md`. Those fields must be physically present and validated in each concrete record; this reference is not a substitute field or default.

## Compliance Obligation

`compliance_obligation_id`, `obligation_type`, `source_instrument_id_version`, `source_provision`, `issuing_authority`, `regulated_legal_entity`, `applicable_activities_data_products`, `jurisdiction_scope`, `applicability_test`, `mandatory_status`, `interpretation_authority_ref`, `qualified_review_ref`, `required_actions_controls`, `prohibitions`, `reporting_duties`, `retention_duties`, `exception_permitted`, and `conflict_refs`.

## Compliance Assessment

`compliance_assessment_id`, `assessment_scope`, `obligation_refs_versions`, `constraint_refs_versions`, `applicability_results`, `control_refs`, `evidence_refs`, `gap_inventory`, `uncertainties`, `conflict_refs`, `remediation_owner`, `remediation_due`, `assessment_method_version`, `assessor_identity`, `assessor_effective_actor_id`, `effective_actor_resolution_ref`, `assessor_qualification`, `override_authority_effective_actor_refs`, `mandatory_SOD_rules_result`, `SOD_validation_ref`, and `conclusion_status`. The assessor cannot be the sole compliance-override authority. Absence of a known gap cannot produce `COMPLIANT`; incomplete scope or unknown effective identity is `UNKNOWN_BLOCKED`.

## Jurisdiction Constraint

`jurisdiction_constraint_id`, `jurisdiction_code`, `legal_entity_id`, `territorial_nexus`, `source_authority_instrument_version`, `applicability_basis`, `required_conditions`, `prohibited_conditions`, `qualified_interpretation_ref`, `hierarchy`, and `conflict_refs`.

## Policy Constraint

`policy_constraint_id`, `policy_owner_layer`, `policy_object_id_version_hash`, `applicability_scope`, `requirement`, `prohibition`, `exception_rule_ref`, `hierarchy`, `interpretation_owner`, and `conflict_refs`. Layer 12 cannot amend policy.

## Contractual Constraint

`contractual_constraint_id`, `contract_id_version`, `contracting_legal_entities`, `provision_ref`, `effective_term`, `applicable_engagement_activity_data`, `requirement`, `prohibition`, `consent_or_notice_requirement`, `interpretation_authority_ref`, `amendment_refs`, and `conflict_refs`.
