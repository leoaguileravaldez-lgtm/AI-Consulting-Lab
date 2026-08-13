# Formal Foundation Traceability Matrix

NORMATIVE_SOURCE: `FORMAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.1`  
DERIVATION_TYPE: primitive/axiom/failure-prevention projection

| Clean-design construct | Formal basis | Historical failure prevented |
|---|---|---|
| Strong identifier types | stable identity/value domains | action splitting/collision and interchangeable IDs |
| AuthorityWitness | external-authority primitive | self/recursive authority and Human substitution |
| TypedEvent | family+lifecycle+phase+epoch identity | generic phase and same-epoch ambiguity |
| TransitionRule | explicit source/destination/cardinality/guard | inferred transitions, fake roots and future edges |
| ClosureWitness | externally authorized exact set | self-attested completeness and parallel local histories |
| StateProjection | deterministic closed-history function | self-attested currentness and competing tips |
| Authority Mandate | Layer 13-bound transformation with SOD | unsupported upstream guarantees and authority amplification |
| EffectSlotId | action-derived global exclusivity domain | local idempotency, duplicate consumption/effects |
| PEP topology/admission events | typed cardinality-one transitions | alternate-route and self-declared mediation |
| Revocation closure event | typed authority/ancestor closure | stale cached descendants and circular rechecks |
| Target/pre-effect events | exact generation plus revalidation | TOCTOU and target substitution |
| E31 break-glass transformation | Human predecessor, later phase, same domain | emergency authority creation, recursion and Human bypass |
| Effect barrier/effect/outcome/audit | strict typed sequence | post-hoc authority and audit repair |

Every Material construct maps to a formal primitive or historical prevention requirement. Unmotivated Material objects: 0.
