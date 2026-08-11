# Challenge Record Schema

## Canonical Status

This schema is subordinate to the canonical `CHALLENGER`, `CHALLENGE` state and assurance records in `01_ORCHESTRATOR`. It creates no canonical state or authority.

## Required Fields

- `challenge_id`, `schema_version`, `record_version`, `engagement_id`, `canonical_task_id`.
- `target_object_type`, `target_object_id`, `target_version_hash`, `target_owner`, `producer_actor_session_refs`.
- `materiality`, `decision_criticality`, `challenge_methods`.
- `challenger_actor_id`, `challenger_session_id`, `canonical_assignment_ref`, `independence_record_ref`.
- `input_manifest`, `evidence_refs`, `assumption_refs`, `model_method_refs`, `dependency_refs`.
- `method`, `findings`, `supporting_evidence`, `contradicting_evidence`, `alternative_hypotheses`.
- `decision_reversal_conditions`, `residual_uncertainty`, `confidence`, `limitations`.
- `outcome`, `response_status`, `rechallenge_status`, `created_at`, `sealed_at`, `superseded_by`, `audit_refs`.

## Rules

Allowed outcomes are the subordinate values defined in `CHALLENGE_TAXONOMY.md`. They cannot approve, reject, transition, resolve a blocker, accept risk, authorize delivery/closure or set canonical confidence.

The record is immutable after sealing. Corrections and changed targets create new versions. Missing exact target, independence, validated evidence, audit or Material finding disposition makes assurance incomplete.
