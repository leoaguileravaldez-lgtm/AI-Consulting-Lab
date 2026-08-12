# Risk Schemas

All six records incorporate every common field under the normative Schema Incorporation Contract in `MATERIAL_OBJECT_INVARIANT.md`. Those fields must be physically present and validated in each concrete record; this reference is not a substitute field or default. The following fields are additional and mandatory unless an exact validated `NOT_APPLICABLE` is permitted.

## Risk Record

`risk_id`, `risk_type`, `risk_title`, `risk_statement`, `cause_refs`, `event_refs`, `consequence_refs`, `threatened_objective_refs`, `affected_scope`, `detection_method`, `source_refs`, `evidence_refs`, `analysis_refs`, `uncertainty`, `risk_owner`, `risk_lifecycle_state`, and `closure_criteria`.

## Risk Assessment

`risk_assessment_id`, `risk_id`, `risk_record_version`, `assessment_method_id_version`, `assessor_identity_role_qualification`, `assessor_effective_actor_id`, `effective_actor_resolution_ref`, `assessment_date`, `time_horizon`, `severity`, `likelihood`, `impact_dimensions`, `velocity`, `detectability`, `materiality`, `assumptions`, `sensitivity`, `contradiction_refs`, `certified_input_refs_versions_hashes`, `mandatory_SOD_rule_refs`, and `assessment_conclusion`.

## Risk Classification

`risk_classification_id`, `risk_id`, `taxonomy_id_version`, `primary_category`, `secondary_categories`, `severity_class`, `materiality_class`, `classification_basis`, `classifier`, `independent_confirmation_ref` where required, and `classification_change_history`.

## Risk Exposure

`risk_exposure_id`, `risk_id`, `measurement_method_version`, `units`, `point_or_range`, `lower_bound`, `upper_bound`, `currency_and_date` where applicable, `population_scope`, `time_horizon`, `scenario_refs`, `correlation_assumptions`, `concentration_refs`, and `uncertainty`.

## Risk Control Reference

`risk_control_ref_id`, `risk_id`, `control_owner_layer`, `control_object_id_version_hash`, `response_type`, `coverage`, `design_status`, `implementation_status`, `operating_period`, `effectiveness_status`, `test_or_assurance_ref`, `control_dependencies`, `failure_modes`, and `credit_allowed`. Planned, untested, stale, or failed controls receive no residual-risk reduction credit.

## Residual Risk Assessment

`residual_risk_assessment_id`, `risk_id`, `inherent_assessment_id_version`, `credited_control_refs_versions`, `control_effectiveness_refs`, `remaining_exposure`, `residual_severity`, `residual_likelihood`, `residual_impact`, `uncertainty`, `open_conflict_and_dissent_refs`, `treatment_status`, `risk_acceptance_requirement_ref`, `risk_acceptance_decision_ref`, `risk_acceptance_effective_actor_id`, `assessor_acceptor_effective_actor_comparison`, `mandatory_SOD_rules_result`, and `SOD_validation_ref`. Missing acceptance authority, unknown effective actor, or assessor/acceptor identity match blocks disposition.
