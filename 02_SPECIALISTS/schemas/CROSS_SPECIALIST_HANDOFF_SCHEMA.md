# Cross-Specialist Handoff Schema

## Canonical Relationship

This schema extends the certified Orchestrator handoff record. The certified fields and guards remain mandatory and controlling.

## Required Record

```text
handoff_id
handoff_version
engagement_id
parent_task_id
work_item_id
originating_specialist_id
originating_actor_id
originating_session_id
receiving_specialist_id
receiving_actor_id
receiving_session_id
question
scope_in
scope_out
classification
materiality
risk_domains
permission_ceiling
data_class
jurisdictions
evidence_package_refs
claim_refs
assumption_refs
unresolved_uncertainties
contradiction_refs
confidence
confidence_rationale
requested_output
acceptance_criteria
dependency_status
approval_status
approval_refs_where_applicable
deadline
priority
stop_conditions
issued_at
accepted_at
recipient_attestation
audit_correlation_id
```

## Validation Rules

- Parent/child tasks and every reference share one immutable engagement ID.
- Recipient identity, scope, tier, data, qualifications, jurisdiction, conflicts, and access are explicitly accepted.
- Unknown or mismatched values fail closed under the certified handoff rules.
- Evidence, assumptions, and inferences remain separately typed.
- Unresolved Material contradictions are linked and cannot be omitted or averaged.
- Approval status is referenced, never inferred or granted by the handoff.
- A handoff cannot broaden parent scope, classification floor, tier ceiling, permission, retention, or authority.
- Material change creates a new version and invalidates affected downstream reliance.
