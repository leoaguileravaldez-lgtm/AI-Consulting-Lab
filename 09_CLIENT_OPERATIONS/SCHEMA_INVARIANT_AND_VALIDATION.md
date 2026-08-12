# Schema Invariant and Validation

## Common Material-object contract

Every Material Layer 09 object must explicitly carry:

- immutable object identifier, `schema_version`, and `record_version`;
- canonical `client_id`, `legal_entity_id`, and `engagement_id` boundaries where applicable;
- `client_security_domain` and exact purpose or purpose limits;
- `classification_ref` and applicable confidentiality restrictions;
- lifecycle or decision `status`;
- `freshness_status`, `verified_or_assessed_at`, and `review_due`;
- source/provenance references and creator identity/time;
- contradictions, limitations, supersession or correction lineage where applicable;
- authorization, review, and audit references appropriate to the object.

An inapplicable boundary or attribute is represented explicitly as `NOT_APPLICABLE` with a reason and validation reference. It is never omitted, empty, inferred, or populated from similarity. `NOT_APPLICABLE` is invalid for `client_id`, `client_security_domain`, classification, purpose, status, version, freshness, limitations, creation provenance, or audit lineage on a client-bound Material object.

## Validation outcome

Each record carries `schema_invariant_status` with only:

- `VALIDATED_CURRENT`: every required field is present, internally consistent, within validity, and supported by the required validation references;
- `INCOMPLETE_BLOCKED`: at least one required field is missing, empty, invalid, unsupported, or not yet validated;
- `STALE_BLOCKED`: freshness or review deadline has failed;
- `CONTRADICTED_BLOCKED`: a Material contradiction is unresolved;
- `BOUNDARY_BLOCKED`: a client, legal-entity, engagement, security-domain, purpose, or classification boundary is missing, ambiguous, foreign, or inconsistent;
- `SUPERSEDED_BLOCKED`: the record is not the current version.

Only `VALIDATED_CURRENT` is eligible for operational or decision context, and only for its exact purpose and boundary. It creates no evidence, permission, approval, access, reuse, or external-action authority.

## Fail-closed validation rule

Any Material object missing required classification, purpose, provenance, freshness, applicability where relevant, boundary, limitation, status, review state/deadline, version, creation provenance, or audit lineage is automatically:

- non-current;
- non-reusable;
- non-authoritative;
- ineligible for an operational-context packet, decision request, association, projection, communication, reuse, or other reliance;
- quarantined until corrected through a new version and independently validated under the competent owner.

No default, inference, ranking, similarity, CRM state, commercial importance, client preference, historical relationship, prior record, model confidence, or technical availability may supply or compensate for a required field. A later correction does not retroactively authorize earlier use.

## Cross-layer boundary

Schema validation proves only Layer 09 record completeness and boundary consistency. It cannot establish factual truth, evidence quality, provenance sufficiency for Layer 04, applicability for analytical use, professional validity, certainty, permission, or Human Principal approval. Layer 04 remains the exclusive evidence authority; certified owners retain all other canonical authority.

## Context-packet gate

Before a Layer 09 record can be referenced in an operational-context packet, the packet must carry the source record's `schema_invariant_status`, exact version, boundary, purpose, classification, freshness, review deadline, contradictions, limitations, and validation reference. Any value other than `VALIDATED_CURRENT`, any elapsed deadline, or any mismatch blocks packet creation and receipt-side reliance.

