# Derived Schemas, Transitions and DAG

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: object, transition, graph and reconciliation projection

The normative model declares 14 non-interchangeable identifier types, 8 enum families, 15 Material object types, 14 transition rules and 51 directed source-to-destination edge tuples. Each object schema is the direct projection of its fields, authority, cardinality, rank and prohibitions. Each transition is the direct projection of its sources, destination, cardinality and guard.

Mechanical reconciliation results:

- nodes: 15
- transition rules: 14
- directed edge tuples: 51
- roots: 1 (`AUTHORIZED_PREDECESSOR_BASELINE`)
- sinks: 1 (`REALIZATION_CONFORMANCE_SPECIFICATION`)
- direct or indirect cycles: 0
- self-loops: 0
- future references: 0
- unresolved mandatory nodes: 0
- rank violations: 0
- missing normative nodes or edges: 0
- extra derived nodes or edges: 0
- ambiguous authority edges: 0

All edges increase rank strictly. The final specification consumes exact reconciled sets; a summary, prose claim, inferred empty set or validation report cannot substitute for membership. Derived documents introduce no mandatory object, transition, authority, invariant or closure criterion.
