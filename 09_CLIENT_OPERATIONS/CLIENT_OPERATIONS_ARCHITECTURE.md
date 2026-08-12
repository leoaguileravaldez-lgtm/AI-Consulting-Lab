# Client Operations Architecture

## Purpose and authority

`09_CLIENT_OPERATIONS` defines client-specific operational records needed to identify a client, associate engagements, represent stakeholders and interactions, track opportunities and commercial metadata, and preserve confidentiality, conflicts, preferences, and change history. It is subordinate to `00_CORE` and certified Layers 01 through 08.

Layer 09 owns only client-operational record structure and operational-state proposals. It does not own engagement truth, evidence, analysis, professional judgment, challenge, risk, QA, deliverable content, knowledge promotion, access control, approval, release, delivery, publication, runtime, credentials, connectors, or external action.

Layer 09 is a governed system-of-record design, not a CRM implementation. This Markdown package creates no storage, service, automation, communication channel, permission, or runtime.

## Non-negotiable invariants

1. A client-operation fact is not evidence and cannot become evidence by repetition, CRM status, relationship strength, preference, account value, or Human Principal attention.
2. Operational state may route a question or raise a blocker; it cannot answer an analytical question or change a certified-layer record.
3. Every client-bound object has one canonical client identifier, one legal-entity context where applicable, explicit engagement associations, classification, purpose, status, version, freshness, and audit lineage.
4. Client and engagement boundaries are deny-by-default. Similar names, common ownership, shared personnel, or related work do not merge boundaries.
5. Client-bound knowledge is non-reusable by default. Layer 08 eligibility and separate authorization are required before any cross-boundary reuse.
6. Unknown, contradictory, stale, duplicate, foreign, downgraded, or materially incomplete state fails closed.
7. Human Principal decisions are explicit, scoped, version-bound, auditable, revocable records. Layer 09 cannot infer, delegate, automate, or proxy them.
8. Commercial importance and client pressure never suppress evidence, dissent, limitations, risk, QA, professional review, or contradictory information.
9. A Layer 09 record never manufactures provenance, freshness, applicability, validation, certainty, permission, or approval.
10. Layer 09 references certified records; it never edits or supersedes them.

## Ownership boundaries

| Concern | Canonical owner | Layer 09 relationship |
|---|---|---|
| Workflow, authority, approvals, blockers, audit | Layer 01 | Submit proposals and preserve exact decision references |
| Specialist analysis and professional judgment | Layer 02 | No authority; may supply labeled operational context |
| Engagement identity, scope, lifecycle, gates | Layer 03 | Reference canonical engagement and change-control records |
| Sources, claims, evidence, provenance, confidence | Layer 04 | No write or interpret authority; operational assertions require independent intake if analytically used |
| Independent challenge and dissent | Layer 05 | Cannot suppress, soften, prioritize, or close |
| Risk, QA, exceptions, professional review | Layer 06 | Raise facts and pressure events; cannot classify, accept, remediate, or waive |
| Deliverable content, readiness, release, delivery | Layer 07 | Communication metadata is not a deliverable or authorization |
| Institutional knowledge and reuse eligibility | Layer 08 | Submit candidates only; client-bound content remains non-reusable until governed approval |
| Client-operational records | Layer 09 | Canonical owner within the limits of this package |
| Access, storage, connectors, automation | Future separately certified layers | No authority is created or implied |

If ownership is uncertain or rules conflict, the stricter certified control governs and the affected operation stops.

## Architectural domains

### Identity domain

Maintains canonical opaque `client_id`, client/account record, legal-entity identifiers, aliases, parent/affiliate relationships, identity status, verification basis, jurisdiction, and duplicate/ambiguity cases. Names and CRM identifiers are attributes, never canonical identity.

### Association domain

Records explicit many-to-many associations between a client/legal entity and canonical Layer 03 engagements. Each association states role, scope, effective period, classification, authorization reference, and status. Association never transfers facts, permissions, evidence, or conclusions between engagements.

### Relationship domain

Represents contacts, stakeholders, roles, declared authority, interactions, communications, preferences, and operational history. A contact's title, influence, or apparent authority is not approval authority. Hidden or uncertain stakeholder changes block affected reliance.

### Opportunity and commercial domain

Represents opportunity stage, value ranges, commercial terms metadata, billing references, and status. It excludes substantive invoices, payment instruments, bank details, tax credentials, contract execution, and accounting authority. Commercial metadata cannot influence analytical or assurance state.

### Protection and governance domain

