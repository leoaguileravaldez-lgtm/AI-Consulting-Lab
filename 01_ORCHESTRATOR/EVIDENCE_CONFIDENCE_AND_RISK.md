# Claim, Evidence, Confidence, and Risk Architecture

## Canonical Scope

This document is the canonical design for claims, sources, provenance, corroboration, contradictions, confidence, and risk findings. Source and confidence policy in `00_CORE` controls.

## Claim-to-Evidence Graph

Every Material deliverable assertion must resolve through this graph:

```text
DELIVERABLE ASSERTION
→ CLAIM
→ SUPPORT / CONTRADICT / CONTEXT RELATION
→ EVIDENCE ITEM
→ SOURCE IDENTITY + RETRIEVAL PROOF + CONTENT LOCATION
→ VALIDATION RESULT
```

A claim cannot be marked `VERIFIED_FACT` solely because its source record is complete. Verification requires proof that an eligible validator accessed the identified source and confirmed that the cited location supports the claim.

## Source Identity and Provenance

Each source requires issuer/owner identity, title, source type, primary/secondary status, publication/reporting dates, canonical or stable location, access timestamp, content location, relevant definitions/population/geography/units, methodology, sponsorship/conflicts, and provenance chain.

For each decision-critical source, retrieval proof must record:

- independent retriever identity and session;
- retrieval timestamp and resolved location;
- source identity checks performed;
- content fingerprint or authorized snapshot reference where lawful and practical;
- page/table/section/cell or exact structured-data coordinates;
- concise support description, not an invented quotation;
- supersession/correction check;
- access outcome.

`NOT_ACCESSED`, `IDENTITY_UNCONFIRMED`, `CONTENT_MISMATCH`, or `SUPERSEDED` cannot support a Material claim.

## Source Reliability and Freshness

Source quality follows the hierarchy in `00_CORE/source_validation/SOURCE_VALIDATION.md`. Fitness is evaluated on authority, directness, methodology, currency, relevance, independence, consistency, and integrity. No composite score may conceal failure of a decision-critical dimension.

Freshness gates remain 24 hours for changeable facts before external delivery/action, 30 days for the latest appropriate company/government release recheck, and an explicit continued-relevance rationale for other sources older than 12 months. Freshness is measured against source currency/supersession, not merely retrieval time.

## Evidence Relationship and Coverage

Every claim-evidence relation is one of `SUPPORTS`, `CONTRADICTS`, `CONTEXT_ONLY`, or `INVALID`. Each relation records the supported portion, limitations, strength, validator, and timestamp.

A Material claim passes evidence coverage only when:

- at least one eligible source directly supports it;
- corroboration satisfies `00_CORE`, or sole-authority status is documented;
- every decision-critical source has valid retrieval proof;
- credible contradictions are linked and dispositioned;
- definitions, period, population, and units match or are reconciled;
- the validator confirms support rather than metadata plausibility.

Missing coverage causes `BLOCKED_EVIDENCE`.

## Qualitative Evidence Standards

Qualitative evidence must be reproducible as an evidentiary process even when the observation itself cannot be recreated. Every Material qualitative claim records collection purpose, question/protocol, collector, participant/source selection, date/context, consent/authorization where applicable, original notes/transcript/reference, coding/synthesis method, negative cases, limitations, conflicts, and transformation from observation to claim.

### Interviews and qualitative customer evidence

- Identify interview type, participant role, selection method, relevant population, interviewer, date, protocol version, and whether statements are direct observation, recollection, opinion, or hearsay.
- Do not expose participant identity beyond authorized need; pseudonymization does not remove engagement controls.
- Record nonresponse, sponsor/interviewer bias, incentives, power dynamics, and whether participants may represent the broader population.
- A single interview or anecdote cannot establish a population-level Material fact. It may support a Hypothesis or contextual claim unless independently corroborated.
- Material thematic conclusions require multiple relevant perspectives or a sole-expert rationale, documented saturation/coverage assessment, contrary cases, and independent review of the coding/synthesis.

### Expert judgment

- Record expert identity/qualification, scope, conflicts, compensation/incentives, information provided, elicitation method, assumptions, uncertainty range, and dissent.
- Distinguish expert inference from verified external fact.
- Decision-critical expert judgment requires an independent expert, empirical corroboration, or explicit sole-expert limitation with confidence reduction and Human Principal visibility.

### Surveys

