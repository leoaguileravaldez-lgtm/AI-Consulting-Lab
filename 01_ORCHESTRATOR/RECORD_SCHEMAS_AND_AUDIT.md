# Canonical Records, Schemas, and Audit Integrity

## Schema Conventions

This document is the canonical conceptual data model. `ID` means immutable unique identifier; `REF` means a required reference to an existing compatible record; `LIST<REF>` means zero or more unless minimum cardinality is stated; timestamps are ISO 8601 with timezone; enums use controlled values. Future implementation must define physical types without weakening these constraints.

Every record requires `id`, `schema_version`, `record_version`, `engagement_id` unless globally administrative, `created_at`, `created_by`, `status`, and audit linkage. Updates create a new version; they do not erase prior versions.

## Entity Schemas

| Entity | Required domain fields | Required relationships/cardinality |
|---|---|---|
| Engagement | immutable ID, client ref, purpose, scope, data classes, jurisdictions, tiers, systems/tools, limits, audience, retention, effective/closure dates | 1 authorization; 1 conflict record before active; 1 isolation/security object; 1..* tasks |
| Task | ID, question, scope in/out, class, risk, tier, state, requestor, owner, deadline | exactly 1 engagement; 0..1 parent; 0..* children; 1 current state version |
| Evidence | source identity, type, provenance, dates, location, methodology, reliability dimensions, freshness, retrieval proof, validation | exactly 1 engagement; 1..* claim relations when relied upon |
| Claim | statement, claim type, materiality, decision-critical flag, confidence, owner, status | exactly 1 task; 0..* evidence relations; Material claims require 1..* valid supporting relations |
| Recommendation | proposed action, benefit, cost, risks, alternatives, feasibility, complexity, confidence | exactly 1 task; 1..* supporting claims; 1 challenge, 1 validation, 1 risk review when Material |
| Challenge | counter-position, findings, reversal conditions, dissent, dispositions | exactly 1 recommendation/task; 1 challenger; 1..* findings for Material unless explicit no-finding attestation |
| Validation | scope, procedures, independence result, evidence result, reproduction result, limitations, disposition | exactly 1 subject artifact; 1 eligible validator; 1 independence record |
| Risk | category, cause, likelihood, impact, inherent/residual ratings, controls, mitigation, owner, acceptance authority | exactly 1 task/recommendation; 0..* related risks; Material work requires all plausible categories assessed |
| Approval | category, approver, authority, scope, decision, timestamp, exact artifact/action/version/hash, conditions, validity, nonce/replay state | exactly 1 engagement/task; 1 approver identity; 1..* approved objects; cannot reference mutable “latest” |
| Approval consumption | consumption ID, approval ID/version, engagement/task, governed transition/action, artifact/action ID/version/hash, scope, reservation/consumption status, idempotency key, lease/fence, transition/action journal and audit refs, timestamps | exactly 1 approval and governed intent; 0..1 successful consumption per one-time approval nonce; all references share one engagement and immutable correlation ID | `AVAILABLE` → `RESERVED` → `CONSUMED`; `RESERVED` may become `RELEASED` only after proof that no governed effect occurred; uncertainty becomes `RECONCILIATION_REQUIRED`; terminal `CONSUMED` is never reusable |
| Audit event | event/sequence IDs, actor, timestamps, action, prior/new state, input/output refs, result, integrity link | exactly 1 audit stream; 0..1 prior event; all referenced objects must exist/current at event time |
| Deliverable | title, audience, class, version/hash, assertions, confidence, dissent/limitations, release/delivery status | exactly 1 task; 1 decision record; Material requires 1 synthesis-integrity review and release check |
| Decision | question, owner, options, evidence, assumptions, recommendation, risks, confidence, unresolved issues, disposition | exactly 1 task; 1..* claims; 0..1 human approval until decided |
| Incident | type, severity, detection, scope, affected engagements/objects, containment, evidence, decisions, recovery, closure | 1..* audit events; 1 incident owner; Critical closure requires Human Principal authorization |

## Governance-Critical Control Records

These records are required wherever their control applies; they are not optional implementation conveniences.

