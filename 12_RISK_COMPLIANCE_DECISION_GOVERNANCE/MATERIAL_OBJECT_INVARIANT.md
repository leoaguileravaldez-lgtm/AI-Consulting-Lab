# Material Object Invariant

Every Material object explicitly carries:

- immutable object ID, object type, `schema_version`, and `record_version`;
- `client_instance_id`, `client_id`, `legal_entity_id`, `engagement_id`, client security domain, and engagement subdomain where applicable;
- jurisdiction set and applicability scope;
- purpose, classification, confidentiality state, lifecycle/status, and materiality;
- source authority, provenance references, creator identity/role/time, effective time, freshness status, review deadline, expiration or explicit non-expiring basis;
- owner, review authority, Human Principal decision reference where required, exact dependency IDs/versions, contradiction/conflict references, and limitations;
- supersedes/superseded-by, retention/disposition reference, append-only audit references, validation state, and validation reference/time.

## Schema incorporation contract

Every object section in `schemas/` incorporates every field above as a required field by this exact normative reference and then adds its object-specific fields. Implementations must expand the incorporated fields into the concrete record; storing only an object-specific list, a pointer to this document, an inherited default, or a prose assertion is invalid. Field omission is permitted only through the explicit field-level `NOT_APPLICABLE` rule below. Schema/version validation must verify both the incorporated common fields and the object-specific fields before `VALIDATED_CURRENT` is possible.

`NOT_APPLICABLE` requires an exact field, reason, scope, authority, and validation reference. It is prohibited for identity/version, purpose, classification, confidentiality, lifecycle, provenance, freshness, source authority, creator, limitations, audit, validation, and applicable tenant/entity/engagement boundaries.

Validation states are `VALIDATED_CURRENT`, `INCOMPLETE_BLOCKED`, `STALE_BLOCKED`, `AUTHORITY_BLOCKED`, `BOUNDARY_BLOCKED`, `JURISDICTION_BLOCKED`, `DEPENDENCY_BLOCKED`, `CONFLICT_BLOCKED`, `SOD_BLOCKED`, `APPROVAL_BLOCKED`, `EXPIRED_BLOCKED`, `REVOKED_BLOCKED`, and `SUPERSEDED_BLOCKED`.

Only `VALIDATED_CURRENT` may be used in a decision-gate proposal. A missing, null, malformed, ambiguous, stale, expired, revoked, superseded, contradictory, unauthorized, cross-boundary, unreconciled, or unvalidated Material field makes the object non-current, non-authoritative, non-reusable, non-actionable, and decision/readiness-ineligible.

No model, agent, workflow, CRM field, user repetition, source count, elapsed time, retry, commercial priority, confidence, vote, consensus, downstream expectation, or aggregate score can cure an invariant failure or create authority.

Validation establishes record completeness and authority binding only. It cannot establish factual truth, evidence validity, analytical correctness, professional approval, QA passage, release, reuse, access, execution, or external-action authority.

## Decision-specific invariant

A Material Decision Record is `VALIDATED_CURRENT` only when its exhaustive typed input snapshot, counts, manifest hash, empty-category validations, exact-ID/version/hash and boundary/authority/currentness reconciliation, complete Approval Requirement-to-Approval Record chain, mandatory SOD/effective-actor evaluation, conflict state, jurisdiction applicability, compliance and risk validity, exception/waiver validity, freshness, and required Human Principal references all pass. Upstream Gate/Request references, prose lineage, inferred emptiness, or aggregate status cannot substitute. Only then may the separately validated decision status be `APPROVED`; otherwise it is `APPROVAL_BLOCKED` or the stricter applicable blocked state.
