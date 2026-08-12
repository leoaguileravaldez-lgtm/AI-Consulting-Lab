# Confidentiality, Access, and Segregation

## Classification

Every Layer 09 object carries the applicable `00_CORE` classification and the most restrictive client, legal, contractual, regulatory, professional, engagement, and purpose limitations known. Classification metadata constrains use but does not itself grant access.

Where classifications conflict or are unknown, the highest plausible restriction applies and affected access/use is blocked pending authoritative resolution. A confidentiality downgrade requires exact object/version scope, documented basis, required reviews, Human Principal authority, effective time, and audit reference. Silent inheritance, bulk downgrade, or inference from recipient seniority is prohibited.

## Access boundary

Layer 09 defines required access attributes—client, legal entity, engagement, purpose, classification, role, time, authorization reference—but provides no identity provider, role assignment, permission engine, credential, encryption, storage, or access grant. Future implementation must be separately certified and default deny.

Possession, visibility, cached availability, technical capability, CRM role, account ownership, or prior access is never authority. Access decisions cannot be inferred from Layer 09 status.

## Cross-client segregation

Client objects are isolated by immutable `client_id`, security domain, classification, and purpose. Engagement-bound objects additionally require exact `engagement_id`. Cross-client queries, joins, exports, summaries, caches, prompts, embeddings, search results, and communications default deny.

A mismatch, missing boundary, mixed result, foreign identifier, or unexpected residue causes denial, quarantine, audit preservation, containment, and escalation. Results cannot be sanitized after disclosure and treated as harmless.

## Confidentiality changes

New restrictions apply prospectively and trigger impact review for existing associations, distributions, cached summaries, communications, reuse candidates, and retention. Downgrades do not retroactively authorize prior use. A jurisdiction, entity, ownership, or stakeholder change triggers classification revalidation.

## Minimum future conformance

Any future implementation must demonstrate tenant and engagement isolation; object- and field-level policy enforcement; deny-by-default authorization; least privilege; time and purpose limits; safe indexing/cache/backup/deletion behavior; metadata and prompt segregation; immutable audit; incident containment; and negative tests for forged, missing, mixed, stale, and replayed identifiers. This document authorizes none of those capabilities.

