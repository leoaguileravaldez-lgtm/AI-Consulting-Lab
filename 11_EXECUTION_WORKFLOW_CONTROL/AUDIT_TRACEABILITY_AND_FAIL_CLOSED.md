# Audit, Traceability, and Fail-Closed Control

Every Material transition records exact work item/object/version, actor/role, time, predecessor/successor, reason, triggering dependency/event, invariant/readiness results, client/engagement boundary, canonical authority, Human reference where applicable, outcome, and correlation.

Audit is append-only by design. Corrections and reconciliation create linked records; Layer 11 cannot alter or delete history. Missing or mixed lineage blocks the affected transition.

Required dependency lineage is deterministically reconstructable as `request -> exact Work Item ID/version -> complete dependency-set declaration -> exact Dependency Record IDs/versions -> exact source objects/versions/hashes -> applicable canonical authority layers`. Operational lineage continues as `dependency reconciliation/readiness -> queue -> lease -> attempt -> transition -> output/handoff -> exception/challenge/QA -> canonical decision -> completion/cancellation/supersession`.

Audit records preserve dependency-set status, declared count, exact membership, validation reference/time, reconciliation results, discovered omissions, contradictions, and every versioned change. No prose-only representation, summary state, or downstream object can fill a lineage gap. A missing declaration, falsely empty set, count mismatch, incomplete enumeration, orphan/forgery, cycle, self-dependency, mutation, or substitution blocks the affected Work Item.

## Fail-closed matrix

Missing/stale dependency, superseded input, ambiguous identity, duplicate work/execution, race, stale write, retry loop, infinite recurrence, stale approval/evidence/client state, conflicting branch, unauthorized transition, Human proxy, commercial override, timeout-to-success, retry laundering, cancellation/rollback laundering, cross-client/engagement queue leakage, runtime/connector leakage, external action, or hidden automation authority yields denial, quarantine, blocker, `REVALIDATION_REQUIRED`, escalation, or failure as appropriate—never progress or authority.

A `RELEASE` lineage terminates at the exact Layer 07 release-governance object and authority. Layer 11 records and validates that reference only; audit presence, dependency satisfaction, or workflow completion cannot become release authorization.

Every Material uncertainty means no eligibility, transition, execution, retry, recurrence, resume, merge, completion, cancellation effect beyond operational stop, rollback effect, release, or external action.