- Record target population, sampling frame/method, field dates, instrument/version, question wording/order, response rate, sample size, weighting, exclusions, missing-data treatment, sponsor, and analysis method.
- Disclose coverage, nonresponse, selection, measurement, survivorship, and weighting bias.
- Do not generalize beyond the sampled population without a defensible inference method.
- Material estimates require uncertainty intervals or an explicit reason they cannot be computed; convenience samples cannot be described as representative.

### Observational evidence and market research

- Record observation protocol, setting, duration, observer, inclusion/exclusion criteria, data lineage, market/geography/time definitions, and known behavior changes caused by observation.
- Vendor or syndicated research must disclose methodology, sample, sponsorship, definitions, and relevant exclusions; opaque methodology lowers confidence and cannot independently verify a decision-critical Material claim.
- Triangulate Material market conclusions across methodologically independent evidence where practical; repeated reports derived from one underlying dataset count as one lineage.

### Causal marketing claims

- Label a claim causal only when design addresses treatment/exposure definition, counterfactual, timing, randomization or identification strategy, confounding, selection, interference, attribution window, statistical/practical significance, and external validity.
- Pre/post change, platform attribution, correlation, or self-reported influence alone supports association, not incrementality.
- When causal identification is inadequate, label the finding `ASSOCIATION` or `HYPOTHESIS`, reduce confidence, and prevent causal wording from entering synthesis.

### Corroboration, conflict, and disclosure

Material qualitative conclusions must link to source records and claim-evidence relations, include disconfirming observations, disclose sample/transferability limitations, and be reviewed by an eligible validator who examines original authorized material or a privacy-preserving verification sample. Conflicting testimony is recorded as contradictory evidence, not silently averaged. Sensitive qualitative material remains engagement-isolated and minimum necessary.

## Contradiction Control

Contradiction records identify competing claims/sources, definition/period/population/method/lineage differences, materiality, resolution attempts, conclusion impact, confidence adjustment, owner, and status. The only terminal statuses are `RESOLVED`, `ACCEPTED_NON_MATERIAL_LIMITATION`, or `UNRESOLVED`.

An unresolved Material contradiction:

- cannot be removed by synthesis;
- caps the affected conclusion below High;
- appears in the decision package and deliverable limitations;
- blocks release if it could change the recommendation or risk acceptance.

## Confidence Control

High, Medium, and Low criteria are inherited exactly from `00_CORE`. The confidence record must identify decision-critical claims, their confidence, key assumptions, evidence limitations, contradiction status, sensitivity results, validation result, assigning actor, and independent confirmation.

The overall recommendation takes the lowest confidence among decision-critical claims. The risk reviewer or validator may add a claim to the decision-critical set; the primary preparer cannot remove one unilaterally. Removal requires recorded rationale and independent confirmation.

Material/Critical work requires Medium or High to enter Human Review. Low-confidence internal reliance uses the narrow Human Principal exception path and cannot masquerade as ordinary approval. Unvalidated Critical work cannot be externally delivered.

## Risk Taxonomy

Risk categories are financial, legal/regulatory, privacy, cybersecurity, client confidentiality, reputational, operational, methodological/evidence, safety, execution/reversibility, third-party/tool, and business continuity.

Likelihood anchors:

- `RARE`: exceptional under credible conditions;
- `UNLIKELY`: plausible but not expected;
- `POSSIBLE`: credible occurrence during the decision horizon;
- `LIKELY`: expected in multiple credible scenarios;
- `ALMOST_CERTAIN`: expected absent new controls.

Impact anchors:

- `LOW`: below all Material triggers and readily reversible;
- `MODERATE`: meaningful but below Material thresholds, contained within engagement;
- `MATERIAL`: meets any `00_CORE` Material trigger;
- `CRITICAL`: meets any `00_CORE` Critical trigger.

Critical impact forces Critical treatment regardless of likelihood. Risk aggregation uses the highest applicable impact plus documented interaction effects; arithmetic averaging cannot lower a risk. Residual risk acceptance authority follows the Approval Policy, and Critical residual risk remains non-delegable.

## Synthesis Evidence Gate

Before synthesis integrity review, generate a coverage report listing every Material assertion, claim type, evidence relations, retrieval proofs, contradictions, confidence, and validation status. Unsupported, invalid, stale, mismatched, or materially contradicted claims fail closed. Persuasive wording cannot substitute for coverage.
