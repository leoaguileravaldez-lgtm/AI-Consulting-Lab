# Work Item and State Model

## Work item

A Work Item binds one exact request and purpose to responsible roles, client/entity/engagement boundary, an exhaustive versioned declaration of exact Dependency Records, completion criteria, and audit lineage. The declaration is canonical for prerequisite completeness; downstream transition or readiness objects only reconcile against it. Parent/child relationships do not transfer readiness or completion. A duplicate or ambiguous identity is quarantined.

Missing, null, defaulted, count-mismatched, unreconciled, or incompletely enumerated dependency sets block the Work Item. Zero dependencies is valid only through the explicit validated-empty representation. Dependency-set changes create a new Work Item record version and preserve the prior declaration; state mutation, direct upstream-object substitution, circular references, and self-dependencies are prohibited.

Priority classes affect queue ordering only and never thresholds, standards, authority, or correctness.

## States

Canonical execution descriptors are:

- `REQUESTED`: recorded, not assessed;
- `QUEUED`: admitted to a segregated queue, not ready;
- `ELIGIBILITY_PENDING`: invariant/prerequisite assessment incomplete;
- `BLOCKED`: Material guard failed;
- `READY`: exact operational guards pass, not substantive approval;
- `RUNNING`: one governed attempt is active;
- `WAITING_DEPENDENCY`: exact dependency unresolved;
- `WAITING_HUMAN`: valid Human decision required;
- `PAUSED`: operational eligibility suspended;
- `RETRY_PENDING`: new attempt proposed after failure;
- `REVALIDATION_REQUIRED`: inputs or circumstances changed;
- `ESCALATED`: issue routed, not resolved;
- `FAILED`: attempt/work criteria failed;
- `CANCELLED`: future operational eligibility terminated;
- `COMPLETED`: recorded completion criteria satisfied, without substantive certification;
- `SUPERSEDED`: replaced by exact newer work item/version.

`TIMED_OUT` is an event/outcome feeding only `WAITING_HUMAN`, `ESCALATED`, `REVALIDATION_REQUIRED`, or `FAILED`; it is not a completion or approval state.

## Completion

Completion requires every exact criterion, dependency, required upstream state, output reference, reconciliation, and audit record to be current and satisfied. Another agent's success claim, majority, downstream expectation, elapsed time, or output existence is insufficient. Completion never suppresses limitations, dissent, risk, or open substantive findings.
