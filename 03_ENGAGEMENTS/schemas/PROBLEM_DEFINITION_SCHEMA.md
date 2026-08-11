# Problem Definition Schema

## Required Fields

- `problem_definition_id`, `engagement_id`, `version`, `status`, `created_at`, `created_by`, `audit_ref`.
- `request`: exact request, requestor, date, intended use and provenance.
- `observed_conditions[]`: labeled observations with evidence references and confidence.
- `root_problem_hypotheses[]`: hypothesis, assumptions, supporting/contrary evidence and falsification test.
- `decision_to_support`: exact decision, authorized decision-maker, deadline, alternatives and consequences.
- `success_criteria[]`: criterion, measurement applicability/reference and decision relevance.
- `out_of_scope[]`: exclusions, rationale and boundary owner.
- `constraints[]`, `known_risks[]`, `unknowns[]`, `stakeholder_and_incentive_notes[]`.
- `requested_deliverables[]` and `deliverable_decision_fit`: `FIT`, `FIT_WITH_CONDITIONS`, or `FIT_GAP`, with rationale.
- `source_refs`, `approval_refs`, `change_record_refs`, `supersedes_version`.

## Rules

Request, observation, hypothesis and decision are never merged. An observation is not proof of a root cause. A preferred thesis is attributed and tested against alternatives. Material `FIT_GAP`, missing authority, incomplete scope or unresolved conflict prevents analytical readiness through canonical controls.

Any Material edit creates a change record, increments the version and invalidates dependent planning/readiness views. Prior versions remain reconstructable. The Human Principal approves the exact Material problem/scope through `01`; this schema does not approve itself.
