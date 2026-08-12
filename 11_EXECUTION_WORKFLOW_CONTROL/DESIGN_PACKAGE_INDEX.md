# Layer 11 Design Package Index

## Status

Local Markdown architecture awaiting Human Principal certification. Certified Layers 00–10 are unchanged; Layer 12 is not begun.

## Controls

- `EXECUTION_WORKFLOW_ARCHITECTURE.md`: scope, authority, separation, object model.
- `MATERIAL_OBJECT_INVARIANT.md`: mandatory fields and fail-closed validity.
- `WORK_ITEM_AND_STATE_MODEL.md`: work item, 16 states, completion.
- `TRANSITION_AND_READINESS_CONTROL.md`: legal edges, guards, pause/resume.
- `DEPENDENCY_BLOCKER_AND_GATE_MODEL.md`: typed exact-version dependencies, canonical Work Item set reconciliation, explicit Layer 07-bound `RELEASE`, and blockers.
- `RETRY_RECURRENCE_TIMEOUT_MODEL.md`: attempts, recurrence, timeout.
- `PARALLELISM_CONCURRENCY_AND_QUEUE_CONTROL.md`: branches, leases, races, queues.
- `EXCEPTION_ESCALATION_CANCELLATION_ROLLBACK.md`: failure routing and history preservation.
- `CLIENT_ISOLATION_AND_HANDOFFS.md`: client/engagement segregation and handoffs.
- `HUMAN_PRINCIPAL_AND_AUTOMATION_BOUNDARY.md`: intervention and non-runtime boundary.
- `AUDIT_TRACEABILITY_AND_FAIL_CLOSED.md`: audit lineage and negative outcomes.
- `EXECUTION_WORKFLOW_IMPLEMENTATION_READINESS.md`: adversarial suite and limitations.
- `MATERIAL_OBJECT_COVERAGE_AUDIT.md`: 14-object invariant review.
- `LOCAL_CERTIFICATION_REPORT.md`: local certification evidence.

## Schemas

- `schemas/WORK_ITEM_TRANSITION_SCHEMA.md`: Work Item with exhaustive dependency declaration, Transition Request, Readiness Assessment.
- `schemas/DEPENDENCY_ATTEMPT_SCHEMA.md`: Dependency, Blocker, Execution Attempt, Recurrence.
- `schemas/QUEUE_CONCURRENCY_EXCEPTION_AUDIT_SCHEMA.md`: Queue, Lease, Timeout, Exception, Escalation, Cancellation/Rollback, Audit.

Certified upstream controls prevail over Layer 11.