| Record | Required fields | Relationships/cardinality | Controlled status lifecycle |
|---|---|---|---|
| Engagement security object | immutable engagement ID, client ref, purpose, namespaces, data classes, jurisdictions, identities/roles/tiers, tools/systems, limits, authorization version, effective/closure dates | exactly 1 per engagement version; provisional requires 1 intake authorization; active requires 1 conflict disposition and 1 engagement authorization | `PENDING_AUTHORIZATION` → `ACTIVE` → `SUSPENDED`/`CLOSING` → `CLOSED`; never mutate ID |
| Independence record | ID, subject validation ID, primary/validator identities, model/provider/version/session IDs, context-manifest ref, assumption/method/evidence-correlation results, conflict attestation, qualification refs, pass/limited/fail rationale | exactly 1 per validation attempt; 1 primary set; exactly 1 validator; 1..* qualification refs as applicable | `DRAFT` → `CHECKED` → `PASS`/`LIMITED`/`FAIL`/`EXPIRED`; any Material input change expires it |
| Validation packet/context manifest | ID/hash, task/engagement, allowed claim/source/input refs, excluded content classes, creator, recipient, access sequence, sealed timestamp | exactly 1 per validation attempt; only same-engagement refs; exactly 1 assigned validator | `PREPARED` → `SEALED` → `OPENED` → `SUPERSEDED`; changes create new version |
| Qualification record | actor ID, subject/method/jurisdiction scope, evidence, approving human, prohibited roles, effective/expiry, registry version | 1 actor; 1..* scoped qualifications; referenced by every eligible assurance assignment | `PROPOSED` → `APPROVED` → `EXPIRED`/`REVOKED` |
| Retrieval proof | source ID, retriever/session, resolved location, access time/outcome, identity checks, content fingerprint/snapshot ref, exact support coordinates, supersession check | exactly 1 source and 1 retrieval attempt; decision-critical evidence requires 1..* valid independent proofs | `ATTEMPTED` → `VERIFIED`/`FAILED`/`STALE`; freshness change can make stale |
| Claim-evidence relation | relation ID, claim/source IDs and versions, relation type, supported portion, limitations, strength, validator, timestamp | exactly 1 claim and 1 evidence item in same engagement; Material claim requires 1..* valid `SUPPORTS` relations | `PROPOSED` → `VALIDATED`/`INVALID`/`SUPERSEDED` |
| Contradiction record | competing claim/source refs, differences, materiality, owner, attempted resolution, impact, confidence adjustment, disposition | 1..* claims and 2..* competing evidence/claim refs | `OPEN` → `RESOLVED`/`ACCEPTED_NON_MATERIAL_LIMITATION`; `UNRESOLVED` remains nonterminal/blocking where Material |
| Synthesis coverage manifest | deliverable/version/hash, assertion-to-claim map, challenge/validation/risk/decision refs, omissions/dissent checks, reviewer, result | exactly 1 per Material deliverable version; every Material assertion maps to 1..* claims | `DRAFT` → `REVIEWED` → `PASS`/`FAIL`; artifact change expires it |
| Transition journal | transition ID, task, source/target/state versions, lease/fence, idempotency key, actor, authorization refs, prepared inputs/hash, phase, timestamps, failure | exactly 1 per transition attempt; 0..1 committed audit event; exactly 1 current task owner | `PREPARED` → `COMMITTING` → `COMMITTED`/`ABORTED`/`OUTCOME_UNKNOWN` |
| Action journal | action ID, task/engagement, operation, exact payload/hash, target, approval/nonce, idempotency key, fence, executor, provider receipt/outcome, timestamps | exactly 1 per material/external action attempt; 1 approval consumption; 0..* response evidence refs | `PREPARED` → `STARTED` → `SUCCEEDED`/`FAILED`/`OUTCOME_UNKNOWN` → `RECONCILED` |
| Checkpoint/recovery record | checkpoint ID, task state/version, record/hash set, audit sequence, dependencies, validator, corruption boundary, selected recovery point, invalidations, recovery authority | 1 checkpoint per successful transition; 1 recovery record per recovery attempt; 1..* affected objects | checkpoint `CREATED` → `VERIFIED`/`INVALID`; recovery `INITIATED` → `RECONCILED` → `AUTHORIZED` → `COMMITTED`/`FAILED` |
| Circuit-breaker record | breaker ID, scope type/ref, trigger, owner, opened time, denied capabilities, containment, health evidence, reset criteria/authority | exactly 1 scope (`TASK`, `ENGAGEMENT`, `TOOL`, `ORCHESTRATOR`); 1 owner; 0..* incidents | `CLOSED` → `OPEN` → `VERIFYING` → `CLOSED`; no direct open-to-closed reset |
| Block/hold record | record ID, affected object/work-item, prior state/version, type, reason/control, creator/authority/time, containment, remediation, recovery targets/authority, approval relationship, resolution/result, immutable audit links | exactly 1 engagement and 1 affected object; exactly 1 record per blocked/hold occurrence; detailed cardinality and lifecycle are canonical below | `OPEN` → `REMEDIATION_IN_PROGRESS` → `READY_FOR_RESOLUTION` → `RESOLVED`; an unresolved status may enter `ESCALATED`, then returns only to `REMEDIATION_IN_PROGRESS` or `READY_FOR_RESOLUTION` after escalation disposition; replacement uses `SUPERSEDED` |
| Escalation record | trigger, affected state/objects, detector/time, facts/unknowns, containment, options, safest recommendation, required authority, response, resumption conditions | exactly 1 task/engagement; 1 required approval authority; 0..* related incidents/exceptions | `OPEN` → `ACKNOWLEDGED` → `RESOLVED`/`SUPERSEDED`; silence never resolves |
| Exception record | policy/control, scope, rationale, risk, alternatives, compensating controls, monitoring, owner, effective/expiry, approval, closure | exactly 1 Human Principal approval; 1..* affected objects; cannot reference prohibited non-waivable exception | `REQUESTED` → `APPROVED`/`REJECTED` → `ACTIVE` → `EXPIRED`/`CLOSED`/`REVOKED` |
| Control-role assignment | actor, canonical role, engagement/scope, authority basis, incompatible roles, effective/expiry, appointing owner | exactly 1 actor and role per assignment; 0..* engagements; conflicts checked against all active assignments | `PROPOSED` → `ACTIVE` → `EXPIRED`/`REVOKED` |
| Capacity queue/reservation | queue entry ID, task/stage, priority, class, deadline, freshness deadline, qualification/role needs, effort estimate, WIP pool, reservation owner, age, blocker | exactly 1 task/stage; 0..* required reservations; all mandatory scarce roles reserved before plan approval | `QUEUED` → `RESERVED` → `ACTIVE` → `RELEASED`; alternatives `BLOCKED`, `EXPIRED`, `REFRESH_REQUIRED`, `CANCELLED` |

