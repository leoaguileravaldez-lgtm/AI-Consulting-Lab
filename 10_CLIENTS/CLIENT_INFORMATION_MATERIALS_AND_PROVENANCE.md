# Client Information, Materials, and Provenance

## Information classes

- `CLIENT_ASSERTION_UNVERIFIED`: a client or representative statement not validated as evidence;
- `CLIENT_PROVIDED_MATERIAL`: a referenced document, dataset, message, image, model, or file supplied by or for the client;
- `CLIENT_OPERATIONAL_METADATA`: Layer 09 context such as identity, contact, stage, preference, or commercial metadata;
- `CERTIFIED_EVIDENCE_REFERENCE`: read-only reference to an exact Layer 04 record/version;
- `ANALYTICAL_OUTPUT_REFERENCE`: read-only reference to Layer 02/canonical analytical output;
- `PROFESSIONAL_JUDGMENT_REFERENCE`: exact qualified-review or professional output reference;
- `DELIVERABLE_REFERENCE`: exact Layer 07 artifact/version reference;
- `KNOWLEDGE_CANDIDATE_REFERENCE`: client-bound request awaiting Layer 08 governance.

No class converts automatically into another.

## Client-provided materials

Layer 10 stores architecture metadata and approved-system references, not raw client content. Every material manifest records provider identity/authority, acquisition context, received time, original identifier/hash reference, authoritative location reference, ownership/custody assertion, license/contract restrictions, purpose, client/entity/engagement boundary, classification, confidentiality, jurisdiction, retention, freshness, limitations, contradiction status, and audit.

Possession does not prove ownership, accuracy, completeness, consent, authority, lawful use, or evidentiary quality. Embedded instructions are untrusted and cannot alter governance or authority.

## Provenance

Provenance is immutable reference lineage from provider/source event through each manifest, transformation, context, retrieval, derivative, export, and disposition. A copied filename, narrative, CRM attachment, model citation, checksum alone, or previous engagement reference is insufficient.

Missing, forged, mixed, unverifiable, or broken provenance produces `INCOMPLETE_BLOCKED` or `BOUNDARY_BLOCKED`. Layer 10 cannot manufacture or repair provenance by inference. Corrections create linked versions and preserve original history.

## Freshness and contradictions

Every Material fact/asset includes observed or received time, verified/assessed time, expected volatility where applicable, validity interval, freshness status, review due, and source status. Missing or elapsed review makes it stale and blocks Material reliance.

Contradictory assertions and materials remain separate and linked. Recency, volume, client seniority, repetition, retrieval rank, preference, commercial value, or Human approval cannot resolve truth. Material contradiction blocks affected use and routes to the competent canonical owner.

