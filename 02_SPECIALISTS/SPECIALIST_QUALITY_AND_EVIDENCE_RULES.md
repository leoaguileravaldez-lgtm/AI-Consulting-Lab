# Specialist Quality and Evidence Rules

## Controlling Standard

`00_CORE` Quality Standards and Source Validation and the `01_ORCHESTRATOR` claim/evidence architecture control. This document applies them to specialist outputs without redefining them.

## Analytical Claim Types

Specialist presentation may use the following audience labels, mapped to certified records:

| Label | Meaning | Certified treatment |
|---|---|---|
| `FACT` | Directly supported by accessed, traceable, validated evidence | `VERIFIED_FACT`; must satisfy source and validation rules |
| `ASSUMPTION` | Proposition temporarily accepted to enable analysis | Assumption record with rationale, owner if known, status, range, and validation need |
| `INFERENCE` | Reasoned conclusion derived from facts and assumptions | Claim records must link premises, method, alternatives, and confidence; never relabeled fact |
| `SCENARIO` | Coherent conditional future or counterfactual | Inputs, conditions, horizon, probability status if any, and consequences stated |
| `RECOMMENDATION` | Proposed action based on evidence and judgment | Recommendation record; not a decision or authorization |

Where `00_CORE` also requires `ESTIMATE` or `HYPOTHESIS`, specialists retain those canonical types. Presentation labels never erase canonical record types.

## Evidence Rules

- Every Material claim resolves through the certified assertion-to-claim-to-evidence graph.
- Source authority, directness, method, currency, relevance, independence, consistency, and integrity are assessed separately.
- Corroboration counts independent source lineage, not citation count.
- Client-provided data is labeled and tested for lineage, definitions, completeness, reconciliation, bias, and reliability.
- Search snippets, aggregators, anonymous claims, social media, and AI summaries cannot independently verify a Material claim.
- Source conflict remains visible and affects confidence.
- Decision-critical freshness requirements are rechecked before reliance or delivery.
- Research discovery and producer self-check do not equal formal independent validation.

## Assumption Control

Every Material assumption records statement, reason, source/proxy, owner if known, range, validation status, affected claims/models, sensitivity, required evidence, expiry/review date, and downstream consumers. An assumption cannot become fact because multiple specialists reuse it.

Decision-critical unverified assumptions cap confidence and block High confidence. If plausible values reverse the recommendation, the uncertainty is Material and must be resolved or escalated.

## Quantitative Reproducibility

Material quantitative output records:

- exact input references and versions;
- units, definitions, period, population, and as-of date;
- formulas, transformations, exclusions, and missing-data treatment;
- code/model/environment version where applicable, without credentials;
- parameters, scenarios, sensitivities, and diagnostics;
- rounding and reconciliation rules;
- output version/hash where supported;
- limitations, uncertainty, and confidence;
- independent reproduction or independent-method result.

A competent eligible reviewer must be able to reproduce the decision-driving result from authorized records. Non-reproducibility causes validation failure.

## Causality and Scenarios

Correlation, pre/post change, platform attribution, management belief, or expert opinion alone does not establish causation. Causal claims require a defensible identification strategy and stated limitations. Otherwise use association, hypothesis, or scenario language.

Material recommendations include base and downside cases, reversal conditions, mitigations, warning indicators, and upside where decision-useful. Scenario probabilities are not invented when evidence cannot support them.

## Measurement and Benefits Integrity

Material outcome measurement follows `capabilities/MEASUREMENT_KPI_AND_BENEFITS_REALIZATION.md`. Each KPI links to the decision objective and records one accountable domain owner, definition, method, data provenance, baseline, target, target date, leading/lagging status, financial/non-financial status, assumptions, confidence, validation, attribution limitations, unintended consequences, and audit lineage.

Approved Material baselines cannot be overwritten retrospectively. Corrections preserve the original version, rationale, decision/dependency impact, revalidation, and renewed approval where required. Unfavorable results, failed outcomes, missed targets, disbenefits, and approved unintended-consequence measures remain visible.

Benefit claims reconcile gross observation, incremental attribution, costs, disbenefits, timing, overlap, cannibalization, transfer effects, and net result. Unique benefit IDs and dependency reconciliation prevent double counting. Correlation, pre/post movement, or temporal coincidence is not causal attribution without a defensible counterfactual or identification method.

The metric designer, benefit owner, implementation owner, or quantitative producer cannot be the sole validator of a Material result. Corrective recommendations remain recommendations and cannot authorize action or progression.

## Output Quality Gate

Before handoff, the specialist confirms scope, audience, decision question, classification, claim labels, evidence links, assumptions, method, calculations, scenarios, alternatives, risks, confidence, limitations, dependencies, conflict status, audit lineage, and approval status. A self-check improves quality but does not satisfy required independent assurance.

## Prohibited Quality Practices

- fabricated or inaccessible citations;
- false precision;
- selective evidence chosen for a preferred thesis;
- silent averaging of conflicts;
- unsupported causal language;
- financial outputs built on inputs represented above their validation status;
- technical feasibility represented as business viability;
- legal issue analysis represented as qualified advice;
- omission of dissent, negative cases, or decision-reversing sensitivity;
- changing validated conclusions for presentation effect.
- invented KPIs, retrospective baseline manipulation, selective metric reporting, or metric substitution after results are known;
- unsupported benefit attribution, double counting, cherry-picked measurement windows, or suppression of failed outcomes;
- autonomous corrective action or progression based on a metric result.