## Controlled Vocabularies

- Classification: `ROUTINE`, `MATERIAL`, `CRITICAL`.
- Claim type: `VERIFIED_FACT`, `ASSUMPTION`, `ESTIMATE`, `HYPOTHESIS`, `RECOMMENDATION`.
- Confidence: `HIGH`, `MEDIUM`, `LOW`.
- Evidence relation: `SUPPORTS`, `CONTRADICTS`, `CONTEXT_ONLY`, `INVALID`.
- Validation: `PASS`, `PASS_WITH_LIMITATIONS`, `FAIL`, `BLOCKED`.
- Independence: `PASS`, `METHOD_INDEPENDENT_LIMITED`, `LIMITED`, `FAIL`, `EXPIRED`.
- Approval decision: `APPROVED`, `REJECTED`, `REVISION_REQUIRED`, `MORE_EVIDENCE_REQUIRED`, `CANCELLED`, `REVOKED`, `EXPIRED`. Approval use/consumption is represented only by the approval-consumption record below, not by overwriting the approval decision.
- Approval-consumption status: `AVAILABLE`, `RESERVED`, `CONSUMED`, `RELEASED`, `RECONCILIATION_REQUIRED`.
- Incident severity: `LOW`, `MODERATE`, `MATERIAL`, `CRITICAL` using `00_CORE` impact triggers.
- States: only those in `STATE_MACHINE.md`.
- Transition phase: `PREPARED`, `COMMITTING`, `COMMITTED`, `ABORTED`, `OUTCOME_UNKNOWN`.
- Action outcome: `PREPARED`, `STARTED`, `SUCCEEDED`, `FAILED`, `OUTCOME_UNKNOWN`, `RECONCILED`.
- Breaker scope: `TASK`, `ENGAGEMENT`, `TOOL`, `ORCHESTRATOR`.
- Block/hold type: `EVIDENCE`, `CONFIDENCE`, `SOURCE_CONFLICT`, `VALIDATION`, `AUTHORIZATION`, `PERMISSION`, `CONFLICT`, `POLICY_CONFLICT`, `DELEGATION`, `CAPACITY`, `VALIDATION_FAILURE`, `SYNTHESIS_FAILURE`, `SECURITY`, `INCIDENT`, `EXPIRED_APPROVAL`, `RECOVERY`.
- Block/hold lifecycle: `OPEN`, `REMEDIATION_IN_PROGRESS`, `READY_FOR_RESOLUTION`, `ESCALATED`, `RESOLVED`, `SUPERSEDED`.
- Canonical control roles: `INTAKE_AUTHORITY`, `INTAKE_CONTROLLER`, `CONFLICT_REVIEWER`, `CLASSIFIER`, `CLASSIFICATION_CONFIRMER`, `PLANNER`, `PLAN_REVIEWER`, `PRIMARY_WORK_OWNER`, `WORKFLOW_CONTROLLER`, `HEALTH_CONTROLLER`, `CAPACITY_CONTROLLER`, `PERMISSION_AUTHORITY`, `REGISTRY_OWNER`, `CHALLENGER`, `VALIDATOR`, `RISK_REVIEWER`, `SYNTHESIZER`, `SYNTHESIS_REVIEWER`, `RELEASE_CONTROLLER`, `APPROVAL_WRITER`, `HUMAN_APPROVER`, `AUDIT_APPENDER`, `AUDIT_VERIFIER`, `SECURITY_REVIEWER`, `INCIDENT_AUTHORITY`, `RECOVERY_INITIATOR`, `RECOVERY_COORDINATOR`, `RECOVERY_VERIFIER`, `RECOVERY_AUTHORIZER`, `RECORDS_OWNER`, `EXECUTOR`, `DELIVERY_VERIFIER`.

