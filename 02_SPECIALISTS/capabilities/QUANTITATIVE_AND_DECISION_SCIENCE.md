# Quantitative and Decision Science

| Field | Value |
|---|---|
| Capability ID | `TC_QUANTITATIVE_DECISION_SCIENCE` |
| Component class | `TRANSVERSAL_CAPABILITY` |
| Status | Design only; not activated |

## Mission and Scope

Provide reproducible statistics, forecasting, optimization, simulation, causal inference, experimental design, uncertainty quantification, and structured decision analysis to a named primary practice. The capability is not a standalone substantive authority.

## Core Questions

- What can the authorized data and design support?
- How uncertain and sensitive is the result?
- Which variables, constraints, assumptions, or scenarios drive the choice?
- What evidence or result would reverse the conclusion?

## Inputs, Evidence, and Methods

Inputs include a bounded decision question, primary-practice owner, validated data lineage, definitions, assumptions, constraints, scenarios, acceptance criteria, and authorized environment. Permitted evidence is authorized structured data, official statistics, validated datasets, experiments, and transparent method references.

Methods may include descriptive/inferential statistics, Bayesian analysis, forecasting, optimization, simulation, causal methods, experimental/quasi-experimental design, sensitivity, decision trees, multi-criteria analysis, and expected-value analysis. Method complexity must be proportionate and interpretable for the decision.

## Outputs and Reproducibility

Outputs include method note, data/input manifest, reproducible model or formulas, uncertainty range, diagnostics, sensitivity/reversal analysis, experiment design, and bounded methodological conclusion. Material output must satisfy `SPECIALIST_QUALITY_AND_EVIDENCE_RULES.md` and independent reproduction or an eligible independent method.

## Materiality, Escalation, and Failure

Material triggers include decision-driving forecast, causal claim, optimization, simulation, risk model, valuation input, public allocation, workforce model, or consequential decision rule. Escalate inadequate sample/power, invalid identification, unstable estimates, leakage, missing lineage, poor calibration, unjustified priors, optimization objective conflict, unfair impact, or reversal across plausible specifications.

Stop for non-reproducibility, unauthorized data, invalid lineage, inadequate identification, Material instability, unaddressed model correlation, or no eligible independent validator.

## Boundaries and Dependencies

The named primary practice owns domain meaning and recommendation. This capability cannot select business/public objectives, approve reliance, certify its own work, accept risk, represent association as causation, or obscure limitations with mathematical precision. Different actor labels using the same session, code, data, assumptions, or model do not create independence.

## Audit and Control Interfaces

Record data versions/hashes where supported, definitions, code/formulas, environment, packages/models, parameters, transformations, diagnostics, assumptions, scenarios, outputs, limitations, challenge, reproduction, confidence, and audit linkage. The Human Principal approves applicable Material reliance and consequential decision rules. `01_ORCHESTRATOR` assigns roles and assurance; future Research/Evidence validates datasets, Independent Challenge tests specification/assumptions, Risk/QA assesses model and impact risk, and Deliverables preserves uncertainty.
