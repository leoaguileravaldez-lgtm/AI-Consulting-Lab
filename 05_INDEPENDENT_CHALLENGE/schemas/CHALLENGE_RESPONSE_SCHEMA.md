# Challenge Response Schema

## Required Fields

- `response_id`, `schema_version`, `record_version`, `engagement_id`, `canonical_task_id`.
- `challenge_id`, `challenge_version`, `target_object_id`, `target_version_hash`.
- `producer_actor_session_refs`, `response_value`, `reasoning`, `evidence_refs`, `assumption_refs`.
- `accepted_findings`, `rebutted_findings`, `revised_fields`, `new_recommendation_ref`.
- `confidence_impact`, `dependency_impact`, `unresolved_matters`, `additional_research_request_refs`.
- `rechallenge_required`, `created_at`, `sealed_at`, `superseded_by`, `audit_refs`.

## Rules

Permitted response values are `ACCEPT`, `PARTIALLY_ACCEPT`, `REBUT`, `REVISE`, `REQUEST_ADDITIONAL_EVIDENCE`, and `UNRESOLVED`. These do not create canonical states, decisions, approval or blocker resolution.

The producer cannot edit the challenge. A Material revision creates a new recommendation version and dependency review; prior challenge coverage becomes `SUPERSEDED` or `RECHALLENGE_REQUIRED`. Silence and timeout are not acceptance.
