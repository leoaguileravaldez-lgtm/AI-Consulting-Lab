# Evidence Propagation and Invalidation

## Ownership

`04_RESEARCH_EVIDENCE` owns source/evidence status records and evidence-impact notices. It does not own canonical task state, blockers, engagement readiness, approval invalidation decisions or transitions.

Deterministic flow:

`SOURCE CHANGE`
→ `EVIDENCE STATUS CHANGE`
→ `CLAIM RELIANCE IMPACT`
→ `01_ORCHESTRATOR NOTIFICATION`
→ `CANONICAL BLOCKER / REVALIDATION DECISION`
→ `03_ENGAGEMENTS REFLECTED READINESS`

## Trigger Events

Correction, revision, revocation, expiry, supersession, disappearance without adequate proof, provenance failure, common-source discovery, jurisdiction mismatch, methodological defect, contradiction, material reinterpretation, freshness failure or client-boundary contamination triggers impact analysis.

## Dependency Traversal

Traverse versioned edges from source and evidence to claims, assumptions, estimates, models, specialist outputs, contradictions, confidence records, recommendations, synthesis, decision packets, approvals, deliverables, KPIs, benefit claims and closure readiness.

For each dependent object, record direct/indirect dependency, materiality, affected proposition, prior/new support, required action and audit reference. Affected Material objects become `INVALIDATED` or `REVALIDATION_REQUIRED`; unaffected objects remain usable only after an explicit dependency test.

Prior exact-object approval cannot preserve reliance on a changed evidence basis. History remains immutable. Restoration requires corrected evidence, independent validation, recalculated confidence, applicable assurance and fresh exact approval.

## Interface to Canonical Control

The evidence-impact notice contains engagement/task IDs, source/evidence old/new versions, trigger, affected graph, materiality, proposed canonical evidence/validation block, freshness urgency and audit linkage. `01` independently evaluates and commits any `BLOCKED_EVIDENCE`, `BLOCKED_SOURCE_CONFLICT`, `BLOCKED_VALIDATION`, revalidation or rework transition. `04` cannot write or simulate that state.

`03` consumes canonical results and source references to mark readiness; it does not validate the evidence. Missing notification delivery, unknown dependency coverage or ambiguous ownership fails closed and is escalated.
