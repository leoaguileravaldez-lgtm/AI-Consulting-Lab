# Orchestrator Architecture

| Field | Value |
|---|---|
| Design version | 0.1.0-draft |
| Status | Design only; not operationally authorized |
| Owner and approver | Human Principal |
| Governing baseline | `00_CORE` version 0.2.0-draft, Approved for Design |

## Mission

The Orchestrator is a deterministic coordination and control plane that converts an authorized consulting question into traceable workstreams, evidence-backed analysis, independent challenge, validation, risk review, synthesis, and a Human Principal decision package.

It optimizes for **decision quality**, not task completion. Completion is invalid if evidence, challenge, validation, security, audit, or approval controls are incomplete.

## Binding Operating Sequence

For every Material or Critical recommendation:

```text
PRIMARY ANALYSIS
→ INDEPENDENT CHALLENGE
→ EVIDENCE VALIDATION
→ RISK REVIEW
→ ORCHESTRATOR SYNTHESIS
→ HUMAN PRINCIPAL DECISION
```

The Orchestrator may coordinate and synthesize. It cannot substitute for an independent validator, qualified human specialist, approver, or executor.

## Architectural Layers

1. **Human authority plane:** Receives intake authorization, exceptions, escalations, exact-version approvals, and final decisions.
2. **Control plane:** Enforces state transitions, materiality, permission tiers, separation of duties, engagement scope, and approval gates.
3. **Work plane:** Routes bounded analytical tasks to specialist divisions.
4. **Assurance plane:** Separates challenge, evidence validation, quantitative reproduction, risk review, and synthesis-integrity review from primary analysis and synthesis.
5. **Records plane:** Maintains typed, referentially linked evidence, claim, assumption, risk, audit, decision, approval, incident, and deliverable records.
6. **Security plane:** Enforces immutable engagement binding, least privilege, data minimization, circuit breakers, and disabled-by-default external capabilities.
7. **Trusted authority plane:** Separately controls identity, permissions, approval writes, registry administration, and audit append/verification so specialists and synthesis cannot grant themselves authority.

These are logical boundaries only. No runtime, agent, storage system, or external connection is authorized by this design.

## Authority Boundary

The future Orchestrator may, within an authorized engagement and tier:

- validate intake completeness;
- propose task classification and plans;
- decompose work and select eligible specialists;
- request research, analysis, challenge, and validation;
- preserve contradictory findings;
- calculate confidence under the approved rubric;
- synthesize findings without suppressing minority views;
- prepare approval and decision records;
- stop, contain, and escalate.

It must not:

- make or represent a final strategic decision;
- approve, validate, or grant an exception to its own work;
- alter governance, identity, permission, approval, or audit records;
- elevate a permission tier or expand engagement scope;
- infer approval from access, silence, urgency, or prior action;
- perform Tier 4 actions autonomously;
- combine client contexts or reuse client information across engagements;
- retry a material action without fresh authorization.

## Decision-Quality Invariants

Every workflow must preserve these invariants:

1. One task belongs to exactly one engagement context.
2. Every actor and action has an explicit maximum permission tier.
3. Materiality can rise automatically but cannot be lowered without approved exception.
4. Claim types remain distinct: Verified Fact, Assumption, Estimate, Hypothesis, Recommendation.
5. Material conclusions cannot bypass independent challenge and validation.
6. Primary preparer, challenger, validator, approver, and executor are separately identifiable.
7. A recommendation cannot exceed the confidence of a decision-critical supporting claim.
8. Approval category and authority are explicit; business approval never silently becomes technical, deployment, governance, or exception approval.
9. Approval binds to an exact artifact/action version and expires upon Material change.
10. State transitions append audit events; prior history is not overwritten.
11. Every Material assertion has validated claim-to-evidence coverage.
12. Synthesis is independently checked against primary, challenge, validation, risk, contradictions, and Human Principal requirements.
13. Control uncertainty produces a blocked state, not optimistic continuation.

## Logical Components

| Component | Responsibility | Prohibited role |
|---|---|---|
| Intake Controller | Completeness, engagement identity, initial authorization | Cannot approve missing scope |
| Classification Controller | Applies Material/Critical triggers and risk domains | Cannot lower classification autonomously |
| Planner | Decomposes work and defines acceptance criteria | Cannot grant specialist permissions |
| Registry Resolver | Finds eligible capabilities and independence constraints | Cannot self-register an agent |
| Workflow Controller | Enforces transition guards and handoffs | Cannot override a failed guard |
| Assurance Coordinator | Routes challenge, validation, and risk review | Cannot count synthesis as validation |
| Synthesis Controller | Integrates supported findings and dissent | Cannot suppress unresolved conflict |
| Synthesis Integrity Reviewer | Compares final package with all upstream assurance records | Cannot be the primary preparer or synthesizer |
| Approval Controller | Builds exact-version approval requests | Cannot approve or fabricate approval |
| Audit Controller | Appends attributable events | Cannot delete or rewrite prior events |
| Security Boundary Controller | Checks engagement, tier, data class, and tool boundary | Cannot authorize a new external system |

## Policy Precedence

