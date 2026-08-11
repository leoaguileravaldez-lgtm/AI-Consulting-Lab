# Challenge Assignment Schema

## Required Fields

- `assignment_projection_id`, `engagement_id`, `canonical_task_id`, `canonical_assignment_ref`, `assignment_version`.
- `target_object_id`, `target_version_hash`, `scope_in`, `scope_out`, `materiality`, `methods_required`.
- `challenger_actor_id`, `session_id`, `model_provider_version`, `role_assignment_ref`.
- `producer_actor_session_refs`, `researcher_refs`, `validator_refs`, `prior_participation_refs`.
- `context_manifest_ref`, `context_exposure_sequence`, `source_selection_plan`, `sealed_assumption_inventory_ref`.
- `method_independence`, `conflict_incentive_attestation`, `reporting_path`, `qualification_refs`.
- `permissions`, `data_class`, `jurisdictions`, `stop_conditions`, `accepted_at`, `audit_refs`.

## Rules

This record projects a canonical `01` assignment and cannot create one. Only `01_ORCHESTRATOR` assigns the challenger and grants access.

Same producer/challenger actor or session fails. Unknown context, shared hidden reasoning, curated-only evidence, producer authorship, unmitigated incentive conflict, inadequate qualification or missing attestation fails closed. A different role label cannot alter the result.

Acceptance confirms bounded assignment only; it is not validation, approval or permission expansion.
