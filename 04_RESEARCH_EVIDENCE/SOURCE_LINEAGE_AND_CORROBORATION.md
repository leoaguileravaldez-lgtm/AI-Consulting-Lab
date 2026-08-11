# Source Lineage and Corroboration

## Lineage Objects

- `ROOT_SOURCE`: earliest identifiable original record, dataset, observation, testimony or authoritative text.
- `DERIVED_SOURCE`: content materially copied, transformed, summarized, analyzed or reported from another source.
- `INDEPENDENCE_GROUP`: sources sharing origin, dataset, authorship/research team, institutional control, funding, information channel or material methodology.
- `CORROBORATION_GROUP`: one evidentiary unit eligible to count once toward corroboration.
- `COMMON_SOURCE_RISK`: `NONE_IDENTIFIED`, `POSSIBLE`, `LIKELY`, `CONFIRMED`, or `UNKNOWN`, with reasons.

Each source records immediate parents, root candidates, derivation method, underlying data, authorship, funding/sponsorship, institutional dependencies, cited sources and unresolved lineage gaps. Unknown lineage never defaults to independent.

## Citation-Laundering Prevention

`Original dataset -> press release -> newspaper article -> consulting report -> specialist memo` remains one potential root lineage and normally one corroboration group. Downstream editorial variation does not create factual independence.

Circular citations are detected by graph-cycle checks. A cycle with no independently accessed root cannot support a Material claim. A derived source that omits its root is marked lineage-incomplete; citation metadata or rhetorical authority cannot cure it.

## Independent Corroboration Test

Sources count separately only when sufficiently independent in origin, underlying dataset, authorship, institutional dependence, funding, methodology and information channel. The test records each dimension and a reasoned result.

Multiple analyses of one dataset may test method robustness but do not establish independent factual corroboration. Independent datasets measuring the same proposition and genuinely separate methods may count separately when definitions and scope match. A primary source and a summary of it count once.

Canonical sole-authority treatment remains permitted only with documented status, limitation and eligible validator rationale. Critical source minimums remain governed by `00_CORE`.

## Efficiency

Research reuses verified public source identity metadata where authorized, but every engagement independently checks applicability, freshness, jurisdiction, access permission and client boundary. Reuse never carries forward claim support or validation automatically.
