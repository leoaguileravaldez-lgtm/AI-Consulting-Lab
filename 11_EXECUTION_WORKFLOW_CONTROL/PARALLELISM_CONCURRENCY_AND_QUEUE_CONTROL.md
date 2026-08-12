# Parallelism, Concurrency, and Queue Control

## Parallel branches

Branches use unique work-item/attempt IDs, exact input versions, isolated outputs, client and engagement boundaries, dependency lineage, contradiction visibility, and append-only audit. They do not share mutable completion flags, counters, leases, or outputs.

Merge requires an explicit synthesis/transition request that preserves every branch, dissent, contradiction, failure, limitation, and version. Averaging, majority vote, last-write-wins, earliest/fastest result, or convenience cannot resolve Material differences.

## Concurrency

Optimistic control requires compare-and-swap against exact predecessor version; mismatch rejects the write and requires revalidation. Pessimistic control requires a scoped lease with owner, object/version, boundary, purpose, acquired/expiry times, renewal rules, and audit.

Stale, expired, foreign, overlapping, orphaned, or unverifiable leases cannot authorize work. Race, stale write, duplicate transition, or conflicting update is rejected, preserved, and escalated. No silent overwrite is permitted.

Duplicate execution prevention uses exact work item/version plus idempotency boundary and attempt identity. Idempotency means duplicate suppression within an exact authorized effect; it is not permission to retry or act externally.

## Queue governance

Queue entries retain client/engagement/security metadata. Segregated logical queues are required; a shared physical implementation may not erase boundaries or expose content/state.

Priority ordering is transparent, versioned, auditable, aging-aware, and constrained against starvation. Commercial value, client pressure, relationship, urgency, or executive interest cannot alter evidence, QA, risk, professional, approval, or Human thresholds. Reprioritization requires reason/actor/time and cannot hide low-priority Material risk.

