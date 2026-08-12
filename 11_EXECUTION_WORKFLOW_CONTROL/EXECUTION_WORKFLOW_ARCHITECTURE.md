# Execution Workflow Control Architecture

## Purpose and authority

`11_EXECUTION_WORKFLOW_CONTROL` defines governed execution-state representation for work items, transitions, dependencies, attempts, recurrence, queues, concurrency, exceptions, cancellation, rollback, and audit. It is subordinate to `00_CORE` and certified Layers 01–10.

Layer 01 remains the canonical orchestration and state-transition authority. Layer 11 may prepare, validate, and record execution-control proposals and exact Layer 01 transition references; it cannot independently transition canonical workflow state.

Layer 11 owns no facts, evidence, analysis, professional judgment, engagement scope, challenge, dissent, risk, QA, deliverable readiness/release, knowledge/reuse, client operations, client instantiation, access, approval, credential, connector, runtime, automation, or external action.

## Execution/substance separation

Execution state means only where governed work stands operationally. `READY`, `RUNNING`, or `COMPLETED` never means factually correct, analytically valid, evidentially verified, professionally approved, risk accepted, QA passed, deliverable released, client authorized, or Human Principal approved.

Every substantive prerequisite remains an exact reference to its certified owner. Layer 11 cannot create, reinterpret, waive, aggregate, infer, or substitute satisfaction.

Each Material Work Item owns an exhaustive, versioned declaration of exact Layer 11 Dependency Record references. Those records bind deterministically to exact upstream objects and their canonical authority layers. Downstream workflow objects may verify the declaration but cannot provide or replace it. `RELEASE` dependencies bind only to Layer 07 release-governance state and convey no release authority to Layer 11.

## Ownership matrix

| Authority | Canonical owner | Layer 11 boundary |
|---|---|---|
| Governance, security, approval, segregation, Human controls | Layer 00 | Inherit; never amend |
| Orchestration and canonical transitions | Layer 01 | Submit exact transition proposals; preserve decision refs |
| Analysis/professional judgment | Layer 02 | Typed dependency only |
| Engagement state/scope | Layer 03 | Exact-version dependency only |
| Evidence/provenance/confidence | Layer 04 | Exact-version dependency only |
| Challenge/dissent | Layer 05 | Preserve; cannot close or waive |
| Risk/QA/professional review | Layer 06 | Preserve; cannot accept, pass, or waive |
| Deliverable readiness/release/delivery | Layer 07 | Preserve; cannot release or deliver |
| Knowledge/reuse | Layer 08 | Preserve; cannot authorize |
| Client operations | Layer 09 | Exact validated state reference only |
| Client instance/isolation | Layer 10 | Preserve client/engagement security boundaries |

If ownership is unclear or rules conflict, the stricter certified rule controls and execution fails closed.

## Material objects

Layer 11 defines 14 Material object types: Work Item, Transition Request, Dependency, Readiness Assessment, Blocker, Execution Attempt, Recurrence, Timeout Event, Queue Entry, Concurrency Lease, Exception, Escalation, Cancellation/Rollback, and Workflow Audit Event.

Every object satisfies `MATERIAL_OBJECT_INVARIANT.md`. All are Markdown design records; none implements a workflow engine.

## Non-runtime declaration

This package creates no code, scheduler, worker, queue, lock, daemon, agent, API, connector, webhook, credential, timer, background task, message, runtime, external call, state mutation, or autonomous operation. Layer 12 remains separate.