Records confidentiality classification, handling restrictions, conflicts, reuse restrictions, freshness, contradictions, decision requests, and audit events. Classification is a label consumed by separately governed access controls; it does not grant access.

## Canonical object model

| Object | Purpose | Required boundary |
|---|---|---|
| Client Account Record | Stable operational identity and status | `client_id` |
| Legal Entity Record | Exact contracting/represented entity and jurisdiction | `client_id`, `legal_entity_id` |
| Identity Resolution Case | Duplicate, alias, ambiguity, merge/split review | candidate IDs and quarantine boundary |
| Engagement Association | Exact Layer 03 linkage | `client_id`, `legal_entity_id`, `engagement_id` |
| Contact/Stakeholder Record | Person/role representation | client and authorized engagement scope |
| Interaction Record | Append-only relationship event metadata | client and engagement when relevant |
| Communication Record | Communication metadata and governed content reference | client, engagement/purpose, classification |
| Opportunity Record | Pre-engagement commercial lifecycle | client/legal entity and isolated opportunity ID |
| Commercial Metadata Record | Terms/billing descriptors and references | client/legal entity and engagement where applicable |
| Preference Record | Operational presentation/process preference | client and explicit scope; non-analytical |
| Confidentiality Profile | Highest applicable handling restrictions | client/legal entity/engagement scope |
| Conflict Record | Actual, potential, apparent, or unknown conflict | parties, scope, status, escalation |
| Reuse Authorization Request | Request for governed reuse review | origin client/engagement and exact object/version |
| Client State Decision | Human Principal decision on client state | exact object/version, scope, conditions, decision record |
| Operational Context Packet | Advisory, exact-purpose cross-layer handoff | source and destination boundaries |
| Operational Contradiction Record | Preserve competing operational assertions | all affected client/entity/engagement boundaries |
| Pressure Event Record | Preserve attempted governance influence | affected client/engagement and records |
| Change Impact Notice | Route potential revalidation effects | changed object and all affected boundaries |
| Audit Event Record | Preserve immutable event lineage | exact object/action boundary |

All records are explicit, versioned, append-audited, and referentially constrained. Every Material object must satisfy the common schema invariant in `SCHEMA_INVARIANT_AND_VALIDATION.md`; a domain schema may add controls but may not omit them silently. Narrative notes cannot substitute for required structured fields or canonical references.

## Operational lifecycle

The allowed descriptive states are `PROPOSED`, `IDENTITY_REVIEW`, `ACTIVE`, `RESTRICTED`, `DORMANT`, `CLOSED`, `QUARANTINED`, and `SUPERSEDED`. `UNKNOWN`, missing, contradictory, or unsupported states are treated as `QUARANTINED` for use, without rewriting the source value.

State changes are proposals until an authorized Human Principal decision or already-certified policy basis is referenced. No state grants permission to contact, access, contract, bill, deliver, publish, or act externally. Closure does not erase retention, confidentiality, conflict, or audit obligations.

## Read-only cross-layer interface

Layer 09 may emit a labeled operational-context packet containing exact record/version references, classification, freshness, contradictions, limitations, and purpose. The receiving certified layer decides whether and how to use it under its own controls.

Operational assertions needed as analytical inputs must enter Layer 04 through the canonical research/evidence process. Layer 09 remains only the origin reference and cannot declare source quality, corroboration, confidence, applicability, or evidentiary status.

Layer 09 may receive status references from other layers for coordination. A cached or summarized status is non-canonical, must identify its source version and as-of time, and cannot change the upstream record.

## Human Principal control

Human Principal authority is required for Material identity resolution; client merge or split; legal-entity interpretation; engagement association or reassociation; confidentiality downgrade; reuse authorization; conflict disposition; Material scope or jurisdiction change; and exceptions permitted by certified policy.

Authority is not a license to rewrite truth. A decision must preserve the request, exact objects and versions, alternatives, contradictions, risks, conditions, scope, effective time, expiry/revocation, rationale/reference, decision-maker identity, and audit reference. Self-approval and inferred approval are invalid.

## Prohibited effects

Layer 09 must never create, modify, reinterpret, suppress, promote, downgrade, or replace Layer 04 evidence; alter certified analytical conclusions; manufacture provenance, freshness, applicability, professional validation, or certainty; translate preference, CRM status, revenue, relationship, or opportunity value into evidentiary authority; suppress dissent, risk, QA, limitations, or contradiction; grant professional-review authority; grant approval, release, delivery, publication, access, connector, credential, runtime, or external-action authority; or create a Human Principal proxy.

Any attempted prohibited effect is rejected, preserved as an auditable boundary event, and escalated under Layers 01 and 06 as applicable.
