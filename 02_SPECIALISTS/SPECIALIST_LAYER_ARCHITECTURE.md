# Specialist Layer Architecture

| Field | Value |
|---|---|
| Status | Design only; not operationally authorized |
| Design version | 0.1.0-draft |
| Owner and approver | Human Principal |
| Governing baseline | Certified `00_CORE`; certified `01_ORCHESTRATOR` v1.1 |

## Mission

`02_SPECIALISTS` defines modular subject-matter practices that produce bounded, evidence-linked analysis for selection and coordination by `01_ORCHESTRATOR`. The layer improves decision quality; it does not approve decisions, operate the Orchestrator, or execute actions.

## Authority and Precedence

Applicable law and binding client obligations apply first, followed by the policy precedence and canonical ownership defined in `00_CORE` and `01_ORCHESTRATOR`. If this package conflicts with either certified baseline, the baseline controls and the affected work enters the applicable policy, authorization, evidence, validation, or security block. The less restrictive interpretation is prohibited.

This package adds no state, transition, approval category, permission tier, control role, or execution authority. Practice, capability, module, actor, role, and session identifiers remain distinct under `01_ORCHESTRATOR/SPECIALIST_REGISTRY.md`.

## Canonical Components

### Primary practices

- `SP_STRATEGY_CORP_DEV`
- `SP_FINANCE_VALUATION_CAPITAL`
- `SP_MARKET_COMPETITIVE_INTELLIGENCE`
- `SP_OPERATIONS_SUPPLY_CHAIN`
- `SP_TECHNOLOGY_DATA_AI`
- `SP_COMMERCIAL_GROWTH`
- `SP_REGULATORY_POLICY`
- `SP_PUBLIC_SECTOR_INSTITUTIONAL`
- `SP_ORGANIZATION_WORKFORCE_CHANGE`

### Transversal analytical capabilities

- `TC_QUANTITATIVE_DECISION_SCIENCE`
- controlled sector and jurisdiction qualification overlays

Transversal capabilities attach to a named primary practice owner. They cannot own the substantive recommendation, create authority, or independently validate their own Material output.

### External control interfaces

Research/evidence validation, Independent Challenge, Risk/QA, Deliverables, and Client Lifecycle remain future independent modules. `01_ORCHESTRATOR` coordinates their canonical roles and records. `02_SPECIALISTS` neither owns nor substitutes for them.

## Operating Sequence

Specialist work follows the certified sequence:

```text
AUTHORIZED INTAKE AND PLAN
→ SELECTIVE SPECIALIST ROUTING
→ RESEARCH / EVIDENCE ACQUISITION
→ PRIMARY SPECIALIST ANALYSIS
→ INDEPENDENT CHALLENGE
→ SPECIALIST REMEDIATION WITH DISSENT PRESERVED
→ INDEPENDENT EVIDENCE VALIDATION
→ INDEPENDENT ANALYTICAL VALIDATION
→ RISK REVIEW
→ ORCHESTRATOR SYNTHESIS
→ SYNTHESIS-INTEGRITY REVIEW
→ HUMAN PRINCIPAL DECISION
```

Research and producer self-checks may precede challenge. They are not formal independent validation. Formal validation follows challenge and remediation.

## Decision-Quality Invariants

1. Every work item belongs to exactly one authorized engagement.
2. The Orchestrator activates only specialists required by the decision and dependency graph.
3. Each question has one accountable primary owner.
4. Facts, assumptions, inferences, scenarios, and recommendations remain distinguishable.
5. Material claims are evidence-linked under the certified claim-to-evidence model.
6. No specialist is sole validator of its own Material or Critical conclusion.
7. Material quantitative output is reproducible by an eligible independent actor.
8. A downstream specialist cannot silently promote an unvalidated input to fact.
9. Contradictions remain visible until resolved or explicitly accepted as a non-Material limitation.
10. Unresolved Material contradictions that could change a recommendation or risk acceptance block reliance and release.
11. Confidence cannot exceed the lowest-confidence decision-critical claim.
12. Human Principal approval is explicit, exact-object-bound, and never inferred from silence, urgency, access, or prior approval.
13. Specialist work grants no external, financial, legal, deployment, publication, or commitment authority.
14. Engagement segregation, least privilege, and credential boundaries apply to every handoff and artifact.

## Work Decomposition

The Planner defines parent and child work items with bounded questions, inputs, outputs, acceptance criteria, dependencies, materiality, risk domains, permission ceilings, assurance requirements, and Human Principal decision dependencies. Parallel work is permitted only when it does not consume an unresolved upstream conclusion as established fact.

The Orchestrator may reuse current, authorized, same-engagement artifacts by exact version and lineage. It must not merge client contexts, silently reuse stale evidence, or treat a shared source as independent corroboration.

## Success and Failure

The layer succeeds when it supplies decision-ready, auditable analysis without unnecessary activation or authority leakage. Completion is invalid when required evidence, challenge, validation, risk review, qualification, security, audit, or approval is incomplete.

Failure of authority, evidence, independence, confidence, qualification, security, engagement binding, or required capacity invokes the corresponding certified fail-closed path. Specialists may preserve evidence and prepare an escalation but cannot route around the failure.

## Non-Operational Status

These Markdown documents create no executable agents, permissions, tools, credentials, external connectivity, client processing, deployment, or approval. Operational activation requires separate implementation design, conformance testing, security review, and Human Principal authorization.
