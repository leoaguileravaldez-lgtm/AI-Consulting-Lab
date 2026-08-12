# Material Object Invariant

## Common contract

Every Material Layer 10 object explicitly carries:

- immutable object ID, `schema_version`, and `record_version`;
- canonical `client_instance_id`, Layer 09 `client_id`, `client_security_domain`;
- `legal_entity_id` and Layer 03 `engagement_id` where applicable;
- exact `purpose`, `classification_ref`, and `confidentiality_state`;
- lifecycle or decision `status`;
- `freshness_status`, `verified_or_assessed_at`, and `review_due`;
- `source_refs`, immutable provenance references, creator identity/time;
- `contradiction_refs`, `limitations`, and applicable supersession/correction lineage;
- `reuse_eligibility`, retention state/reference, authorization/review references, and audit lineage;
- `invariant_status` and `invariant_validation_ref`.

An inapplicable legal-entity or engagement boundary is explicitly `NOT_APPLICABLE` with reason and validation reference. `NOT_APPLICABLE` is invalid for client instance, client ID, security domain, purpose, classification, confidentiality, status, version, freshness, limitations, provenance, reuse eligibility, creator, or audit lineage.

## Invariant outcomes

- `VALIDATED_CURRENT`: complete, internally consistent, within validity, exact-boundary, and supported by required reviews;
- `INCOMPLETE_BLOCKED`: any mandatory value is absent, empty, malformed, unsupported, or unvalidated;
- `STALE_BLOCKED`: freshness or review deadline failed;
- `CONTRADICTED_BLOCKED`: Material contradiction unresolved;
- `IDENTITY_BLOCKED`: client or legal entity unknown, ambiguous, duplicated, or inconsistent;
- `BOUNDARY_BLOCKED`: security domain, engagement, purpose, classification, confidentiality, or ownership mismatch;
- `REUSE_BLOCKED`: reuse is missing, unknown, unauthorized, revoked, or incompatible;
- `LIFECYCLE_BLOCKED`: lifecycle or retention state missing, invalid, or incompatible;
- `SUPERSEDED_BLOCKED`: record is not current.

Only `VALIDATED_CURRENT` may be used in client-instance operational context, and only for the exact client, entity, engagement, purpose, classification, and time. It still creates no evidence, analytical validity, permission, reuse, approval, access, delivery, or external-action authority.

## Non-compensable failure

Any Material missing or invalid field makes the object non-current, invalid for reliance, non-reusable, non-applicable, non-authoritative, decision-ineligible, operationally ineligible, and quarantined. It cannot enter context, retrieval, analysis intake, export, deliverable association, aggregation, knowledge extraction, disposition, or state transition until a corrected version is independently validated.

No default, inference, similarity, fuzzy match, rank, repeated retrieval, CRM value, client preference, commercial priority, relationship history, model confidence, automation result, prior approval, or aggregate score can compensate. Later correction never retroactively authorizes earlier use.

## Type and authority integrity

Invariant validation proves schema completeness and boundary consistency only. It cannot establish evidence, Layer 04 provenance sufficiency, factual truth, analytical applicability, professional validity, certainty, access, reuse, approval, or action authority. An object retains its certified type across every reference and derivative.

