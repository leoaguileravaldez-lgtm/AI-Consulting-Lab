# Derived Objects, States, Transitions and DAG

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: exact structural projection

## Inventory

- Identifier types: 13.
- Enum families: 7.
- Material object types: 15.
- Transition rules: 18.
- Invariants: 25 (`A`–`Y`).
- Fixed falsification classes: 25.
- Historical counterexample families: 10.
- Deferred Category 2 implementation families: 11.
- Predecessor contracts: 18.
- Normative truth sources: 1.

## Ranked objects

| Rank | Object | Authority character |
|---:|---|---|
| 10 | `AUTHORIZED_EXTERNAL_ACTION_INTENT` | predecessor authorization only |
| 20 | `EXECUTION_SCOPE_BINDING` | Layer 18 exact binding only |
| 30 | `HUMAN_AUTHORITY_VALIDATION` | predecessor Human authority, Layer 18 validation only |
| 40 | `EXECUTOR_ASSIGNMENT` | predecessor identity authority, Layer 18 validation only |
| 50 | `TARGET_ACTION_PARAMETER_BINDING` | target source plus Layer 18 exact binding |
| 60 | `EXECUTION_ELIGIBILITY_DECISION` | Layer 18 non-authorizing eligibility |
| 70 | `EXECUTION_ATTEMPT_REGISTRATION` | Layer 18 attempt registration |
| 80 | `LAYER14_EFFECT_BARRIER_BINDING` | Layer 14 authority, Layer 18 consumption only |
| 90 | `PRE_DISPATCH_REVALIDATION` | Layer 18 validation only |
| 100 | `EXECUTION_DISPATCH_RECORD` | bounded Layer 18 dispatch |
| 110 | `EXECUTION_EVIDENCE_SET` | non-authoritative executor evidence |
| 120 | `UNRESOLVED_EFFECT_HOLD` | fail-closed hold only |
| 130 | `INDEPENDENT_RECONCILIATION_HANDOFF` | non-authoritative handoff only |
| 140 | `EXECUTION_INVALIDATION_RECORD` | source revocation/invalidation projection |
| 150 | `FINAL_EXECUTION_AUDIT` | observational only; not consequential closure |

## State separation

`AuthorizationStatus`, `AttemptStatus`, `DispatchEvidenceStatus`, `RetryDisposition`, `LifecycleStatus` and `FailureDisposition` remain independent. No local dispatch evidence state includes `EFFECT_SUCCEEDED`, `NO_EFFECT_CONFIRMED`, `OUTCOME_SUCCEEDED`, `RECONCILED` or `CLOSED`.

## DAG reconciliation

Every `transition_rules.sources[] → destination` tuple is an edge. The graph has 15 nodes, 18 transition rules and 39 source-edge tuples. Every edge moves to a strictly greater rank. A retry enters only through a new predecessor-authorized intent generation carrying an exact future external authoritative disposition; Layer 18's prior handoff is not an authority source and no backward edge is created. Every terminal registered attempt without a dispatch has a direct mandatory route to explicit no-dispatch evidence and handoff, including barrier-binding, revalidation or invalidation failures.

Roots: `AUTHORIZED_EXTERNAL_ACTION_INTENT`. Sinks: `FINAL_EXECUTION_AUDIT`. The handoff is not a closure authority; it reaches only descendant audit inside Layer 18. Cycles: 0. Self-loops: 0. Same-rank edges: 0. Unresolved source/destination IDs: 0. Competing normative sources: 0.

Exact comparison with `CANONICAL_MODEL.json` is mandatory. This projection cannot add an effect, outcome, reconciliation, retry or lifecycle-closure state.
