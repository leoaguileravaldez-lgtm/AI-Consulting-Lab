# Break-Glass, Revocation, and Freshness

## Emergency access

Break-glass is exceptional and never default. A request records the emergency, necessity, alternatives, exact requester/effective actor, object/action/purpose/boundary/confidentiality scope, requested duration, risks, constraints, source authority, and review plan. The decision requires distinct eligible authority, mandatory SOD, Human Principal reference where required, explicit expiry, containment, monitoring, post-use review, and audit.

An Emergency Decision is structurally subordinate to one exact, current canonical Authorization Decision and that decision's exact reconciled Access Decision. It records both IDs/versions/hashes, their reconciliation manifest, input and request-comparison manifests, complete deputy chain, effective actor, final executor, object/action, purpose, jurisdiction, confidentiality, boundaries, lifecycle/freshness, and one exact execution-instance ID/version/hash. A contemporaneous typed equality/intersection manifest is mandatory before use; later audit assembly cannot cure its absence.

Break-glass cannot waive confidentiality, client/entity/engagement isolation, provenance, Human restrictions, mandatory law/contract, audit, or Layer 12 SOD. A separately certified emergency rule may narrow one otherwise delegable access constraint only by exact scope, duration, authority, and compensating controls; it cannot create unlimited administration, professional authority, decision approval, or external-action authority.

Break-glass never creates independent access authority, substitutes an executor or deputy, expands scope, or bypasses Authorization/Access Decision reconciliation. Any canonical-reference mismatch, stale/revoked/expired/superseded authority, failed intersection, self-approval, or incomplete lineage denies.

The only authority-source type is `CANONICAL_AUTHORIZATION_DECISION`. Parent emergency authority is validated empty, emergency delegation depth is zero, and nested, recursive, delegated, or emergency-to-emergency authority is denied.

Each emergency authorization binds a unique use ID to exactly one execution instance and permits exactly one use. The unconsumed state has count zero; consumption creates an append-only successor state and immutable audit event with count one. A consumed record, duplicate identifier, second execution, changed execution binding, uncertain prior use, or replay attempt denies even before expiry. Time validity alone is never replay protection.

## Revocation precedence

Revocation of identity, role, permission, authority, delegation, elevation, assurance, session/context, or emergency decision immediately invalidates every dependent authorization. Terminated, suspended, or compromised identity is treated equivalently for affected operations.

No cache, retry, recurrence, stale workflow, downstream copy, summary, prior approval, or prior Authorization Decision can resurrect revoked state. Revocation propagation records dependency graph, affected decisions, notification reference, reconciliation, and audit. Unknown propagation is deny.

## Time and freshness

Every Material object has effective, expiry/non-expiring basis, assessment, review due, revocation, supersession, and freshness fields. Layer 13 implements no clock. A certified time source/reference is a dependency; unknown time, clock basis, freshness, or validity window denies Material authorization.
