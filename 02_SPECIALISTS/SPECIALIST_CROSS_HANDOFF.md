# Specialist Cross-Handoff

## Purpose and Authority

This document specializes, but does not replace, the canonical handoff contract in `01_ORCHESTRATOR/DELEGATION_HANDOFF_AND_ASSURANCE.md`. Any conflict is resolved in favor of the certified contract.

## Material Handoff Contract

Every Material cross-specialist handoff contains:

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
confidence_level
confidence_rationale
requested_output
acceptance_criteria
dependency_status
approval_status
approval_refs_where_applicable
deadline
priority
stop_conditions
audit_correlation_id
issued_at
accepted_at
recipient_attestation
```

References bind exact record versions. Secrets and unnecessary Sensitive content are excluded.

## Sender Duties

The sender must:

- state the bounded question rather than delegate an open-ended mandate;
- distinguish verified inputs from assumptions and unresolved claims;
- disclose evidence lineage, validation status, freshness, and contradictions;
- identify decision-critical inputs and downstream consumers;
- preserve the parent classification, tier ceiling, engagement, data class, retention, and stop conditions;
- state what the receiver may not conclude or do;
- link the handoff to the audit trail.

## Recipient Acceptance

The recipient explicitly attests that engagement, identity, scope, tier, access, qualification, jurisdiction, conflicts, evidence package, output, acceptance criteria, and stop conditions are understood and valid. Silence is not acceptance. A mismatch produces the certified delegation, permission, conflict, security, or validation block.

Acceptance does not validate supplied evidence or assumptions. The receiver records reliance status for every decision-critical input.

## Dependency States

- `NOT_STARTED`: predecessor work has not begun.
- `IN_PROGRESS`: predecessor output is not stable and cannot support reliance.
- `PROVISIONAL`: may support labeled scenarios only.
- `READY_FOR_REVIEW`: submitted but assurance is incomplete.
- `VALIDATED`: applicable evidence and analysis validation passed for the stated version and use.
- `BLOCKED`: a certified stop condition prevents reliance.
- `SUPERSEDED`: replaced by an explicit new version and lineage.

No local dependency label changes the canonical task state.

## Contradiction Handling

No contradiction may be silently merged, averaged, deleted, or resolved by narrative emphasis.

1. Open a contradiction record linking competing claims, evidence, assumptions, methods, periods, definitions, populations, units, and source lineages.
2. Classify the disagreement as factual, definitional, temporal, methodological, scenario-based, jurisdictional, or judgmental.
3. Assess materiality and affected dependencies.
4. Assign a neutral resolution owner who did not own both competing conclusions.
5. Obtain targeted evidence, recalculation, qualified review, or challenge as appropriate.
6. Recalculate confidence and invalidate affected downstream records.
7. Record only `RESOLVED`, `ACCEPTED_NON_MATERIAL_LIMITATION`, or `UNRESOLVED` under the certified contradiction model.
8. Preserve minority views and failed resolution attempts.

An unresolved Material contradiction that could change the recommendation, confidence, approval, or risk acceptance blocks synthesis/release and is escalated to the Human Principal. Human judgment may select among disclosed alternatives; it cannot turn an unsupported claim into a verified fact.

## Handoff Completion

A handoff completes only when the requested artifact, evidence/assumption lineage, confidence, limitations, contradictions, dependency impact, and audit reference meet the acceptance criteria. Completion never implies approval, validation, risk acceptance, or downstream authorization.