## Canonical Block/Hold Record

This section is the sole schema owner for the record required whenever the canonical task state enters a blocked, failed, hold, expired-approval, or recovery-required state. Its lifecycle describes the control record only; it does not create or compete with task states or transitions in `STATE_MACHINE.md`.

### Fields and cardinality

| Field | Cardinality | Requirement |
|---|---:|---|
| `block_hold_id` | exactly 1 | Immutable globally unique record identifier. |
| `schema_version`, `record_version` | exactly 1 each | Controlled schema version and monotonically increasing record version. |
| `engagement_id` | exactly 1 | Immutable reference to the affected object's engagement security object. |
| `affected_object_id`, `affected_work_item_id` | exactly 1 object; exactly 1 work item | The blocked object and owning task/work item; both must exist in the same engagement. They may be the same ID only where the task itself is the affected object. |
| `prior_canonical_state`, `prior_state_version` | exactly 1 each | State and committed version immediately preceding this block/hold occurrence. |
| `block_hold_type` | exactly 1 | Controlled value mapped below to the canonical blocked/hold state. |
| `triggering_reason`, `triggering_control` | exactly 1 each | Specific observed reason and canonical control ID/document section that required blocking. Free-text rationale cannot replace the control reference. |
| `created_by_actor`, `created_by_authority`, `created_at` | exactly 1 each | Attributable actor ID, active canonical role-assignment reference, and timezone-bearing timestamp. |
| `lifecycle_status` | exactly 1 | Controlled block/hold record lifecycle value; current value is derived from the latest valid version. |
| `permitted_recovery_targets` | 1..* | Explicit subset of targets allowed by the applicable B/E/F/R transition in `STATE_MACHINE.md`; no target may be inferred at resolution time. |
| `prohibited_recovery_targets` | 0..* | Explicitly excluded targets where the state-machine constraint needs additional narrowing. All targets not permitted are prohibited even if not enumerated here. |
| `breaker_id`, `containment_status` | 0..1 breaker; exactly 1 status | Breaker reference is mandatory when the triggering control or state-machine failure rule requires one. Containment status records active denied capabilities and cannot be represented only by prose. |
| `remediation_requirements` | 1..* | Verifiable conditions that must be met before `READY_FOR_RESOLUTION`. |
| `required_approval_class` | exactly 1 | Controlled approval category or explicit `NONE`; derived from the applicable transition and cannot be downgraded within this record. |
| `approval_id`, `approval_consumption_id` | 0..1 each before resolution; exactly 1 each when required | Exact approval record and one-time consumption/reservation relationship. Both are required before a transition requiring approval can resolve the block/hold. `NONE` permits no approval reference. |
| `recovery_authority` | exactly 1 | Active canonical recovery/transition role-assignment reference authorized for the selected recovery transition; ordinary business approval alone is insufficient. |
| `resolution_evidence` | 0..* while open; 1..* for `READY_FOR_RESOLUTION` or `RESOLVED` | Versioned evidence references proving every remediation, containment, validation, and reconciliation requirement. |
| `resolution_actor`, `resolution_timestamp` | 0..1 each while unresolved; exactly 1 each when `RESOLVED` | Attributable eligible actor and timezone-bearing resolution time. |
| `resulting_state`, `resulting_state_version` | 0..1 each while unresolved; exactly 1 each when `RESOLVED` | The committed target and new canonical state version produced by the permitted transition. |
| `transition_journal_id`, `transition_correlation_id` | 0..1 each while unresolved; exactly 1 each when `RESOLVED` | Exact transition attempt and shared atomic-commit correlation identifier. |
| `audit_event_ids` | 1..* | Immutable links covering creation and every lifecycle/version change; resolution requires the committed transition audit event. |

