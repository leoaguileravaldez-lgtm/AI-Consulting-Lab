# Measurement, KPI and Benefits Realization

| Field | Value |
|---|---|
| Parent capability | `TC_QUANTITATIVE_DECISION_SCIENCE` |
| Component class | `TRANSVERSAL_CAPABILITY` |
| Status | Design only; not activated |

## Purpose and Boundary

This subordinate capability standardizes outcome definition, baselines, KPIs, targets, milestones, measurement, attribution, variance, and benefits realization across primary practices. It is not a primary practice, registry authority, workflow role, lifecycle state, validator, benefit certifier, or autonomous monitoring function.

One named primary practice owns the domain meaning and intended outcome for each KPI. This capability supplies measurement method and record discipline. `01_ORCHESTRATOR` coordinates task lineage, dependencies, assurance, Human Review, and follow-up work without becoming the substantive metric designer or validator.

## Measurement Record

Every Material KPI or benefit claim records:

```text
metric_id
metric_version
engagement_id
task_id
decision_id
practice_owner
metric_owner
metric_name
metric_purpose
metric_type
definition
formula
unit
population
scope
data_source_refs
data_provenance
collection_method
baseline_value
baseline_period
baseline_validation_status
target_value
target_range_where_applicable
target_date
target_basis
leading_or_lagging
financial_or_non_financial
measurement_frequency
implementation_milestone_refs
observed_result_refs
realized_benefit_refs
assumption_refs
confidence
attribution_category
attribution_method
attribution_limitations
unintended_consequence_metrics
risk_and_gaming_indicators
approval_status
audit_correlation_id
```

The record uses exact versions and same-engagement references. `approval_status` refers to a trusted approval record and cannot be granted by a specialist or this capability.

## Measurement Lifecycle

### Outcome definition

Before Material implementation reliance, define the decision objective, intended outcome, affected population or operating unit, financial and non-financial dimensions, decision owner, accountable primary practice, and metric owner. Activity, output, adoption, and outcome measures remain distinguishable.

### Baseline

Record value, period, population, scope, units, source/provenance, method, exclusions, missing-data treatment, seasonality/external conditions, validation, confidence, and limitations. An approved Material baseline is immutable. Correction creates a new version, preserves the original, explains the defect, identifies affected decisions and dependencies, and requires revalidation and renewed approval where Material.

### KPI and target

A KPI must link to an authorized decision objective and have one accountable domain owner. Target value/range and date record their evidence, scenario, assumptions, dependencies, feasibility, sensitivity, and reversal conditions. A target cannot be chosen merely because it supports a preferred case.

### Implementation milestones

Each Material milestone identifies owner, required condition or artifact, due date, dependency, evidence of completion, leading indicator, stop/escalation threshold, and approval dependency. Milestone completion is not realized benefit.

### Observation and variance

Observed results preserve period, source versions, method version, missing/changed data, confidence, baseline/target comparison, external factors, and unintended consequences. Variance separates volume, price, mix, timing, cost, productivity, quality, risk, implementation, assumption, measurement, and external effects as applicable.

### Attribution

Attribution is classified as `DIRECT_VERIFIED_EFFECT`, `CREDIBLE_CAUSAL_ESTIMATE`, `CONTRIBUTION`, `ASSOCIATION`, or `NO_DEFENSIBLE_ATTRIBUTION`. A causal category requires a defensible counterfactual or identification strategy. Correlation, pre/post movement, management belief, or temporal coincidence alone cannot establish attributable benefit.

### Realized benefits

Realized-benefit analysis records gross observed benefit, incremental attributable benefit, implementation and ongoing cost, disbenefits, unintended consequences, overlap/double-counting adjustments, net realized benefit, variance from the approved case, confidence, attribution limitations, unresolved disagreements, and corrective-action recommendations.

Financial benefits reconcile with Finance definitions. Non-financial benefits use observable definitions and are not monetized without a defensible method.

### Post-implementation review