All design behavior is subordinate to the precedence stated in `00_CORE/operating_principles/OPERATING_PRINCIPLES.md`: applicable law and binding client obligations, then Security Policy, Approval Policy, Quality Standards, Source Validation, and Operating Principles. A conflict routes to `BLOCKED_POLICY_CONFLICT`; the Orchestrator must not select the less restrictive interpretation.

## Canonical Operational-Capacity Model

Capacity controls preserve governance under load; they never remove challenge, validation, security, evidence, or approval gates.

### Queues and priority

Work is partitioned by engagement and control stage into intake, primary analysis, challenge, evidence validation, analytical validation, risk review, synthesis integrity, Human Review, delivery authorization, and recovery queues. Queue entries contain task/classification, deadline, source-freshness deadlines, required qualifications, separation constraints, estimated effort, age, and breaker status.

Controlled priority is `P0_INCIDENT_CONTAINMENT`, `P1_CRITICAL_GOVERNANCE`, `P2_MATERIAL_TIME_BOUND`, `P3_MATERIAL_STANDARD`, then `P4_ROUTINE`. Priority cannot bypass prerequisites. Aging may raise scheduling priority within a governance class but cannot lower classification, qualifications, or review.

### Admission and maximum work in progress

Each stage, engagement, specialist pool, validator qualification class, and Human Principal queue has an approved maximum work-in-progress limit. Numeric limits require future Human Principal approval based on measured capacity. Admission reserves mandatory downstream scarce roles, not merely primary-analysis capacity.

When capacity is unavailable, the task enters `BLOCKED_CAPACITY` at its current canonical state before new work is delegated. The system cannot substitute an ineligible validator, combine prohibited roles, reduce assurance, or silently extend approval/freshness windows.

### Validator and Human Principal routing

- Maintain qualification-specific validator pools with current independence, conflict, and availability status.
- Reserve Critical human-specialist capacity separately from Routine or AI capacity.
- Route delegable approval only to a current authorized delegate within category, engagement, limit, and expiry.
- Route non-delegable decisions only to the Human Principal.
- Batch presentation may consolidate related packages, but every decision/approval remains separately identified and version-bound.
- The capacity controller cannot approve prioritized work.

### Backpressure and safe degradation

Backpressure first stops admitting lower-priority Routine work, then pauses unstarted work, then limits new engagement activation when downstream mandatory capacity is unavailable, then escalates deadline/freshness/client-obligation conflicts. Overload threatening control integrity opens a stage or engagement capacity breaker.

Safe degradation permits read-only status, evidence preservation, incident containment, and authorized cancellation. It prohibits skipped review, lower-quality validators, implicit approval, stale-source reliance, or Tier 3/4 execution.

### Freshness aging

Evidence-dependent queue entries record `next_freshness_deadline` from the strictest decision-critical source. Monitoring sets the capacity queue-record status `REFRESH_REQUIRED` before expiry; this is not a lifecycle state. Work cannot enter validation, Human Review, or delivery with expired evidence; it returns to evidence validation and invalidates affected downstream records.

### Capacity escalation and ownership

Unavailable mandatory review capacity produces an escalation recording required qualification, queue age, deadline/freshness exposure, alternatives, and risk. The Human Principal may reprioritize, use a qualified delegate where policy permits, revise scope/schedule, or cancel, but cannot declare an ineligible validator independent or bypass Critical review.

The `CAPACITY_CONTROLLER` owns queue/WIP enforcement, not classification or approval. Required telemetry includes arrival rate, wait time, WIP, validator utilization, Human Principal queue age, freshness expiry, capacity blocks, rework, and incident load. Capacity limits/configuration are versioned and audited.

### Runtime measurement and calibration obligations

Design language about reliability, recovery, retries, duplicate prevention, or capacity is a control requirement, not an operational performance guarantee. Future implementation must emit engagement-safe telemetry sufficient to measure transition attempts/failures, guard failures by control, retry attempts and outcomes, duplicate attempts and suppressions, lease conflicts and stale-owner rejections, approval reservation/consumption/replay rejection, queue age and freshness aging, breaker openings, recovery attempts/duration/outcomes, reconciliation backlog, and unknown external outcomes. Metric labels must not contain client content, credentials, source text, or cross-engagement identifiers accessible outside the authorized boundary.

Alert thresholds, WIP limits, lease/heartbeat timing, retry limits, recovery objectives, and SLO/SLA inputs must be calibrated from post-implementation testing and observed operating data. Until approved, absence of a calibrated parameter cannot be interpreted as permission to weaken a guard, add a retry, extend freshness/approval, or execute during overload. Each numerical parameter is an **implementation-calibration parameter requiring future Human Principal approval**, with owner, rationale, measurement window, scope, version, effective date, alert route, and review/rollback condition recorded. No numerical reliability or failure-rate target in this design is an achieved guarantee.

Calibration review must compare observed failure and recovery behavior with approved control objectives, investigate material deviations, and adjust only through versioned governance/configuration approval. SLO/SLA pressure cannot override separation of duties, evidence sufficiency, approval, isolation, audit, or fail-closed requirements.

## Success Criteria

A design is ready for implementation planning only when schemas, canonical transitions, trusted authority boundaries, verifiable validation independence, enforceable engagement isolation, claim-level evidence integrity, synthesis integrity, audit integrity, recovery, scalability, and threat controls are internally consistent and traceable to `00_CORE`. This document does not authorize implementation or operation.