Exactly one block/hold record exists for each entry into a blocked/failed/hold state occurrence. A task may have multiple historical records but no more than one unresolved record for the same state occurrence and affected object. Re-entry after resolution creates a new `block_hold_id` linked to the prior record; it never reopens or overwrites the resolved record.

### Type-to-state mapping

`EVIDENCE` maps to `BLOCKED_EVIDENCE`; `CONFIDENCE` to `BLOCKED_CONFIDENCE`; `SOURCE_CONFLICT` to `BLOCKED_SOURCE_CONFLICT`; `VALIDATION` to `BLOCKED_VALIDATION`; `AUTHORIZATION` to `BLOCKED_AUTHORIZATION`; `PERMISSION` to `BLOCKED_PERMISSION`; `CONFLICT` to `BLOCKED_CONFLICT`; `POLICY_CONFLICT` to `BLOCKED_POLICY_CONFLICT`; `DELEGATION` to `BLOCKED_DELEGATION`; `CAPACITY` to `BLOCKED_CAPACITY`; `VALIDATION_FAILURE` to `VALIDATION_FAILED`; `SYNTHESIS_FAILURE` to `SYNTHESIS_FAILED`; `SECURITY` to `SECURITY_HOLD`; `INCIDENT` to `INCIDENT_HOLD`; `EXPIRED_APPROVAL` to `EXPIRED_APPROVAL`; and `RECOVERY` to `RECOVERY_REQUIRED`. No custom or free-text type is valid.

### Lifecycle and immutability

- `OPEN` is created atomically with entry into the corresponding blocked/hold state. `REMEDIATION_IN_PROGRESS` records authorized remediation without changing canonical task state. `READY_FOR_RESOLUTION` requires all remediation and resolution evidence, recovery authority, containment verification, and any required exact approval to be present and current. `RESOLVED` is set only in the atomic commit of a permitted B/E/F/R transition. F17/F18 atomically resolve the prior hold record and open a distinct linked `RECOVERY` record; they never reuse or overwrite the hold record. `ESCALATED` adds containment and an escalation record; it does not authorize recovery and may return only to `REMEDIATION_IN_PROGRESS` or `READY_FOR_RESOLUTION` after an audited escalation disposition. `SUPERSEDED` applies only when a duplicate record is reconciled to the surviving canonical record and requires an audit correction; it cannot erase the underlying block.
- `block_hold_id`, engagement/object/work-item identity, prior state/version, type, triggering reason/control, creator/authority/time, and existing audit links are immutable. Corrections append a new version and `CORRECTION` audit event; prior values remain visible. Resolution fields are append-once and cannot be cleared or retargeted after commit.
- `permitted_recovery_targets` may only narrow after creation. Expansion requires a new independently reviewed recovery record and cannot exceed `STATE_MACHINE.md`. A changed approval, recovery target, affected object, or remediation basis invalidates `READY_FOR_RESOLUTION` and returns the record to `REMEDIATION_IN_PROGRESS` through an audited new version.

### Referential integrity and fail-closed rules

- Engagement, affected object/work item, actor assignments, evidence, approval, recovery, breaker, transition journal, and audit references must exist, be current for the attempted transition, and share the immutable `engagement_id`. Cross-engagement linkage is rejected.
- `prior_canonical_state`/version must equal the last committed state/version before the block. `resulting_state` must equal one enumerated `permitted_recovery_target`, and that target must satisfy the exact B/E/F/R row and target limitations in `STATE_MACHINE.md`.
- Approval category, scope, artifact/action version/hash, conditions, validity, authority, and consumption status must match the selected transition. Missing, stale, reused, wrong-category, or merely business approval cannot satisfy technical/recovery authorization.
- Recovery authority must be a current eligible canonical role assignment distinct from ordinary approval authority and satisfy separation-of-duties rules. Where independent verification or Human Principal approval is required, their records are additional mandatory resolution evidence.
- Missing or inconsistent required data, unresolved containment, absent audit linkage, incomplete remediation, invalid target, failed referential integrity, or unavailable required approval/verification leaves the record unresolved and the task in its blocked/hold state. The Orchestrator must not manufacture defaults or route around the block.

### Canonical-control mapping

- `STATE_MACHINE.md` owns permitted state transitions, target limitations, actor/tier/delegation rules, evidence, approvals, and failure behavior; this record instantiates those requirements and cannot add a transition.
- `DELIVERABLE_LIFECYCLE_AND_RECOVERY.md` owns recovery initiation, verification, authorization, checkpoint, reconciliation, rollback, and safe-state requirements; the block/hold record references the applicable recovery record and evidence.
- `AUTHORITY_PERMISSIONS_AND_DUTIES.md` owns approval classes, approver eligibility, non-delegability, exact-object binding, validity, and consumption; this schema stores required references but cannot reinterpret approval.
- The tamper-evident audit model in this document owns append, correction, sequence, retention, and independent-verification semantics; every block/hold lifecycle change is an attributable event in the engagement audit stream.

