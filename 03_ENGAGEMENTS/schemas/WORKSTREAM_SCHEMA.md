# Workstream Schema

## Required Fields

- `workstream_id`, `engagement_id`, `version`, `title`.
- `canonical_parent_task_id` and `canonical_child_task_ids[]` with source versions/statuses.
- `accountable_question_owner`, distinct `contributors[]`, and `assurance_role_refs[]`.
- `question`, `inputs[]`, `dependency_refs[]`, `required_evidence[]`, `assumptions[]`.
- `expected_output`, `materiality`, `due_date`, `priority`.
- `capability_need_request[]`, `routing_decision_ref`, `handoff_refs[]`.
- `blocker_refs[]`, `contradiction_refs[]`, `human_principal_dependency_refs[]`.
- `derived_status`, `freshness`, `change_record_refs[]`, `audit_refs[]`.

## Rules

One accountable owner applies per question/output. The workstream cannot select an actor, activate a specialist, grant access, create a canonical task, delegate, transition status, validate itself or authorize reliance. `capability_need_request` is declarative input to `01_ORCHESTRATOR` only.

Every dependency identifies whether it is informational, sequential, evidence, assumption, assurance, approval or delivery. Material dependency changes mechanically invalidate affected outputs. Unaccepted handoffs, recursive delegation, self-routing or engagement-boundary mismatch fail closed under applicable canonical controls.

Derived status must cite all canonical task states and the deterministic rule. Mixed states remain visible; pending or terminal tasks cannot be hidden by a roll-up.
