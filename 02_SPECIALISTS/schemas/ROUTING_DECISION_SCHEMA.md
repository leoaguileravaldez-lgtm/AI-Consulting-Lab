# Routing Decision Schema

## Required Record

```text
routing_decision_id
routing_version
engagement_id
task_id
decision_question
engagement_type
sector_overlays
jurisdiction_overlays
classification
materiality_triggers
risk_domains
financial_exposure
regulatory_exposure
technology_dependence
operational_complexity
workforce_intensity
public_sector_involvement
evidence_uncertainty
strategic_uncertainty
deliverable_type
primary_practice_id
mandatory_practice_ids
optional_practice_ids
escalation_triggered_practice_ids
transversal_capability_ids
excluded_practice_ids_and_rationale
work_item_question_owners
dependency_graph_ref
assurance_capacity_refs
registry_and_qualification_versions
conflict_checks
permission_and_data_eligibility
duplication_check
selection_rationale
approval_dependencies
owner
created_at
audit_correlation_id
```

## Rules

- Exactly one primary practice owns each work-item question.
- Every activated practice has a bounded distinct question or required assurance-independent dependency.
- Optional activation has a recorded marginal-value rationale.
- Exclusion does not waive a mandatory trigger.
- Eligibility failures through jurisdiction suitability are disqualifying.
- Re-routing is versioned and includes downstream impact.
- Routing grants no role, permission, access, authority, or approval.
