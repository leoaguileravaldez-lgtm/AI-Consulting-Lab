# Change, Invalidation, and Recurrence

## Deterministic invalidation

The QA dependency manifest is traversed whenever a Material change affects engagement identity, scope, object/version/hash, jurisdiction, evidence validity/provenance/freshness/applicability, analysis, assumptions, method/model, challenge or dissent, control design/operation, remediation, professional review, exception, approval, canonical state/blocker, assessor independence, or QA methodology.

Affected QA reliance becomes:

- `SUPERSEDED` when it no longer applies to the changed object; or
- `REVALIDATION_REQUIRED` when renewed QA is required.

These are QA record statuses only. Prior versions remain immutable. Similarity, prior approval, a previous QA result, or an unchanged summary cannot preserve reliance after a Material dependency change.

## Propagation

The only permitted path is:

`QA observation → QA finding → advisory impact notice → 01_ORCHESTRATOR canonical evaluation → 03_ENGAGEMENTS reflected readiness`.

`06` cannot open or resolve a canonical blocker, return work to a state, change readiness, reopen a task, revoke approval, or authorize revalidation. If canonical receipt or dependency reconciliation is unknown, reliance fails closed.

## Recurrence and stale analysis

New occurrences link to earlier findings without overwriting them. Repeated remediation failure, reopened challenge, evidence revocation, model update, changed jurisdiction, or stale professional review triggers new impact analysis. Material recurrence remains visible and routes canonically; no recurrence count automatically changes state or severity.

