# Audit, Traceability, and Fail-Closed Control

## Audit

Every create, read proposal, bind, classify, retrieve, derive, correct, supersede, transition, conflict event, reuse request, export preparation, retention/disposition proposal, denial, pressure event, incident, decision request, and invalidation notice requires an append-only audit reference with actor/role, time, exact object/version, client/entity/engagement/security boundary, purpose, classification, action, basis, authorization, outcome, reason, and correlation.

Layer 10 cannot alter or delete audit history. Audit content minimizes sensitive information and never contains credentials. Missing or mixed audit lineage blocks the affected Material object.

## Traceability

Required chains include:

- Layer 09 client/entity -> Layer 10 instance/binding -> Layer 03 engagement;
- provider/source event -> material/assertion manifest -> Layer 04 intake/evidence reference;
- boundary manifest -> retrieval result -> context -> analysis reference -> challenge/QA -> deliverable association;
- origin object -> derivative/export -> destination/retention/disposition;
- knowledge candidate -> Layer 08 reviews -> authorization -> destination Layer 04 validation;
- change/contradiction/conflict -> impact notice -> owner decision -> revalidation or continued block.

Every edge preserves IDs, versions, types, boundaries, purpose, classification, creator, time, and audit. A link cannot bypass content authorization.

## Fail-closed matrix

| Condition | Mandatory result |
|---|---|
| Duplicate/unknown client | `IDENTITY_BLOCKED`; quarantine; no instance use |
| Ambiguous legal entity/related party | Block entity-dependent activity and preserve candidates |
| Missing/malformed classification | Highest plausible restriction; `BOUNDARY_BLOCKED` |
| Missing provenance/freshness/review/lifecycle | `INCOMPLETE_BLOCKED`, `STALE_BLOCKED`, or `LIFECYCLE_BLOCKED` |
| Unsupported client assertion | Keep unverified; route to Layer 04; no factual reliance |
| Material contradiction | `CONTRADICTED_BLOCKED`; no inferred resolution |
| Cross-client/engagement result | Deny, suppress, quarantine, contain, audit, escalate |
| Unauthorized reuse/aggregation | `REUSE_BLOCKED`; quarantine origin and derivative |
| Unsafe de-identification | Non-reusable; no promotion or sharing |
| Confidentiality downgrade | Retain higher restriction until exact authorized review |
| Jurisdiction/ownership/stakeholder change | Suspend affected use and revalidate |
| Prompt, embedding, cache, export leakage | Deny and contain as boundary incident |
| CRM/commercial/preference pressure | Record pressure; no truth, assurance, or authority effect |
| Automation/external-action attempt | Do not execute; preserve request and escalate |
| Human Principal proxy attempt | Reject; require exact valid canonical decision |

Every Material uncertainty means no reliance, no retrieval/context use, no boundary crossing, no reuse, no downgrade, no lifecycle transition, no disposition, and no external action.

