# Authorization Decision and Access Context

## Exact context

Every Material determination binds exact actor, effective actor, principal, object IDs/versions/hashes, client, legal entity, engagement, jurisdiction, action, purpose, role assignment, permission grant, authority grant where the operation requires determination authority, confidentiality constraints, SOD constraints, dependencies, session/authentication references, currentness, time, basis, and audit.

Purpose is exact and non-transferable. Evidence review, analysis, QA, decision review, client operations, billing, reuse, delivery, compliance, and administration require separately evaluated purposes. Downstream use, possession, retrieval, or successful prior access cannot broaden purpose.

## Decision states

States are `REQUESTED`, `ELIGIBILITY_PENDING`, `APPROVED`, `DENIED`, `CONDITIONALLY_APPROVED`, `EXPIRED`, `REVOKED`, `SUPERSEDED`, and `REVALIDATION_REQUIRED`. No implicit `APPROVED` exists.

The Authorization Decision Record owns an exhaustive exact-version/hash manifest of identity/resolution, assurance/context, roles, permissions, authorities, policies, boundaries, purpose, confidentiality, SOD, delegation/elevation/break-glass if applicable, dependencies, object/action set, and Human Principal references. It records per-category counts, validated-empty declarations, reconciliation status/time, failed conditions, and decision basis.

For acting-on-behalf-of access, the manifest also owns the originating actor/principal/effective actor, every ordered intermediary/deputy/service/model/agent principal and effective actor, the final executor, every authority and delegation source, and a chain hash. Each member carries its own roles, permissions, authority, purpose, jurisdiction, boundaries, confidentiality, lifecycle/freshness, and authorization references. Direct access requires an explicit validated-empty deputy chain.

Direct access additionally requires canonical identity equality across requester, originator, final executor, effective actor, and Authorization Decision actor. The equality manifest contains exact IDs, versions, hashes, and identity-resolution references; count zero plus matching hashes alone is insufficient.

The Authorization Decision contains a typed comparison manifest against the exact Access Request. Every requested chain member and adjacency is compared in order, with the complete role/grant/delegation/scope/purpose/jurisdiction/boundary/confidentiality/lifecycle/freshness/validity state. A missing, substituted, reordered, or expanded item denies. Permissive change cannot be treated as reconciliation; it requires a new request and authorization lifecycle.

Effective authorization is the narrowest intersection of all chain members' current permissions, authority, delegation, object/action scope, purpose, jurisdiction, client/entity/engagement boundary, confidentiality, validity, and non-waivable controls. No deputy inherits the service's or originator's authority by implication. Empty intersection, omitted member, alias/shared-account ambiguity, cycle, or amplification denies.

`APPROVED` requires every applicable input `VALIDATED_CURRENT`, exact set reconciliation, matching purpose/boundaries/action/object versions, unrevoked grants, valid assurance/context, SOD pass, current time window, no Material conflict, and required Human reference. Any unknown or mismatch is `DENIED` or `REVALIDATION_REQUIRED`, never conditional permission by inference.

An Access Decision is only a reconciled projection. Its state and scope cannot differ from or exceed the exact authoritative Authorization Decision; mismatch or stale authority invalidates it.

Break-glass is not an alternate decision. It binds to the same exact canonical Authorization Decision and exact reconciled Access Decision, plus their request-comparison, input, chain, actor, executor, scope, purpose, jurisdiction, confidentiality, lifecycle/freshness, and boundary state. It binds one execution instance and a single-use identifier. Its enumerated transformations are constrained by a separately recorded least-privilege intersection; actor, executor, chain, or execution-instance changes are forbidden. Emergency-derived authority, nesting, replay, and post-hoc reconciliation are invalid.
