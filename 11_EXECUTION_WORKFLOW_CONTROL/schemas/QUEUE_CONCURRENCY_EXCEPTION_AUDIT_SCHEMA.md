# Queue, Concurrency, Exception, and Audit Schemas

## Queue Entry

Required fields include `queue_entry_id`, schema/record/work-item versions, segregated queue ID, client/entity/engagement/security boundaries, purpose/classification/confidentiality, priority and basis, enqueue/update times, aging/starvation status, eligibility/readiness refs, reprioritization history, status, limitations, creator, authority, audit, and invariant validation.

## Concurrency Lease

Required fields include `lease_id`, schema/record/work-item/attempt versions, owner, exact protected object/version, boundary/purpose/classification, lease status, acquired/expiry/renewal, conflict refs, compare-and-swap token reference, release/reconciliation status, limitations, creator, authority, audit, and invariant validation. A lease grants no substantive or external-action authority.

## Timeout Event

Required fields include `timeout_event_id`, schema/record/work-item/attempt versions, timeout policy/clock basis/start/deadline/pause rules, observed time, boundary, purpose/classification, status, permitted routed state, limitations, creator, authority, audit, and invariant validation. Timeout cannot imply approval or success.

## Exception Record

Required fields include `exception_id`, schema/record/work-item/attempt versions, exception type/materiality/status, boundary, purpose/classification/confidentiality, event/failure refs, containment, affected scope, canonical owner/escalation, required disposition, freshness/review, limitations, creator/time, audit, and invariant validation.

## Escalation Record

Required fields include `escalation_id`, schema/record/work-item versions, issue/materiality/status, exact affected refs, boundary, purpose/classification, recipient authority, requested disposition, opened/due/closed times, canonical resolution ref, limitations, creator, audit, and invariant validation. Escalation is not resolution.

## Cancellation and Rollback Record

Required fields include `control_action_id`, action type, schema/record/work-item/attempt versions, boundary, purpose/classification/confidentiality, requested status, exact scope, original action/outcome refs, preserved records inventory, reversibility evidence, canonical authority/Human refs, verification plan/result, freshness/review, limitations, creator/time, audit, and invariant validation.

## Workflow Audit Event

Required fields include `workflow_audit_event_id`, schema/record version, exact object/work-item/version, `work_item_dependency_set_status`, `work_item_declared_dependency_count`, `dependency_record_refs_and_versions`, `dependency_reconciliation_ref`, actor/role, time, predecessor/successor, reason, trigger, boundary, purpose/classification, authority/Human refs, outcome, correlation, provenance, limitations, audit-chain refs, and invariant validation. Audit events are append-only and contain no credentials. When an event does not evaluate dependencies, the dependency fields still preserve the exact Work Item declaration and explicitly record that reconciliation was not performed; omission is prohibited.
