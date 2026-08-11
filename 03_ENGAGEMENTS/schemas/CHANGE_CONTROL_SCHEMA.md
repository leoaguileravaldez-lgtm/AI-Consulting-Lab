# Change Control Schema

## Required Fields

- `change_id`, `engagement_id`, `requested_at`, `requestor`, `reason`.
- `change_type`: scope, objective, decision, assumption, target, materiality, jurisdiction, deliverable, routing need, date, exposure, risk tolerance, measurement baseline, evidence validity/provenance/source/version/freshness/interpretation/correction/revocation/expiry/supersession/invalidation, data/security/tool, or other governed source field.
- `old_object_ref`, `old_version`, `proposed_value_ref`, `proposed_version`.
- `materiality_before`, `materiality_after`, `classification_review_ref`.
- `impact_graph[]`: affected tasks, workstreams, evidence, assumptions, analyses, outputs, contradictions, synthesis, packets, approvals, deliverables, measurements, economics and closure checks.
- `invalidation_actions[]`, `unaffected_dependency_tests[]`, `required_rework[]`.
- `approval_category`, `human_principal_decision_ref`, `effective_at`, `status`.
- `canonical_transition_or_block_refs[]`, `audit_refs[]`, `superseded_by`.

## Rules

Proposed discretionary changes have no effect until the authoritative source owner and applicable canonical approval/transition accept them. An authoritative evidence correction, revocation, expiry, supersession or invalidation immediately suspends affected reliance and triggers dependency propagation; it does not wait for engagement-level acceptance. Material changes automatically mark every affected downstream projection `INVALIDATED` or `REVALIDATION_REQUIRED`; exact-object approvals tied to prior objects, versions or evidence bases cannot be reused.

Statuses are record labels only: `PROPOSED`, `IMPACT_ASSESSED`, `AUTHORIZED`, `REJECTED`, `WITHDRAWN`, `APPLIED_TO_SOURCE`, `SUPERSEDED`. They are not canonical workflow states. Unknown impact, unavailable Human Principal, or conflicting authority remains fail closed.
