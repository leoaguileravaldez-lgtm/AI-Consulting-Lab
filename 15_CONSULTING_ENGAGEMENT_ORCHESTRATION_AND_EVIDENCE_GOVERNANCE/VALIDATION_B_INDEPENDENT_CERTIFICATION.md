# Validation B — Fresh Independent Certification

Historical pre-correction evidence for normative model 1.0.0. Its conclusion was superseded by the Human Principal conclusion-lifecycle review and must not be treated as current certification evidence.

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: independent certification evidence

Validation B did not use Validation A conclusions as evidence. It reconstructed the model from the normative object types, transition rules, ranks, predecessor contracts, and fixed boundary.

Independent reconstruction found 22 Material objects, 33 transition rules, 44 edge tuples, one authority/admission root, zero duplicate type identities, zero unresolved required types, zero backward or same-rank edges, zero audit-to-authority edges, zero cycles, zero self-loops, and zero future references. The independently reconstructed graph equals the published derived graph.

The fresh suite contained 4,656 cases:

| Attack family | Cases | Fail closed |
|---|---:|---:|
| Mandate and scope bypass | 320 | 320 |
| Evidence laundering | 480 | 480 |
| Claim/analysis lineage breaks | 420 | 420 |
| Self-certification and independence | 360 | 360 |
| Recommendation and failed-validation laundering | 360 | 360 |
| Delivery and handoff authority | 320 | 320 |
| Termination and resurrection | 300 | 300 |
| Cross-layer authority leakage | 320 | 320 |
| Cycles, future references, and audit authority | 1,056 | 1,056 |
| Three-control valid-looking compositions | 720 | 720 |

Retested development counterexamples: 2. Both were blocked: a synthesis/independent PASS with unmet mandate evidence standards, and omission-based bypass of applicable Layer 14 protected-action authority. Still successful: 0.

Invariant results: A–R PASS. Successful structural counterexamples: 0. Category 1 = 0. Category 2 = 8 deferred implementation families. Category 3 = 0. Category 4 = 0. No finding depends on optional stronger typing or production infrastructure for architectural closure.

Independent verdict: PASS under the fixed Layer 15 boundary. Testing converged; additional test-count growth is not warranted.
