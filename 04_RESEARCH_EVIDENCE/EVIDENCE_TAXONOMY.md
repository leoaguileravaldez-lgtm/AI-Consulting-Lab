# Evidence Taxonomy and Canonical Claim Mapping

## Subordinate Labels

These research/presentation classifications map to, but do not modify, canonical `01_ORCHESTRATOR` claim semantics:

| Research label | Required canonical treatment |
|---|---|
| `FACT` | `VERIFIED_FACT`, only after eligible validation confirms direct support |
| `SOURCE_CLAIM` | Attributed assertion represented as `HYPOTHESIS`, `ESTIMATE`, or candidate fact pending validation |
| `ASSUMPTION` | `ASSUMPTION` with rationale, range, owner, expiry and dependencies |
| `INFERENCE` | `HYPOTHESIS` or `ESTIMATE` with premises, method, alternatives and confidence |
| `ESTIMATE` | `ESTIMATE` with method, range, uncertainty and as-of date |
| `SCENARIO` | Analytical artifact composed of explicit assumptions/estimates, conditions and horizon; not a new claim type |
| `EXPERT_JUDGMENT` | Evidence/source subtype; related claim remains assumption, estimate or hypothesis |

Canonical claim types remain exactly `VERIFIED_FACT`, `ASSUMPTION`, `ESTIMATE`, `HYPOTHESIS`, and `RECOMMENDATION`. No schema in `04` may add a competing canonical type.

## Legal Conversions

- `SOURCE_CLAIM` may become `FACT` only through a new canonical `VERIFIED_FACT` version after identity, retrieval, content-support, fitness, freshness, contradiction, corroboration and independent-validation checks pass.
- `ASSUMPTION` may become `FACT` only through new supporting evidence and independent validation; reuse, repetition, agreement or approval is insufficient.
- `INFERENCE` remains an inference even when premises are facts unless the proposition is directly evidenced and separately validated.
- An `ESTIMATE` remains an estimate; precision or later agreement does not retroactively make it a fact.
- A `SCENARIO` never becomes a prediction through presentation.
- `EXPERT_JUDGMENT` does not inherit professional authority outside verified qualification and scope.

## Illegal Conversions

Silent relabeling; copying an assumption into a fact field; treating client documentation as independent proof; treating model output as empirical fact; converting consensus into corroboration; and using Human approval as evidence validation are invalid. They require correction, dependency impact, and the applicable canonical evidence/validation block.

## Evidence Relationships

The only canonical relation semantics remain `SUPPORTS`, `CONTRADICTS`, `CONTEXT_ONLY`, and `INVALID`. Each relation identifies exact supported text, source coordinates, limitations, strength, validator and version.