## Referential Integrity

- Cross-engagement references are rejected except a separately authorized, client-approved administrative linkage containing no client content.
- Referenced record versions and hashes are explicit; “latest” is prohibited for evidence, validation, approval, decision, and delivery.
- Deleting a referenced Material record is prohibited; disposition changes status and retention behavior.
- A changed claim/evidence/method invalidates dependent validation, synthesis, decision, and approval through a dependency-impact event.
- A task cannot reach Human Review while required relationship cardinalities are unsatisfied.
- A governance-critical control record cannot be replaced by an unstructured narrative or an unversioned foreign identifier.
- State, audit, transition journal, approval-consumption, action-journal, and checkpoint records share the same transition correlation ID where produced by one transition.

## Tamper-Evident Audit Architecture

Each engagement has an independent ordered audit stream. The trusted audit appender—not specialists—assigns event ID, monotonically increasing stream sequence, recorded timestamp, and prior-event integrity link. Events include actor identity, actor/session version, occurred/recorded times, task/state/action, object/input/output refs, authorization/approval/exception refs, result, and reason.

Future implementation may use hash chaining or an equivalent mechanism. This design does not claim cryptographic security. Whichever mechanism is chosen must demonstrably detect alteration, deletion, reordering, duplication, and insertion within the protected stream and must be independently verifiable.

### Append semantics

- A state/action is not successful until its audit append commits.
- The appender rejects an unexpected prior sequence/version.
- Event payloads are immutable after append.
- Corrections append `CORRECTION` events referencing the original and explaining actor, reason, old/new values, and approval where required.
- Sensitive values and credentials are excluded; references are used instead.

### Privileged access and verification

Audit append, read, verification, retention administration, and export are separate privileges. Specialists and synthesizers cannot alter streams. An independent verifier periodically and before Material release checks sequence continuity, integrity links, referenced-record existence, actor authority, and expected event completeness. Failed verification produces `INCIDENT_HOLD`.

### Retention

Audit retention follows the longer of engagement requirement or 12 months unless law/contract requires otherwise. Legal holds override disposal. Archive and restore must preserve sequence and integrity-verification capability.

## Canonical Approval-Governed Transaction Contract

This section is the sole design contract for approval verification, reservation, consumption, governed state mutation, and governed action dispatch. `STATE_MACHINE.md` identifies which transitions require approvals; `AUTHORITY_PERMISSIONS_AND_DUTIES.md` owns approval categories and approver eligibility. Secondary descriptions may not change this sequence.

### Invariants

1. An approval and one-time nonce bind immutably to one engagement, task, approval category, authority, scope, governed transition or action, artifact/action ID and version/hash, recipient/system/destination where applicable, named executor, validity window, conditions, and idempotency key.
2. One-time approval has at most one successful `CONSUMED` record. A uniqueness guard over approval ID/version plus nonce rejects concurrent or differently keyed consumption.
3. `RESERVED` excludes every other transition/action and worker. Reservation is not permission to create an external effect.
4. A governed internal state mutation requiring approval is successful only when state/version mutation, approval consumption, transition-journal commit, and audit append share one immutable transaction correlation ID and become authoritative together.
5. A governed external/material effect may occur only after its approval consumption and action-journal `STARTED` intent are durably committed and fenced. The external effect itself is never represented as atomically reversible.
6. `CONSUMED` never returns to `AVAILABLE`, even if execution fails or no effect occurs. A new attempt requires a fresh approval unless the same idempotent attempt is proven already committed and merely returns its recorded result.
7. No state, action, approval consumption, journal, or audit record from another engagement or a changed object/version/scope may join the transaction.
8. Any unknown, partial, stale, conflicting, or unreconciled condition fails closed; it never releases or recreates approval authority.

### Canonical sequence

For every approval-governed transition or action:

