# Validation A — Construction-Team Review

Historical pre-correction evidence for normative model 1.0.0. Superseded for current-state conclusions by `CONCLUSION_LIFECYCLE_LOCALIZED_REVIEW.md`.

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: construction validation evidence

Validation A parsed the normative model, independently projected every source/destination edge, checked identifier uniqueness, resolved every predecessor type, enforced strict rank monotonicity, and topologically sorted the graph.

Results: 22 nodes, 33 transition rules, 44 directed edge tuples, one root (`L15_AUTHORIZED_INPUT`), 22 sorted nodes, zero cycles, zero self-loops, zero future references, zero missing destinations/sources, zero unresolved mandatory nodes, and zero rank violations.

The final construction mutation suite generated 1,555 cases after two pre-certification construction findings were corrected in the normative source: evidence-standard-to-verdict gating and typed Layer 14 applicability.

| Mutation family | Cases | Fail closed |
|---|---:|---:|
| Required-field omission | 227 | 227 |
| Mandatory-predecessor omission | 44 | 44 |
| Wrong predecessor type | 924 | 924 |
| Rank/future inversion | 44 | 44 |
| Authority removal/substitution | 44 | 44 |
| Explicit prohibition bypass | 63 | 63 |
| Invalid enum/classification | 62 | 62 |
| Cross-engagement/mandate binding | 66 | 66 |
| Duplicate exclusive/terminal state | 81 | 81 |

All invariants A–R passed. Targeted semantic cases blocked mandate omission/ambiguity, workstream scope expansion, assertion-to-fact laundering, derived-as-primary evidence, missing claim lineage, producer self-validation, challenge mutation, unsupported recommendation, validation bypass, failed-verdict delivery, subordinate termination override, retroactive handoff authority, and audit-created authority.

Findings: Category 1 = 0; Category 2 = 8 deferred implementation families; Category 3 = 0; Category 4 = 0 after correcting one derived edge-count transcription during construction. Successful structural counterexamples = 0.

This is a construction-team result and is not used as evidence by Validation B.
