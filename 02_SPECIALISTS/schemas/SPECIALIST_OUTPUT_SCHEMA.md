# Specialist Output Schema

## Required Record

Every Material specialist output contains:

```text
output_id
output_version
engagement_id
task_id
work_item_id
practice_id
actor_id
session_id
question
scope_in
scope_out
classification
materiality
as_of_date
audience
input_record_refs
fact_claim_refs
assumption_refs
inference_claim_refs
scenario_refs
recommendation_refs
evidence_relation_refs
method_and_model_refs
quantitative_reproduction_requirements
measurement_plan_ref
metric_refs
metric_owner_refs
baseline_refs
target_values_and_dates
leading_and_lagging_indicator_refs
financial_and_non_financial_benefit_refs
implementation_milestone_refs
observed_result_refs
realized_benefit_refs
variance_analysis_ref
attribution_category_and_method
attribution_limitations
unintended_consequence_refs
corrective_recommendation_refs
post_implementation_review_ref
alternatives
downside_and_reversal_conditions
dependencies
contradictions
risks
limitations
confidence
confidence_rationale
challenge_status
validation_status
risk_review_status
approval_status
stop_conditions
artifact_hash_where_supported
audit_correlation_id
created_at
```

## Rules

- Claim labels map to certified record types and never promote assumption or inference to fact.
- Material assertions link to validated evidence relations before reliance.
- Decision-critical inputs state validation and freshness status.
- Every Material KPI has one accountable domain owner and links to definition, method, provenance, baseline, target/date, assumptions, confidence, validation, attribution limits, and audit lineage.
- Approved Material baselines cannot be overwritten; corrections create a new version, preserve the original, and invalidate affected assurance or approval.
- Realized-benefit records distinguish gross observation, attributable increment, costs, disbenefits, overlap/double-counting adjustments, and net benefit.
- Correlation, contribution, association, and causal estimates remain explicitly distinguished.
- Unfavorable results, missed targets, unintended consequences, and failed outcomes cannot be omitted from required Material reporting.
- Corrective recommendations grant no approval, progression, expenditure, deployment, communication, commitment, or execution authority.
- `approval_status` is descriptive and referenced from the trusted approval record; the specialist cannot set approval to granted.
- Draft outputs are clearly unapproved and non-executing.
- Semantic changes create a new version and invalidate affected assurance/approval.
- Output completion does not mean validation, approval, delivery, or execution.
