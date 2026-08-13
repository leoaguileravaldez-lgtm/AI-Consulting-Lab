# Derived DAG and Reconciliation

NORMATIVE_SOURCE: `FORMAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.1`  
DERIVATION_TYPE: transition-source edge projection and rank validation

Every source expression in a transition rule expands its closed union. Each expanded source produces one edge to that rule's destination. Foundation nodes use epoch 0; initial attempts use epoch 1; retry/recovery attempts use the preceding retry decision at epoch n and begin epoch n+1. No human-selected edge or phase is permitted.

Two-attempt phase expansion results:

| Metric | Result |
|---|---:|
| Typed node classes | 42 |
| Expanded nodes | 77 |
| Expanded mandatory edges | 211 |
| Direct cycles | 0 |
| Indirect cycles | 0 |
| Self-loops | 0 |
| Mutual-authority cycles | 0 |
| Future references | 0 |
| Unresolved same-epoch nodes | 0 |
| Unsorted nodes | 0 |
| Manual phase bindings | 0 |
| Missing edges | 0 |
| Extra authority edges | 0 |
| Ambiguous edges | 0 |
| Rank violations | 0 |

The causal order is uniquely constrained by `(attempt_epoch, phase_ordinal, event_ordinal)`. Independent same-phase context events may have multiple harmless linear serializations, but the edge set and partial order are unique. The E07→E31 edge is same attempt epoch, `CONTEXT(20) → BREAK_GLASS_TRANSFORMATION(21)`.

Structural equality:

| Representation | Material count | Equality |
|---|---:|---|
| Normative model node classes | 42 | source |
| Derived schema objects | 42 | exact |
| Derived transition objects | 42 | exact |
| Derived DAG node classes | 42 | exact |
| Reconciliation rows | 42 | exact |

Model-only, schema-only, transition-only, DAG-only, or reconciliation-only Material objects: 0. Manual synchronization sources: 0.
