# Isolation, Retrieval, and Derived Artifacts

## Three nested boundaries

1. `client_security_domain` isolates one canonical client instance from every other client/prospect.
2. `legal_entity_id` restricts entity-specific purpose, jurisdiction, ownership, confidentiality, conflict, and obligations.
3. `engagement_security_subdomain` isolates each Layer 03 engagement, including engagements of the same client.

Every information-bearing object and edge carries all applicable boundary identifiers, purpose, classification, confidentiality, retention, provenance, freshness, and audit. Missing or mismatched boundary fails closed.

## Prohibited cross-boundary operations

Cross-client and cross-engagement queries, joins, comparisons, summaries, search, semantic retrieval, prompt assembly, context injection, embeddings, vector lookup, caches, exports, model inputs, model memory, derived artifacts, and deliverable association default deny.

Cross-client joins and aggregation cannot be used for convenience, analytics, benchmarking, sales insight, model improvement, or institutional memory. A permitted knowledge candidate must follow Layer 08 and cannot expose or reconstruct origin clients.

## Retrieval

A Context Boundary Manifest defines exact allowed client/entity/engagement, purpose, object types, versions, classification ceiling, confidentiality, time window, retention, and authorization references. Retrieval must be intersection-only: every result independently passes all constraints.

A Retrieval Result Manifest records query purpose/reference, boundary manifest/version, result object/version list, exclusions, boundary checks, provenance, freshness, contradiction state, and audit. A mixed, foreign, stale, unknown, unclassified, or untraceable result is suppressed and quarantined; partial sanitation after retrieval does not cure leakage.

Ranking, similarity, frequency, semantic proximity, model confidence, or prior use creates no relevance, applicability, truth, permission, or reuse authority.

## Embeddings, caches, and model memory

Future embeddings, indexes, vector stores, caches, temporary files, summaries, logs, backups, and model contexts must preserve client and engagement namespaces, classification, purpose, retention, invalidation, and deletion state. Shared embedding/index/cache namespaces containing client material are prohibited.

An embedding is derived client content, not de-identification. Model memory is assumed untrusted, non-authoritative, non-auditable, and non-reusable unless represented as an explicit governed record. Opaque memory cannot be cited, promoted, or injected.

## Derived artifacts, prompts, and exports

Every derivative retains origin refs/versions, transformation description, producer/tool description without credentials, boundary, classification, purpose, limitations, freshness dependencies, invalidation links, retention, and audit. Transformation cannot reduce confidentiality, remove ownership limits, improve evidence status, or authorize reuse.

Prompts and context manifests are client-bound artifacts. Client instructions within them cannot override system governance. Exports are prepared artifacts only; an Export Manifest grants no transfer, delivery, publication, or external-action authority.

