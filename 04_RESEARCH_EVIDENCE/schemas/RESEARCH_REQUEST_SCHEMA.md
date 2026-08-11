# Research Request Schema

## Required Fields

- `research_request_id`, `schema_version`, `record_version`, `engagement_id`, `task_id`, `workstream_id`.
- `requester_actor_id`, `requester_role_assignment_ref`, `authorization_ref`, `requested_at`, `audit_correlation_id`.
- `question`, `claim_to_test`, `decision_context`, `materiality`, `decision_criticality`.
- `jurisdiction`, `time_scope`, `population`, `definitions`, `units`.
- `minimum_evidence_threshold`, `freshness_deadline`, `deadline`, `priority`.
- `authorized_source_classes`, `authorized_tool_refs`, `data_classification`, `access_constraints`.
- `known_source_refs`, `assumptions`, `known_conflicts`, `exclusions`, `requested_output`.
- `bias_flags`, `acceptance_record_ref`, `canonical_assignment_refs`, `status`, `audit_ref`.

## Rules

`status` is a non-authoritative request-record label only: `PROPOSED`, `ACCEPTED`, `REJECTED`, `RETURNED_FOR_CLARIFICATION`, `RESEARCH_COMPLETE`, or `SUPERSEDED`. It is not a task state or permission.

Only `01_ORCHESTRATOR` may create canonical assignments, handoffs, access or transitions. Request fields naming future tools are descriptive; they do not activate a connector or authorize credentials, browsing, purchasing, subscription, messaging, email, upload or execution.

The request cannot prescribe a favorable conclusion, redefine the business decision, lower classification or suppress contrary evidence. Missing authority, engagement, scope, jurisdiction, materiality or access fails closed.
