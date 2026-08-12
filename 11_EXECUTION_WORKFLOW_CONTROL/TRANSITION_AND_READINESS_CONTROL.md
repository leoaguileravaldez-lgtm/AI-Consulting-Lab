# Transition and Readiness Control

## Legal transition model

Every transition is a proposal containing exact work item/version, predecessor state/version, requested successor, triggering event, guards, dependencies, actor/role, authority reference, reason, time, and audit. Layer 01 alone determines the canonical transition.

Allowed operational edges are:

- `REQUESTED -> QUEUED | ELIGIBILITY_PENDING | BLOCKED | CANCELLED`;
- `QUEUED -> ELIGIBILITY_PENDING | BLOCKED | CANCELLED`;
- `ELIGIBILITY_PENDING -> READY | BLOCKED | WAITING_DEPENDENCY | WAITING_HUMAN | REVALIDATION_REQUIRED | CANCELLED`;
- `READY -> RUNNING | BLOCKED | REVALIDATION_REQUIRED | CANCELLED`;
- `RUNNING -> WAITING_DEPENDENCY | WAITING_HUMAN | PAUSED | RETRY_PENDING | REVALIDATION_REQUIRED | ESCALATED | FAILED | CANCELLED | COMPLETED`;
- `WAITING_DEPENDENCY | WAITING_HUMAN | PAUSED | RETRY_PENDING | ESCALATED -> ELIGIBILITY_PENDING | REVALIDATION_REQUIRED | FAILED | CANCELLED`;
- `REVALIDATION_REQUIRED -> ELIGIBILITY_PENDING | BLOCKED | FAILED | CANCELLED`;
- `FAILED -> RETRY_PENDING | ESCALATED | SUPERSEDED`;
- `CANCELLED | COMPLETED -> SUPERSEDED` only through a new governed version/reference.

Any unlisted edge is unauthorized. Terminal records are never reopened in place.

## Readiness

Readiness is a derived operational assessment over exact authoritative references. It requires invariant pass; a Work Item-level dependency declaration that is `VALIDATED_COMPLETE` or `VALIDATED_EMPTY`; exact equality between the declaration and the assessment's dependency references; current dependencies; no Material blocker; valid boundary; eligible queue/lease state; and exact Layer 01 authority reference where required.

A Transition Request or Readiness Assessment cannot supply a missing Work Item dependency declaration, treat an omitted declaration as empty, add an undeclared dependency, omit a declared dependency, or substitute a direct upstream reference. Any mismatch, lineage gap, cycle, or failed reconciliation yields `DEPENDENCY_BLOCKED` or `REVALIDATION_REQUIRED`.

Layer 11 cannot certify evidence, approve judgment, accept risk, waive dissent, pass QA, release deliverables, or authorize action. Missing, stale, revoked, superseded, ambiguous, contradicted, or foreign upstream state yields `REVALIDATION_REQUIRED` or `BLOCKED`.

A `RELEASE` dependency is satisfied only by the exact current Layer 07 object/version and condition named in its Dependency Record. Its presence or satisfaction is not release approval and cannot authorize publication, delivery, execution, or external action.

## Pause/resume

Pause preserves attempt, lease, outputs, failures, dependencies, and audit. Resume is never automatic; it returns through eligibility/revalidation with current versions. Paused time does not satisfy prerequisites or approvals.
