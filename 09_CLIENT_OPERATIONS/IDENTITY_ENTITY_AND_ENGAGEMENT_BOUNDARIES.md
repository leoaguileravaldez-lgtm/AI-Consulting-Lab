# Identity, Entity, and Engagement Boundaries

## Canonical identity

`client_id` is an immutable, opaque internal identifier. `legal_entity_id` identifies an exact legal person or organization within that client boundary. Display name, brand, domain, address, tax label, external CRM ID, and parent name are mutable attributes or aliases and must never be used alone as identity.

A verified identity records verification basis, verifier, source reference, time, jurisdiction, limitations, and next review. Verification metadata is operational assurance only and does not make the underlying assertion Layer 04 evidence.

## Entity relationships

Parent, subsidiary, affiliate, joint venture, franchise, fund, portfolio company, government unit, and successor relationships are explicit typed edges. A relationship edge does not merge confidentiality, conflicts, authorizations, engagements, knowledge, or access.

Legal-entity changes use new versions or successor records. Merger, acquisition, conversion, rename, dissolution, redomiciliation, or contracting-party substitution triggers identity review, conflict review, jurisdiction review, confidentiality review, engagement-scope revalidation, and affected authorization review. Prior records remain immutable history.

## Duplicate and ambiguity handling

Potential duplicate indicators create an `Identity Resolution Case`. Candidate records are quarantined from new association, reuse, outbound action, and Material reliance until resolved. Automatic merge is prohibited.

Resolution outcomes are `DISTINCT`, `ALIAS_CONFIRMED`, `MERGE_APPROVED`, `SPLIT_APPROVED`, or `UNRESOLVED`. Merge and split require Human Principal authority, survivor/new IDs, field-by-field provenance, preserved aliases, complete edge remapping, confidentiality and conflict review, rollback/reconstruction information, and audit lineage. Conflicting facts remain visible; they are not silently selected.

Unknown client identity, ambiguous legal entity, missing canonical ID, contradictory ownership, or uncertain association blocks affected work and is escalated. Urgency and commercial value cannot compensate.

## Engagement association and isolation

An association references the canonical Layer 03 `engagement_id`; Layer 09 does not create it. The association records exact client and legal entity, relationship role, effective dates, purpose, classification, scope reference, authorization reference, and status.

Each engagement remains isolated even for the same client. A client-level contact or preference may be projected into an engagement only through an explicit, current, purpose-compatible association. No projection imports evidence, assumptions, conclusions, models, challenge, QA, risk, deliverables, or knowledge.

Scope amendment, reassignment, new stakeholder authority, or new jurisdiction creates a proposed association version and blocks affected reliance until Layer 03 change control and required revalidation complete. A Layer 09 note cannot amend engagement scope.

## Negative rules

- Fuzzy match is a review signal, not identity.
- Shared email domain is not proof of entity or authority.
- Parent ownership does not authorize subsidiary data access or reuse.
- Same client does not mean same engagement boundary.
- Same engagement name does not mean same engagement ID.
- A copied identifier without valid boundary and purpose is rejected.
- Dormant or closed status does not remove confidentiality or conflict obligations.

