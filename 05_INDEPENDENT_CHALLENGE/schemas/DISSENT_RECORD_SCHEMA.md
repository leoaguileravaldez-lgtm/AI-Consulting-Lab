# Dissent Record Schema

## Required Fields

- `dissent_id`, `schema_version`, `record_version`, `engagement_id`, `canonical_task_id`.
- `recommendation_ref`, `challenge_ref`, `producer_response_ref`, `revised_recommendation_ref`, `rechallenge_refs`.
- `competing_positions`, `evidence_refs`, `assumption_refs`, `method_refs`.
- `materiality`, `severity`, `decision_impact`, `confidence_differences`, `residual_uncertainty`.
- `resolution_attempts`, `status`, `synthesis_ref`, `decision_packet_ref`, `human_principal_disposition_ref`.
- `created_at`, `created_by`, `sealed_at`, `superseded_by`, `audit_refs`.

## Status Rules

Permitted record statuses are `OPEN`, `RESOLVED`, `ACCEPTED_NON_MATERIAL_LIMITATION`, `UNRESOLVED`, and `SUPERSEDED`. They are not canonical task states or blocker dispositions.

An unresolved Material dissent remains visible and cannot be downgraded by majority, seniority, averaging, timeout, commercial pressure, Human preference, synthesis or formatting. Each position and failed resolution attempt remains immutable.
