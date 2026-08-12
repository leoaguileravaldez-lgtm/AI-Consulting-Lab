# Clients Architecture

## Purpose

`10_CLIENTS` governs client-specific instantiation of the certified consulting architecture. A client instance binds an exact Layer 09 client, legal entity, and Layer 03 engagement to a purpose-limited security and confidentiality domain. It provides architecture for organizing references to client materials and derived work without turning this repository into a client-data store.

Layer 10 is subordinate to `00_CORE` and certified Layers 01 through 09. It creates no client, legal entity, engagement, evidence, analysis, professional judgment, recommendation, deliverable approval, knowledge promotion, operational state, access grant, connector, credential, runtime, automation, or external action.

## Separation model

The following remain distinct and cannot be collapsed by a common identifier, folder, search result, narrative, or model context:

1. institutional architecture and blank methods;
2. Layer 09 client identity and legal-entity identity;
3. Layer 03 engagement identity and scope;
4. client-provided assertions, materials, and operational context;
5. Layer 04 sources, claims, and evidence;
6. Layer 02 analysis and professional judgment;
7. recommendations and Layer 05 challenge/dissent;
8. Layer 06 risk, QA, exceptions, and professional review;
9. Layer 07 deliverables, release, and delivery;
10. Layer 08 reusable institutional knowledge;
11. Layer 09 operational metadata;
12. confidentiality, access, retention, and disposition controls;
13. Human Principal decisions.

References preserve separation; they never transfer authority or transform object type.

## Non-negotiable invariants

- Every Material Layer 10 object satisfies `MATERIAL_OBJECT_INVARIANT.md`.
- One client instance has one immutable `client_instance_id`, canonical Layer 09 `client_id`, exact legal-entity scope, one client security domain, and explicit Layer 03 engagement bindings.
- Each engagement has a distinct context boundary even when engagements share a client or legal entity.
- Prospect, rejected, inactive, closed, and active instances never share state by implication.
- Client statements and materials are unverified inputs until Layer 04 independently governs their evidentiary use.
- Layer 04 is the exclusive evidence authority. Layer 10 cannot manufacture, validate, upgrade, downgrade, reinterpret, replace, suppress, or supersede evidence.
- Client preference, CRM state, commercial importance, repetition, retrieval rank, model confidence, and Human Principal approval cannot create factual validity or analytical truth.
- Client-bound content is non-reusable by default. Layer 08 exclusively governs knowledge promotion and reuse eligibility.
- Cross-client and cross-engagement movement defaults to deny across facts, documents, evidence, analysis, recommendations, confidential information, prompts, contexts, embeddings, search results, caches, summaries, deliverables, exports, and derived artifacts.
- Unknown, stale, incomplete, contradictory, mixed, foreign, malformed, unauthorized, or jurisdictionally changed Material state fails closed.
- Human Principal authority is explicit, exact-scope, contemporaneous where required, auditable, revocable, and non-proxyable.

## Canonical ownership

| Concern | Canonical owner | Layer 10 role |
|---|---|---|
| Core policy, security, approval, materiality | Layer 00 | Inherit without amendment |
| Workflow, authority, blockers, audit orchestration | Layer 01 | Reference exact decisions and route blocks |
| Analysis and professional judgment | Layer 02 | Preserve references and object type only |
| Engagement identity, scope, lifecycle, change control | Layer 03 | Bind exact engagement/version; never create or amend |
| Sources, claims, evidence, provenance, confidence | Layer 04 | Reference only; client inputs require Layer 04 intake |
| Challenge and dissent | Layer 05 | Preserve without suppression or closure |
| Risk, QA, exceptions, professional review | Layer 06 | Preserve references; cannot classify, accept, waive, or close |
| Deliverables, release, delivery | Layer 07 | Bind artifact references; no release/delivery authority |
| Knowledge, de-identification, generalization, reuse | Layer 08 | Submit exact candidates only; cannot authorize |
| Client operations, client/entity records, stakeholders, CRM metadata | Layer 09 | Consume validated exact-version references only |
| Client-specific instantiation and boundary manifests | Layer 10 | Own within this architecture only |

If ownership is unclear or controls conflict, the stricter certified rule controls and affected use stops.

## Material object model

Layer 10 defines 18 Material object types:

1. Client Instance Record;
2. Legal Entity Binding;
3. Related-Party Edge;
4. Engagement Instance Binding;
5. Client Lifecycle Transition;
6. Client Information Asset Record;
7. Client Assertion Record;
8. Client Material Manifest;
9. Context Boundary Manifest;
10. Retrieval Result Manifest;
11. Derived Artifact Manifest;
12. Export Manifest;
13. Retention State Record;
14. Disposition Request;
15. Knowledge Extraction Request;
16. Conflict and Related-Party Review Record;
17. Client Instance Decision Record;
18. Client Instance Audit Event.

Every object is metadata/reference architecture. Raw client content, credentials, and Restricted values are excluded from this Markdown package.

## Non-runtime declaration

This package is architecture only. It provides no storage, tenant, database, index, vector store, cache, model memory, identity provider, permission engine, CRM, browser, API, connector, scheduler, webhook, agent, messaging, export, deletion, delivery, publication, or execution capability. Future implementation requires separate certification and Human Principal authorization.

