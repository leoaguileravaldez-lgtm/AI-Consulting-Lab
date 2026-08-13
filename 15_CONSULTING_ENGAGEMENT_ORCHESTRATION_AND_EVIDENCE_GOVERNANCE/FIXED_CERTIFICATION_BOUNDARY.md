# Fixed Certification Boundary

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.2`  
DERIVATION_TYPE: certification-boundary projection

Layer 15 is sufficient when invariants A–R in the normative model hold in every valid abstract history, the derived graph is acyclic and predecessor-complete, no structural counterexample produces an invalid governed outcome, and Layers 00–14 remain unchanged.

Findings use exactly four categories:

1. Certification-blocking architectural defect: a declared invariant fails in a valid abstract history and can produce an invalid protected outcome without first requiring failure of a deferred runtime mechanism.
2. Implementation/runtime obligation.
3. Optional architectural hardening.
4. Documentation/non-material.

Only Category 1 blocks closure. Production databases, persistence, cryptography, consensus, locking, network isolation, specialist agents, LLM orchestration, APIs, portals, CRM, billing, deployment, observability, business performance, and external execution are outside this architectural certification boundary.

The boundary is unchanged in model version 1.0.2. Validation may falsify its invariants but may not add new closure criteria.
