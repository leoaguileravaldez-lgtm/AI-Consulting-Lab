# Retry, Recurrence, and Timeout Model

## Retry

Each retry creates a distinct Execution Attempt with immutable lineage to the original work item and all prior attempts. It increments `retry_count`, preserves failures/outputs/audit, re-evaluates invariants, prerequisites, dependencies, boundaries, freshness, authority, and idempotency, and respects a recorded ceiling/backoff policy.

Repetition never converts failure to success. Ceiling reached, recurring same failure, unknown outcome, non-idempotent uncertainty, or Material failure escalates or fails closed. Retry cannot replay approval outside exact version/scope/window.

## Recurrence

Recurrence is not retry. Each scheduled occurrence creates a distinct governed work item/instance with `recurrence_count`, series lineage, intended cadence, creation authority, and independent validation. It never inherits stale evidence, approvals, client/entity/engagement state, jurisdiction, confidentiality, applicability, risk/QA, or completion.

Infinite or unbounded recurrence is prohibited. Missing end/review condition or recurrence authority yields `LIFECYCLE_BLOCKED`.

## Timeout

A timeout records policy, clock basis, start/deadline, pause rules, observed time, affected attempt, and audit. It can only route to `WAITING_HUMAN`, `ESCALATED`, `REVALIDATION_REQUIRED`, or `FAILED`.

Timeout never means approval, rejection, success, completion, risk acceptance, waiver, cancellation, or Human decision. Timer and scheduler capability are not implemented here.

