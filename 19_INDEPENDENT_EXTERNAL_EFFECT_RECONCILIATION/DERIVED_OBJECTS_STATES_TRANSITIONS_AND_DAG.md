# Derived Objects, States, Transitions, and DAG

This is a non-normative projection of `CANONICAL_MODEL.json`.

## Inventory

- Identifier types: 18
- Enum families: 9
- Enum values: 38
- Material object types: 17
- Transition rules: 18
- DAG edges: 50

## Ranked object types

| Rank | Object |
|---:|---|
| 10 | `AUTHORIZED_RECONCILIATION_INPUT` |
| 20 | `RECONCILIATION_SCOPE_BINDING` |
| 30 | `TARGET_SYSTEM_IDENTITY` |
| 40 | `EXPECTED_EFFECT_SPECIFICATION` |
| 50 | `OBSERVATION_REQUIREMENT_SET` |
| 60 | `INDEPENDENT_OBSERVER_ASSIGNMENT` |
| 70 | `EXTERNAL_OBSERVATION_SET` |
| 80 | `OBSERVATION_PROVENANCE_CURRENTNESS_ASSESSMENT` |
| 90 | `BEFORE_AFTER_STATE_COMPARISON` |
| 100 | `EFFECT_PRESENCE_COMPLETENESS_CARDINALITY_ASSESSMENT` |
| 110 | `INDEPENDENT_RECONCILIATION_VERDICT` |
| 120 | `RETRY_PREREQUISITE_DISPOSITION` |
| 125 | `REMEDIATION_COMPENSATION_NEED_FINDING` |
| 130 | `HUMAN_ESCALATION_RECORD` |
| 140 | `RECONCILIATION_INVALIDATION_RECORD` |
| 150 | `DOWNSTREAM_RECONCILIATION_HANDOFF` |
| 160 | `FINAL_RECONCILIATION_AUDIT` |

## State/verdict dimensions

The nine enum families and all values are normative only in the canonical model. The material verdict dimensions are effect presence (4), completeness (4), cardinality (4), reconciliation process (3), observation status (5), lifecycle status (5), retry-prerequisite status (2), finding status (4), and failure disposition (7). These dimensions must not be collapsed into a single success state.

## Transition inventory

`T01` binds scope; `T02` identifies target; `T03` binds expected effect; `T04` establishes complete observation requirements; `T05` assigns independent observers; `T06` accepts an explicit observation generation; `T07` assesses provenance/currentness; `T08` compares before/after state; `T09` assesses effect dimensions; `T10` issues the independent verdict; `T11` issues the non-authorizing retry prerequisite; `T12` issues the non-authorizing remediation finding; `T13` records Human escalation; `T14` hands off a current verdict; `T15` invalidates affected reliance; `T16` hands off invalidation; and `T17`–`T18` create descendant audits.

Each source-to-destination tuple is one DAG edge. Every edge moves to a strictly greater rank. The graph has one root (`AUTHORIZED_RECONCILIATION_INPUT`), one sink (`FINAL_RECONCILIATION_AUDIT`), no cycle, self-loop, future reference, unresolved mandatory node, ambiguous authority edge, or backward authority edge.
