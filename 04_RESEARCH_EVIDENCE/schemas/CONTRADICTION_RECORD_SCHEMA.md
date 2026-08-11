# Contradiction Record Schema

## Required Fields

- `contradiction_id`, `schema_version`, `record_version`, `engagement_id`, `task_id`, `created_at`, `created_by`, `audit_ref`.
- `competing_claim_refs` with at least two positions and `competing_evidence_refs`.
- `definition_differences`, `period_differences`, `population_differences`, `jurisdiction_differences`, `unit_differences`.
- `quality_comparison`, `lineage_comparison`, `method_comparison`, `incentive_conflict_comparison`.
- `severity`, `materiality`, `unresolved_uncertainty`, `resolution_attempts`.
- `affected_analysis_refs`, `recommendation_refs`, `decision_packet_refs`, `approval_refs`, `deliverable_refs`, `revalidation_refs`.
- `confidence_impact_ref`, `owner`, `validator_ref`, `challenge_ref`, `canonical_blocker_ref`.
- `disposition`: `RESOLVED`, `ACCEPTED_NON_MATERIAL_LIMITATION`, or `UNRESOLVED`.
- `resolution_rationale`, `human_principal_visibility_ref`, `prior_version_ref`.

## Rules

`UNRESOLVED` remains nonterminal and blocking where Material under canonical controls. Only a genuinely non-Material limitation may be accepted as such. Human Principal judgment cannot validate unsupported evidence or erase conflict.

No role may delete, average, summarize out, soften or overwrite competing positions. Resolution creates a new version, preserves every prior position and propagates dependency impacts. `04` records and notifies; `01_ORCHESTRATOR` owns any canonical blocker or transition.
