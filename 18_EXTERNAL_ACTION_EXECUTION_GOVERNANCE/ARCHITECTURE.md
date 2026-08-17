# External-Action Execution Governance

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: human-readable architecture projection

## Responsibility

Layer 18 governs the bounded transition from one exact predecessor-authorized external-action intent to one controlled execution attempt and immutable executor-side evidence. It may validate and consume authority but cannot originate, broaden, reinterpret or repair it.

The canonical path is:

`AUTHORIZED INTENT → EXACT SCOPE BINDING → HUMAN-AUTHORITY VALIDATION + EXECUTOR ASSIGNMENT + TARGET/ACTION/PARAMETER BINDING → NON-AUTHORIZING EXECUTION ELIGIBILITY → UNIQUE ATTEMPT → EXACT LAYER 14 EFFECT BARRIER → IMMEDIATE PRE-DISPATCH REVALIDATION → AT-MOST-ONE DISPATCH → APPEND-ONLY EXECUTION EVIDENCE → UNRESOLVED-EFFECT HOLD → NON-AUTHORITATIVE INDEPENDENT-RECONCILIATION HANDOFF → DESCENDANT AUDIT`.

No-dispatch attempts also produce explicit evidence and a handoff. Missing is never empty.

## Authority separation

Authorization remains with certified predecessor owners and the Human Principal boundary. Layer 14 remains the sole owner of protected-action runtime-enforcement semantics, including global effect-slot uniqueness and the immediate effect barrier. Layer 18 owns only execution-scope binding, executor/attempt governance, bounded dispatch, executor-side evidence, fail-closed hold and handoff.

Execution evidence is evidence, not external truth. Adapter acknowledgement, receipt, timeout, silence, local failure, telemetry, audit and prior success cannot prove no effect, exact effect, intended outcome or closure. Every dispatched attempt remains in `UNRESOLVED_EFFECT_HOLD` until a future separately governed authority acts.

Effect/Outcome Reconciliation and Closure remains institutionally separate, independently governed, unnumbered, outside Layer 18 and unauthorized for construction. Sequencing is dependency order only and creates no authority precedence.

## Retry, crash and partial execution

An initial attempt may be registered only after every exact authorization, Human, identity, target, parameter, predecessor and currentness guard passes. A retry preserves the same action, authority domain and effect slot and requires a future independent authoritative reconciliation disposition plus every separately required current authorization. The Layer 18 handoff alone never authorizes retry.

Crash after possible dispatch, timeout, missing receipt, partial report, contradictory evidence or any uncertainty preserves the possible effect, prohibits effect-bearing retry, creates a hold and requires independent handoff. Cancellation, rollback, compensation or remediation cannot erase history or imply no effect. Compensation/remediation is a new consequential action with its own authorization and attempt.

## Human Principal boundary

The exact repository-defined authorization applies to bounded non-consequential, Material/external, Tier 4, capital, legal/regulatory, public-statement, privacy/security-sensitive, irreversible and compensation/remediation actions. Required contemporaneous Human Principal authority cannot be cached, generalized, inferred, proxied or replaced by a delegate, technical capability, admission, workflow state or emergency. Unknown authority denies.

## Fail-closed behavior

Missing, ambiguous, mismatched, stale, expired, revoked, superseded or cross-boundary authority; identity or executor uncertainty; target or parameter drift; absent or consumed Layer 14 barrier; unresolved prior possible effect; duplicate attempt or dispatch; incomplete evidence; or failed separation produces denial, quarantine, hold, escalation or revalidation—never dispatch, retry, effect, outcome or closure authority.