Post-implementation review uses a new authorized follow-up task or work item with explicit lineage to the original decision. It does not reopen or overwrite a terminal task. Existing Orchestrator analysis, challenge, validation, risk, synthesis, Human Review, retention, and closure controls apply.

## Independence and Assurance

For Material measures and benefits:

1. the primary practice defines domain outcome and meaning;
2. this capability defines or tests measurement and attribution method;
3. Independent Challenge tests incentives, gaming, counterfactuals, omitted harms, and alternative explanations;
4. Research/Evidence validates data identity, provenance, definitions, completeness, and freshness;
5. an eligible independent actor reproduces calculations or uses an independent method;
6. Risk/QA assesses selective reporting, double counting, control integrity, adverse outcomes, and residual risk;
7. Orchestrator synthesis preserves favorable and unfavorable results, dissent, confidence, and limitations;
8. the Human Principal makes applicable decisions and authorizations.

The metric designer, benefit owner, implementation owner, Finance producer, or quantitative model producer cannot be the sole validator of affected Material results. Different labels or sessions do not cure shared actor, data, method, code, assumptions, incentives, or hidden context.

## Anti-Gaming Controls

- Invented KPI: prohibit any Material metric without decision purpose, definition, method, data, owner, and audit linkage.
- Baseline manipulation: preserve approved baselines; corrections are versioned, explained, validated, and impact-assessed.
- Selective reporting: retain every approved Material KPI, unfavorable result, missed target, disbenefit, and unintended-consequence measure.
- Metric substitution: prohibit replacement after results are known without visible versioning, rationale, impact analysis, validation, and approval.
- Scope manipulation: reconcile denominator, population, period, definition, exclusion, and methodology changes to the approved baseline.
- Unsupported attribution: label association/contribution honestly and prohibit causal wording without adequate design.
- Double counting: assign unique benefit IDs and reconcile overlap, cannibalization, transfer effects, and shared initiative dependencies.
- Gross-benefit inflation: include implementation/ongoing costs, timing, leakage, disbenefits, and required operating capacity.
- Cherry-picked windows: define measurement windows before results; disclose and validate changes.
- Incentive conflict: disclose metric-owner and implementation-owner incentives connected to reported performance.
- Result alteration: prohibit deletion, overwrite, confidence inflation, or narrative suppression of unfavorable outcomes.
- Autonomous correction: recommendations remain proposals; no metric result authorizes action, expenditure, progression, deployment, communication, or commitment.

## Cross-Specialist Responsibilities

- Strategy owns strategic outcomes and reversal conditions.
- Finance owns financial definitions, reconciliation, capital effects, costs, and net-benefit logic.
- Market owns external market and demand baselines.
- Commercial owns customer, pricing, acquisition, retention, and revenue measures.
- Operations owns process, capacity, service, quality, safety, and resilience measures.
- Technology owns system, data, AI-evaluation, reliability, and security measures.
- Regulatory/Policy owns issue identification and constraints, not legal certification.
- Public Sector owns public-value, equity, fiscal, institutional, and citizen-outcome meaning.
- Organization owns workforce, capability, adoption, governance, and change outcomes.
- Quantitative and Decision Science owns common measurement, uncertainty, attribution, and reproducibility methods, not domain conclusions.

Cross-specialist dependencies are documented by exact version. They do not dilute the single accountable domain owner.

## Human Principal Control and Stop Conditions

Human Principal approval is required where Material for outcome/KPI reliance, baseline, target, Material metric changes, attribution limitations, corrective strategic direction, expenditure, staffing, contracting, deployment, external action, stage progression, risk acceptance, and closure.

Stop and escalate for missing ownership, invented or unvalidated metrics, retrospective baseline change, selective reporting, unverifiable provenance, decision-reversing uncertainty, unsupported causality, double counting, self-validation, suppressed failure, unauthorized data, ambiguous authority, or attempted autonomous corrective action. A failed control cannot be cured by a favorable result.
