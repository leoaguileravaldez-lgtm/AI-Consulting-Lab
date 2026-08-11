# Client Boundary and De-identification

## Default

Every engagement-derived knowledge candidate is `CLIENT_BOUND_NON_REUSABLE` by default. Missing restriction or authorization metadata never means reusable. Similarity, time, repetition, client-name removal, aggregation, prior success or Human preference cannot authorize cross-engagement use.

Every candidate carries originating engagement, client/security domain, classification, purpose, reuse restriction, de-identification/generalization records, authorization and audit lineage.

## Separate operations

De-identification, generalization and reuse authorization are distinct. Successful de-identification does not establish generalization. Successful generalization does not grant reuse authority. Layer 08 records results but cannot self-authorize any operation.

Re-identification review tests combinations of geography, sector, revenue/scale, timing, project facts, counterparties, personnel, staffing, vendors, operating structure, transaction values, rare facts, event sequence and metadata. Removing direct identifiers is insufficient.

Material or unknown re-identification risk yields `NOT_REUSABLE`, `REVIEW_REQUIRED` or `QUARANTINED`.

Client facts, models, recommendations, charts, documents, prompts, summaries, notes, metadata, embeddings and caches cannot cross boundaries. Reusable templates/methods require verified sanitation and authorization.