1. **Resolve intent:** Create an immutable transition/action intent with engagement, task, source state/version, intended target or operation, exact artifact/action version/hash, scope, actor/executor, approval ID/version/nonce, idempotency key, lease, fencing token, and transaction correlation ID.
2. **Verify approval:** Authenticate approver identity and authority; verify category, engagement, task, exact scope/object/version/hash, recipient/system/destination, executor, tier, conditions, valid-from/expiration, revocation, Material-change status, and unused nonce. Business, technical, deployment, recovery, and exception categories remain distinct.
3. **Verify state and authority:** Compare canonical source state/version, current lease/fence, actor role assignment, permission tier, engagement isolation, breaker state, required evidence, validation, risk, security, and all transition guards.
4. **Reserve:** Create or compare-and-set the approval-consumption record to `RESERVED`, bound to the exact intent and fence. Concurrent reservation or any different binding fails. A reservation cannot survive beyond its recorded validity conditions.
5. **Commit internal transition:** For an internal transition, atomically append the required audit event, mutate state/version, set the transition journal `COMMITTED`, and set the approval-consumption record `CONSUMED`. The commit succeeds in full or none becomes authoritative.
6. **Prepare external dispatch:** For N23, atomically commit the state transition to `DELIVER`, its audit event and transition journal, the exact action journal in `PREPARED`, and the approval reservation. No external effect occurs at N23.
7. **Commit dispatch authority:** Immediately before dispatch, revalidate approval freshness, exact binding, state/version, lease/fence, breaker and destination. Atomically set the approval consumption to `CONSUMED`, set the action journal to `STARTED`, and append `ACTION_DISPATCH_COMMITTED`. Only after this durable commit may the named fenced executor issue the external effect.
8. **Record outcome:** Append provider/recipient receipt or `OUTCOME_UNKNOWN`, without changing or releasing consumed approval. N24 then advances from `DELIVER` to `DELIVERY_VERIFICATION` using the consumed approval/action-attempt reference and the ordinary atomic state-transition protocol; N24 does not consume approval again.

### Success, failure, and prohibited states

- **Success—internal:** exact approval consumption, new state/version, committed transition journal, and audit event exist under one correlation ID; none exists alone.
- **Success—external/material:** N23 has a committed reservation and prepared action; the pre-dispatch commit has one consumed approval, one `STARTED` action attempt, and `ACTION_DISPATCH_COMMITTED`; N24 references that same attempt and records its outcome.
- **Failure before consumption:** canonical state remains unchanged for an internal transition; external dispatch is prohibited. A reservation may become `RELEASED` only after deterministic proof that no state mutation or external effect occurred and after an audited reconciliation.
- **Failure after consumption:** approval remains `CONSUMED`. Internal state is determined by the authoritative commit record. For external work, no automatic redispatch occurs; outcome is recorded or becomes `OUTCOME_UNKNOWN`.
- **Prohibited:** approval marked both available/released and consumed; action effect with approval `AVAILABLE`, `RESERVED`, or absent; state mutation without its required consumption record; consumption without exact intent/journal/audit linkage; two successful consumptions; consumption under a stale fence; changed artifact/scope under the same consumption; rollback that makes consumed approval reusable.

### Crash, retry, and reconciliation contract

- A crash before reservation leaves approval `AVAILABLE` and no governed mutation/effect.
- A crash after reservation but before internal commit leaves the original state authoritative. Recovery either completes the same idempotent transaction after full guard revalidation or proves no effect and appends `RELEASED`; ambiguity sets `RECONCILIATION_REQUIRED` and blocks use.
- A crash during internal commit is resolved solely by the authoritative commit record. If committed, state, audit, journal, and consumption are all present; otherwise none is authoritative.
- A crash after external consumption but before dispatch leaves approval consumed and the action attempt unresolved; redispatch is prohibited unless deterministic evidence proves the same idempotent provider operation was not issued and fresh approval authorizes a new attempt.
- A crash after dispatch produces a receipt-backed outcome or `OUTCOME_UNKNOWN`; approval remains consumed and independent reconciliation is mandatory.
- Same idempotency key and identical binding returns the already committed result. Same key with different binding, a new key using the same nonce, a stale fence, or a duplicate worker is rejected.
- If true atomic storage is unavailable, the implementation must use an independently reviewed transactional/outbox or saga-equivalent protocol with durable intent, exclusive reservation, monotonic fence, authoritative commit marker, idempotent append/publish, compensating record changes that never recreate consumed authority, and independent reconciliation. No component may expose new state or dispatch an action before the authoritative commit marker covers all required internal records.
- Reconciliation records observed state, approval/consumption status, journal phases, audit sequence, action/provider evidence, lease/fence, decision, verifier, authority, and resulting disposition. Any missing or contradictory evidence remains `RECONCILIATION_REQUIRED`, opens the applicable breaker, and routes through canonical recovery; silence or elapsed time never resolves it.

### Required audit evidence

