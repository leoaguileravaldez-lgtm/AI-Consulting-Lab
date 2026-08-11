# Stage Gates and Human Decisions

## Gate Nature

Engagement gates are governed decision records and materiality-based overlays. They do not add lifecycle states. A gate packet is a request for Human Principal judgment; only the canonical `01_ORCHESTRATOR` approval and transition machinery can record and effect the authorized result.

| Gate | Decision focus | Applicability |
|---|---|---|
| A — Acceptance / Problem Definition | authorize exact engagement/problem/scope | Mandatory before activation; proportional intake authority applies |
| B — Evidence Sufficiency | permit reliance on stated evidence for defined analysis | Material/Critical and evidence-sensitive Routine work |
| C — Analytical Direction | select material approach among alternatives | Material/Critical or disputed direction |
| D — Economic / Strategic Feasibility | accept stated feasibility basis for further reliance | Where economic/strategic claims are material |
| E — Risk / Implementation Feasibility | accept stated risk/feasibility basis, without delegating risk acceptance | Material implementation/exposure |
| F — Final Recommendation | decide internal reliance or require rework/rejection | All recommendation engagements; proportional for Routine |
| G — Delivery Authorization | approve exact artifact, recipient, channel and action | Any external delivery |
| H — Measurement / Closure | decide measurement disposition and closure | Outcome-producing work or closure review |

An inapplicable gate follows the documented `NOT_APPLICABLE` rule in the lifecycle document. Gate compression may reuse one packet only when each applicable decision, authority, object, version and disposition remains explicit.

## Decision Semantics

`GO`, `REVISE`, `HOLD`, and `STOP` are presentation labels, not canonical commands or states.

| Label | Required canonical expression |
|---|---|
| `GO` | Applicable exact-object approval and an independently evaluated existing transition, such as `APPROVE_INTERNAL` (N17) or `APPROVE_FOR_DELIVERY` (N18); it never guarantees transition |
| `REVISE` | N20 to `PLAN`, N21 to `ANALYZE`, or N22 to `REMEDIATE`, selected by `01` from the explicit defect |
| `HOLD` | Remain in `HUMAN_REVIEW` or enter the applicable existing `BLOCKED_*`/hold state; no timeout approval |
| `STOP` | N19 to `REJECTED` or C01 to `CANCELLED`, according to the recorded reason and scope |

If no safe canonical mapping exists, the disposition is invalid and produces `BLOCKED_AUTHORIZATION` or `BLOCKED_POLICY_CONFLICT`. A label cannot reinterpret, broaden, replay or amend a Human Principal decision.

## Decision Packet

Each Material gate packet gives the exact decision requested; recommendation and alternatives; evidence and freshness; fact/assumption/inference/scenario separation; dissent and contradictions; uncertainty and confidence; financial, operational, regulatory, technology, workforce and implementation implications as applicable; blocker status; exact objects/versions; approval category; and permitted dispositions with consequences.

The packet minimizes operator burden but preserves decisive detail. The Human Principal's signed decision record controls over summaries. Unavailability yields hold/block, never implicit approval or delegated judgment.

## Fail-Closed Conditions

Missing evidence, unresolved Material contradiction, failed validation, stale authorization, uncertain scope, unavailable assurance, or wrong object/version prevents reliance/progression. A deadline cannot override the blocker. External action also requires the distinct canonical delivery authorization and transaction controls.
