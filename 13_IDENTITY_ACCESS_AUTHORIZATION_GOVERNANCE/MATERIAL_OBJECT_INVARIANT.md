# Material Object Invariant

Every Material object physically includes and validates, where applicable:

- immutable object ID/type, `schema_version`, and `record_version`;
- actor identity, principal identity, effective actor identity, identity class, and exact alias/resolution references;
- Layer 10 client instance/security domain, Layer 09 client, legal entity, Layer 03 engagement/subdomain, and jurisdiction;
- role reference, authority source, permission scope, access scope, purpose-of-use reference, confidentiality scope, data/object scope, and operation/action class;
- lifecycle/status, `effective_at`, `expires_at` or explicit non-expiring basis, `assessed_at`, `review_due_at`, `revoked_at`, `superseded_at`, and freshness state;
- provenance, issuer, grantor, approver, review authority, revocation/supersession, dependencies, contradictions, limitations, retention/disposition, append-only audit references, validation state/reference/time.

## Schema incorporation contract

Every schema object incorporates all common fields by this normative reference and adds object-specific fields. Implementations must expand the fields into each concrete record. A pointer, inherited default, prose claim, absent collection, or null does not satisfy the contract.

`NOT_APPLICABLE` requires exact field, scope, reason, source authority, and validation reference. It is prohibited for identity/version, status, purpose, classification/confidentiality, provenance, freshness, source authority, issuer/creator, limitations, audit, validation, and applicable client/entity/engagement/jurisdiction boundaries.

Validation states are `VALIDATED_CURRENT`, `UNVERIFIED_BLOCKED`, `INCOMPLETE_BLOCKED`, `IDENTITY_BLOCKED`, `AUTHENTICATION_BLOCKED`, `AUTHORITY_BLOCKED`, `PERMISSION_BLOCKED`, `PURPOSE_BLOCKED`, `CONFIDENTIALITY_BLOCKED`, `BOUNDARY_BLOCKED`, `JURISDICTION_BLOCKED`, `SOD_BLOCKED`, `DEPENDENCY_BLOCKED`, `STALE_BLOCKED`, `EXPIRED_BLOCKED`, `REVOKED_BLOCKED`, `SUPERSEDED_BLOCKED`, and `REVALIDATION_REQUIRED`.

Only `VALIDATED_CURRENT` may support an authorization proposal. Unknown, missing, ambiguous, stale, terminated, suspended, compromised, expired, revoked, superseded, cross-boundary, unreconciled, or unauthorized Material state means deny. No silence, timeout, default, role, title, seniority, domain, CRM state, workflow state, model inference, agent assertion, consensus, vote, client request, commercial urgency, retry, cache, or aggregate score creates access or authority.

Validation proves record integrity and bindings only. It does not authenticate a person, grant runtime permission, create professional/evidence/QA/release/reuse/workflow/risk/decision/Human authority, or execute access.

## Access-decision and deputy-chain invariant

A Material Access Decision is valid only as an exact, current, state-locked projection of one authoritative Authorization Decision ID/version/hash. State, actor/principal/effective actor, deputy manifest, input manifest, authority, scope, client/entity/engagement, jurisdiction, purpose, confidentiality, lifecycle/freshness, validity, revocation, and supersession must reconcile; no projection may be more permissive.

A Material Authorization Decision involving a deputy, service, model, agent, connector, automation, or acting-on-behalf-of relationship is valid only with the complete ordered originator-to-final-executor chain. Every member and delegation link must be exact, current, attributable, boundary-consistent, and independently authorized. Effective authorization is the least-privilege intersection across all members and non-waivable controls. Missing chain declarations never mean direct access; direct access requires a validated-empty deputy chain. Chain truncation, authority inheritance, confused-deputy use, or any amplification is denied.

A zero-deputy declaration is valid only when a typed equality manifest proves, using canonical identity ID/version/hash, that requester, originator, final executor, effective actor, and Authorization Decision actor are the same identity. A different human, alias, shared account, service, model, agent, machine, proxy, or intermediary makes the chain non-empty; representing it as empty denies.

Every Authorization Decision must own a typed exhaustive Access-Request comparison manifest. It compares exact request identity/version/hash, requester/originator, ordered chain/count/members/adjacency, final executor, roles, grants, delegation sources, object/action, purpose, jurisdiction, boundaries, confidentiality, lifecycle, freshness, validity, and constraints. Missing comparisons, reordering, omission, substitution, expansion, or generic prose reconciliation cannot validate a decision. A change requires a new explicit authorization lifecycle.

A Material Break-Glass Decision is valid only as a narrower subordinate transformation of one exact current Authorization Decision ID/version/hash and its input, request-comparison, chain, actor/executor, object/action, and boundary manifests. It cannot independently authorize access. The canonical/emergency least-privilege intersection and unchanged actor/executor/chain proof are mandatory.

It must also bind the exact reconciled Access Decision ID/version/hash and one exact execution-instance ID/version/hash through a typed contemporaneous comparison manifest. Its authority-source type is only `CANONICAL_AUTHORIZATION_DECISION`; parent emergency reference is validated empty, emergency delegation depth is zero, and emergency-derived authority is prohibited.

Every emergency authorization has one unique use ID, `permitted_use_count = 1`, explicit zero-or-one consumed count, consumption state, replay state, and immutable consumption audit reference. Consumption creates an append-only successor state; it never overwrites the authorization record. `CONSUMED`, duplicate-use, unknown-use, or execution-binding mismatch is non-actionable regardless of remaining validity.