Audit must make it possible to prove the approval and governed result are one lineage: approval ID/version/nonce, consumption ID/status, approver and authority basis, engagement/task, exact object/version/hash/scope, actor/executor, source/target state versions or operation/destination, idempotency key, lease/fence, transition/action journal IDs, transaction correlation ID, verification results, reservation/consumption timestamps, audit sequence/integrity link, outcome, failures, reconciliation, and final disposition. Sensitive values and credentials remain excluded.

## Atomic Commit and Crash-Consistency Model

### Internal transition protocol

1. **Acquire:** The workflow controller acquires a task lease and receives a monotonically increasing fencing token. It reads the expected task state/record version.
2. **Prepare:** It writes a `PREPARED` transition journal containing exact input versions/hashes, intended source/target, actor, approvals, idempotency key, lease, and fence. No canonical state changes.
3. **Validate:** It evaluates every transition guard and follows the canonical approval-governed transaction contract where approval is required.
4. **Commit:** In one atomic transaction where supported, compare expected state/version/fence; append the audit event; write the new task state/version; commit any required approval consumption; and mark the transition journal `COMMITTED`.
5. **Publish:** Downstream visibility occurs only after commit. Derived notifications use a durable outbox tied to the committed transition.

If a future storage design cannot atomically commit state, audit, approval consumption, and journal status, it must provide an equivalent formally reviewed protocol in which no new state becomes authoritative until an independently verifiable commit record covers all required parts. Dual writes without reconciliation are prohibited.

### External/material action protocol

1. N23 atomically prepares the action journal, exclusively reserves the exact one-time approval, binds payload hash, target, idempotency key, and fencing token, and appends `ACTION_PREPARED`; it does not dispatch.
2. Immediately before dispatch, the current fenced executor follows the canonical pre-dispatch commit: revalidate, consume approval, set the action journal `STARTED`, and append `ACTION_DISPATCH_COMMITTED`. Only then may it dispatch.
3. Record provider/recipient receipt and outcome. The provider-side idempotency identifier, where supported, must equal or bind to the action ID.
4. A crash after dispatch but before confirmed recording yields `OUTCOME_UNKNOWN`; no worker may redispatch automatically.
5. Independent reconciliation determines outcome and requires fresh authorization for any new action.

## Lease, Fencing, and Takeover

- Exactly one unexpired lease may exist per task transition owner.
- Every lease grant increments the fencing token; downstream state, journal, and action writers reject lower tokens even if an old lease holder continues running.
- Lease expiry does not itself grant takeover. A new owner must obtain the next fencing token and reconcile the prior transition/action journal.
- Takeover is prohibited while the prior journal is `COMMITTING`, `STARTED`, or `OUTCOME_UNKNOWN` until recovery authority resolves it.
- Heartbeat failure opens a task circuit breaker; it does not authorize duplicate work.
- Stale owners may finish local computation but cannot commit, consume approval, dispatch action, or publish output.

## Crash Matrix

| Crash point | Authoritative state | Required recovery |
|---|---|---|
| Before `PREPARED` journal | Original task state | New owner may retry with new attempt after lease reconciliation |
| After `PREPARED`, before commit | Original task state | Abort/reuse same idempotent attempt after guard revalidation |
| During internal commit | Determined by atomic commit record | Independent verifier resolves committed versus aborted; never infer from partial records |
| After commit, before notification | New task state | Durable outbox republishes notification without repeating transition |
| After action prepared/reserved, before pre-dispatch consumption | No external effect | Reconcile reservation and journal; dispatch remains prohibited until the canonical pre-dispatch consumption commit succeeds under the current fence |
| After dispatch, before receipt recorded | External outcome unknown | Set/retain `OUTCOME_UNKNOWN`; no retry; independent provider/recipient reconciliation |
| After confirmed external outcome, before state advance | External outcome known from verified receipt | Record/reconcile outcome, then advance through recovery transition; never redispatch |

## Idempotency, Duplicate Suppression, and Replay

- The trusted controller binds each idempotency key to engagement, task, transition/action, exact input/payload hash, authorization, and attempt lineage.
- Same key/same binding returns the committed result; same key/different binding is rejected.
- A one-time approval nonce cannot be consumed by another key or action.
- Duplicate analytical tasks are detected within one engagement using normalized question, scope, output, parent lineage, and dependency graph; probable duplicates require review and are not silently merged.
- Duplicate detection never compares client content across engagements.
- Analytical retries require a new attempt number under the same lineage and an approved retry limit.
- Tier 3/4 and external actions have zero automatic retries.

## Conflicting Writes and Reconciliation

Conflicting or stale writes fail with `STALE_STATE` and preserve both proposed versions outside the canonical record. A reconciliation task identifies the valid lineage, invalidates dependent records, records the discarded proposal, and requires independent verification for Material work. Manual “last write wins” is prohibited.
