# Source Taxonomy

## Multidimensional Classification

No single label establishes authority, reliability, independence, applicability, freshness, or confidence. Every source is assessed across separate dimensions:

1. `source_type`: regulatory, government, company, academic, industry/professional, media/news, client, expert, commercial provider, web publisher, or other.
2. `provenance_class`: `PRIMARY`, `DERIVED`, `AGGREGATED`, `SYNTHETIC`, or `UNVERIFIED`.
3. `content_form`: document, record, dataset, interview, observation, model output, database result, media item, or webpage.
4. `source_authority`: `AUTHORITATIVE`, `RECOGNIZED`, `LIMITED`, `UNVERIFIED`, with scope.
5. `source_quality`: dimensional result under the quality model.
6. `source_independence`: root, independence and corroboration groups plus conflicts.
7. `source_freshness`: controlled freshness status and as-of dates.
8. `jurisdictional_relevance`: exact jurisdiction and applicability result.
9. `methodological_transparency`: `PASS`, `LIMITED`, `FAIL`, or `UNKNOWN`.

## Requested Categories

| Category | Allowed role | Potential ceiling, never automatic |
|---|---|---|
| Primary source | Direct evidence when authentic, fit and current | High possible |
| Regulatory source | Official applicable law/guidance within jurisdiction | High possible |
| Academic source | Original or secondary research according to lineage | High possible |
| Dataset | Evidence subject to origin, definitions and transformations | High possible |
| Authoritative secondary | Support/context where direct source unavailable | Medium alone |
| Secondary analysis | Context, discovery and qualified support | Medium |
| Industry source | Market evidence with method/sponsor review | Medium |
| Client-provided source | Client assertion/documentation pending independent testing | Medium after testing |
| Expert interview | Judgment/context, not population fact by itself | Medium alone |
| Model output | Estimate/scenario; never proof of its own inputs | Medium |
| Media/news | Attributed current reporting or discovery | Medium where transparent |
| Web content | Depends on identity and method | Low or Medium |
| Unverified source | Discovery only | Low; no independent Material support |

The canonical `00_CORE` hierarchy controls whenever these categories are applied. Rank never overrides fitness. Search snippets, anonymous claims, aggregators and AI summaries cannot independently verify a Material claim.

## Client Status

Client information is separately labeled `CLIENT_ASSERTED`, `CLIENT_DOCUMENTED`, `INDEPENDENTLY_VERIFIED`, or `UNVERIFIED`. Client origin never automatically becomes independent validation. The independently verified label applies only to the tested proposition, definitions, period and version.
