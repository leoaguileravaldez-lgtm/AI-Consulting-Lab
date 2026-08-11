# Engagement Operating-System Architecture

## Authority and Purpose

`03_ENGAGEMENTS` is a non-executable, referential case-file and planning aggregate over the canonical Engagement entity and Task records governed by `01_ORCHESTRATOR`. It is not a second engagement system of record. If a local summary conflicts with its cited canonical record, the canonical record controls and the summary is invalid.

Authority dependency is `Human Principal -> 00_CORE -> 01_ORCHESTRATOR -> 02_SPECIALISTS -> 03_ENGAGEMENTS`; this is precedence, not delegated authority. This package adds no state, transition, approval category, permission, runtime role, specialist activation, validation, risk acceptance, external action, or lifecycle authority.

## Functions

The aggregate may organize:

- the decision problem, engagement profile, plan, and workstream map;
- references to canonical tasks, evidence, assumptions, outputs, contradictions, decisions, approvals, deliverables, measurements, and audit events;
- derived lifecycle phase, gate readiness, dependency impact, change history, and closure readiness;
- concise, version-bound decision packets for the Human Principal.

It may not route actors, validate evidence, challenge conclusions, perform Risk/QA, synthesize or deliver externally, operate a CRM, execute automation, or certify outcomes. Those functions remain with their certified or future independent owners.

## Source-of-Truth Rule

Every referenced field records `source_module`, `source_object_id`, `source_version_or_hash`, `observed_at`, and `freshness_status`. Cached values are display-only. They must be mechanically refreshed or marked `STALE`; they may never overwrite a source record.

Authoritative ownership is:

| Information | Authority |
|---|---|
| Engagement identity, activation, task state, transitions, routing, approvals, audit and delivery | `01_ORCHESTRATOR` |
| Specialist analysis, declared assumptions, methods and outputs | `02_SPECIALISTS` under `01` routing |
| Problem contract, workstream plan, gate profile, change-impact view and closure-readiness view | `03_ENGAGEMENTS` |
| Evidence validation, challenge, Risk/QA and deliverable objects | Their future independent modules, coordinated by `01` |
| KPI methods and benefit analysis | Certified `02` transversal capability; source data and assurance remain independent |

## Fail-Closed Invariants

- Unknown, stale, missing, conflicting or unresolvable authority references block affected Material/Critical reliance.
- A phase or readiness label cannot cause a canonical transition.
- A gate record cannot substitute for an exact-object approval.
- A workstream cannot delegate or activate another workstream or specialist.
- Deadlines, commercial pressure, apparent consensus and Human Principal unavailability never waive controls.
- No client-specific object may cross its engagement security boundary without explicit canonical authorization.

## Module Interfaces

`03` submits bounded planning and capability-need requests to `01`; `01` decides classification, routing, role eligibility, assurance, state transitions and authorization. Specialists return outputs only through canonical task/handoff records. Future modules receive schema-compatible references but acquire no authority from those references.

## Audit Minimum

All material changes, derived calculations, source versions, invalidations, gate packets, contradictions, Human Principal dispositions, closure checks and access decisions require append-only audit linkage sufficient to reconstruct who knew what, when, from which source, and under which exact authorization.
