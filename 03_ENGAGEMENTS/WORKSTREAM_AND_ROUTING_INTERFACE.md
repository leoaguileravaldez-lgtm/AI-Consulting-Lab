# Workstream and Routing Interface

## Workstream Model

A workstream is a planning projection over one or more canonical `01_ORCHESTRATOR` parent/child Task records. It does not execute, delegate, approve, or own canonical status.

Each workstream records an ID, accountable question owner, question, canonical task references, inputs, dependencies, required evidence, assumptions, expected output, materiality, blockers, due date/priority, derived status, contradiction links and Human Principal dependencies.

## Routing Request

`03_ENGAGEMENTS` may submit a capability-need request containing engagement/task scope, question, deliverable need, sector/jurisdiction overlay, exposure, materiality, evidence uncertainty, independence constraints, timing and declared exclusions. Only `01_ORCHESTRATOR` may select registry-qualified actors, activate specialists, reserve capacity, issue access or create canonical handoffs.

The minimum sufficient team is the default. Optional specialists require a distinct unanswered question or control dependency. Mandatory specialists follow certified routing triggers. Escalation-triggered participation follows changed risk, evidence, contradiction or feasibility conditions. No engagement class activates every practice by default.

## Anti-Duplication

- One accountable owner per question and output; contributors have bounded subquestions.
- Shared evidence and assumptions are referenced, never recopied as new facts.
- Overlapping capability requests are consolidated by `01` before delegation.
- Finance, Strategy, Market, Operations, Technology and other practices retain their certified boundaries.
- Sector/jurisdiction overlays and quantitative/measurement capabilities are not primary practices or independent authorities.

## Delegation Controls

A workstream or specialist cannot create another workstream, recursively delegate, self-route, expand scope, nominate itself as validator, or directly request unauthorized peer work. All child-task and handoff proposals return to `01` for conflict, capability, context, access, capacity and independence checks. Rejected, absent or mismatched acceptance produces the applicable `BLOCKED_DELEGATION`/`BLOCKED_PERMISSION` route.

## Routing Tests

Single-domain work uses one primary practice plus proportional assurance. Strategy/market uses two bounded questions. Finance-heavy work adds Market only when demand/input claims matter. Industrial entry adds Strategy, Market, Operations and exposure-triggered Finance/Regulatory. AI transformation requires Technology plus commercial/operational/financial/regulatory practices only as material. Public-sector work requires the Public Sector practice and relevant overlays. Regulated work mandates Regulatory involvement. Critical multidisciplinary work reserves independent assurance but still excludes irrelevant practices.
