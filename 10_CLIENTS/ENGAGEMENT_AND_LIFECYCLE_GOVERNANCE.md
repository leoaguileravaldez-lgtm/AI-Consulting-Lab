# Engagement and Lifecycle Governance

## Engagement binding

An Engagement Instance Binding references the exact Layer 03 engagement ID/version, exact Layer 09 client/entity association, client instance, engagement security subdomain, scope, purpose, classification, jurisdiction, validity, and audit lineage. Layer 10 cannot create or change engagement identity, scope, stage, gate, blocker, approval, or closure.

One client may have multiple engagements, but each has isolated contexts, materials, prompts, retrieval, embeddings, caches, analysis, deliverables, exports, and audits. Same client, team, problem, source, or method does not authorize cross-engagement movement.

## Lifecycle interaction

Layer 10 lifecycle describes instance availability and handling only. It cannot advance Layer 03 lifecycle. A Layer 03 scope, jurisdiction, entity, stakeholder-authority, classification, or purpose change invalidates affected Layer 10 bindings until revalidated.

Lifecycle transitions require exact prior record/version, transition reason, source, dependencies, classification, freshness, required canonical decisions, effective time, review deadline, and audit. Unknown or invalid transition is `LIFECYCLE_BLOCKED`.

Closure blocks new use but does not erase records. Reopening requires new validation of identity, entity, engagement, conflict, jurisdiction, confidentiality, access, freshness, retention, and authorization; prior state cannot be replayed.

## Hidden change control

Hidden or uncertain stakeholder, ownership, jurisdiction, contracting entity, purpose, or scope changes suspend affected reliance, recipient assumptions, access proposals, delivery association, and external-action preparation. Layer 10 emits a governed impact reference; certified owners decide upstream changes.

