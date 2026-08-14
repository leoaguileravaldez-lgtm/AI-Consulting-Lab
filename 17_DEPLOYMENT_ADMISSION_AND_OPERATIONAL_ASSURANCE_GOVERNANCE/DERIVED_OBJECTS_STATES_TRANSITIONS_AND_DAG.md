# Derived Objects, States, Transitions and DAG

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: schema, state, transition, graph and reconciliation projection

The normative model declares 17 typed identifiers, 8 enum families, 16 Material object types, 16 transition rules, 20 explicit prohibited-transition families and 32 directed source-to-destination edge tuples. Object fields, authorities, cardinalities, ranks and prohibitions project directly. State families remain separate; documentation may not merge them.

Mechanical reconciliation results:

- nodes: 16
- transition rules: 16
- directed edge tuples: 32
- roots: 1 (`AUTHORIZED_ADMISSION_INPUT`)
- sinks: 1 (`FINAL_ASSURANCE_AUDIT`)
- cycles, self-loops, future references: 0
- unresolved mandatory nodes: 0
- rank violations: 0
- missing or extra derived nodes/edges: 0
- ambiguous authority edges: 0
- competing normative authorities: 0

All edges increase rank strictly. `ASSURANCE_INVALIDATION_RECORD` is descendant invalidation evidence and cannot revive prior state. `FINAL_ASSURANCE_AUDIT` is an observational sink. No node represents deployment execution, production observation, effect, outcome or reconciliation.
