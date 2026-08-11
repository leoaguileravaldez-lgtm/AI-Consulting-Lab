# Specialist Mandate Schema

## Required Fields

Every practice mandate contains:

```text
practice_id
component_class
mandate_version
status
owner
approval_reference
mission
scope_in
scope_out
core_questions
required_inputs
permitted_evidence
analytical_methods
expected_outputs
confidence_requirements
materiality_triggers
mandatory_escalation_triggers
dependencies
prohibited_actions
explicit_non_authorities
conflict_of_interest_risks
correlated_reasoning_risks
required_audit_trail
failure_stop_conditions
human_principal_approval_dependencies
orchestrator_interface
research_evidence_interface
independent_challenge_interface
risk_qa_interface
deliverables_interface
qualification_overlays
effective_from
expires_at
```

## Validation Rules

- `practice_id` matches the certified registry taxonomy.
- `component_class` is `PRIMARY_PRACTICE` for practice mandates.
- Scope and non-authority cannot conflict.
- No mandate creates a state, role, tier, approval, exception, external-action, or registry authority.
- Materiality and confidence cannot be weaker than `00_CORE`.
- Assurance interfaces cannot place independent controls inside the practice.
- Missing mandatory fields block mandate approval or activation.
- Changes create a new version and require dependency/control impact review.
