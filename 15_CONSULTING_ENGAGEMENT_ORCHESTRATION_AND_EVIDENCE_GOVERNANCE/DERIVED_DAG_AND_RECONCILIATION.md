# Derived DAG and Reconciliation

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.2`  
DERIVATION_TYPE: graph and reconciliation projection

The graph is derived mechanically: one node per `object_types` entry and one edge per `(transition source, transition destination)` tuple. Rank is read only from each object type; every edge must increase rank strictly.

Construction-team derivation results:

- nodes: 23
- transition rules: 35
- directed edge tuples: 51
- direct cycles: 0
- indirect cycles: 0
- self-loops: 0
- future references: 0
- unresolved mandatory nodes: 0
- rank violations: 0
- missing normative edges: 0
- extra derived edges: 0
- ambiguous edges: 0

Topological order is rank order, with same-rank absence guaranteed because no edge connects equal ranks. Multiple termination sources converge on the single terminal disposition and never point backward. `FINAL_AUDIT` is a sink.

Reconciliation keys are exact object IDs and exact edge tuples. Model objects, schema objects, transition destinations, DAG nodes, and reconciliation objects all equal the same 23-ID set. Documentation introduces no additional mandatory predecessor, authority, transition, or cardinality.
