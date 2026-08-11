# Claim–Evidence Contract

## Canonical Relationship

This contract operationalizes the canonical claim/evidence graph in `01_ORCHESTRATOR/EVIDENCE_CONFIDENCE_AND_RISK.md` and its schemas. It does not redefine canonical claims, confidence, blockers or workflow.

Every Material claim records:

- claim ID, exact text, canonical claim type and subordinate research label;
- engagement, task and workstream references;
- materiality and decision-critical flag;
- supporting, contradicting, contextual and invalid evidence relations;
- assumptions, inference method and calculation/model dependencies;
- root lineage, independence and corroboration groups;
- jurisdiction, population, definitions, units and time scope;
- confidence and rationale from its canonical owner;
- evidence validation result, limitations and freshness;
- current versions/hashes and audit linkage.

## Evidence Assessment Results

`SUPPORTED`, `PARTIALLY_SUPPORTED`, `CONTRADICTED`, `INSUFFICIENT_EVIDENCE`, and `NOT_VERIFIABLE` are subordinate evidence-assessment results. They are not task states, confidence levels, approval decisions, or permission to rely.

- `SUPPORTED`: applicable evidence directly supports the stated claim portion and all required retrieval, fitness, freshness, lineage, corroboration, contradiction and validation controls pass.
- `PARTIALLY_SUPPORTED`: defined portion is supported; gaps and prohibited extrapolations are explicit.
- `CONTRADICTED`: credible conflicting evidence materially weakens or opposes the proposition.
- `INSUFFICIENT_EVIDENCE`: available evidence cannot meet the applicable threshold.
- `NOT_VERIFIABLE`: the proposition cannot presently be tested using authorized, accessible and fit evidence.

## Coverage Rules

A Material claim needs at least one eligible direct source, canonical corroboration or sole-authority treatment, valid retrieval proof for every decision-critical source, visible credible contradictions, matched definitions/period/population/units and eligible independent validation. Failure produces an evidence-impact notice and the applicable canonical route.

The overall recommendation cannot exceed the confidence of its lowest-confidence decision-critical claim. `04` supplies evidence findings but does not set or approve recommendation confidence.

## Integrity

Claim text changes create a new version and require relation reassessment. Evidence supporting only a narrower proposition cannot be cited for a broader one. Persuasive wording, citation volume, client preference, specialist consensus or prior approval cannot upgrade support.
