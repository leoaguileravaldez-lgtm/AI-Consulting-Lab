# Derived Schemas and Transitions

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.2`  
DERIVATION_TYPE: schema and transition projection

The normative model declares 18 non-interchangeable identifier types, 10 enum families, 23 Material object types, and 35 transition rules. Each object schema is the direct projection of its `required_fields`, authority owner, cardinality, rank, and prohibitions. Each transition schema is the direct projection of source types, destination type, cardinality, and guard.

No generic identifier may substitute for a typed identifier. No omitted field means “not applicable”; policy-defined non-applicability must be represented by the governing cardinality/guard. Any unknown enum, missing required field, wrong parent engagement, wrong mandate version, unauthorized producer/reviewer, or out-of-rank predecessor is invalid.

Materiality gates are bounded:

- every finding set receives a proportionate producer-distinct validation disposition, bounded challenge, and independent verdict before recommendation;
- `MATERIAL` and `CRITICAL` require every declared cross-validation dimension, full adversarial Challenge, and full independent validation before recommendation and delivery;
- materiality may strengthen evidence and decision authority but cannot relax any invariant or expand recursively during certification.

The transition catalog is exhaustive. `CONCLUSION` is the typed product of current synthesis, mandate, exact validated findings, and Challenge. Its identity, provenance, scope, lifecycle generation/state, freshness basis, and authority domain are distinct structural fields. Independent validation and recommendation require the same exact current Conclusion ID/version set and matching use domain. Record references alone do not create a transition. Termination transitions are explicit terminal branches; audit transitions are observational and cannot be used as authority sources.
