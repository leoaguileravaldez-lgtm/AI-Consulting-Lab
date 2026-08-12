# Decision Gates, Approvals, and Provenance

## Decision semantics

Records distinguish `RECOMMENDATION`, `PROPOSED`, `APPROVED`, `REJECTED`, `DEFERRED`, `EXPIRED`, `SUPERSEDED`, and `REVOKED`. Recommendation and proposal are non-authoritative. Approval is exact actor/category/scope/version/conditions/time authority; it is not execution, evidence validation, professional judgment, QA, release, access, or external-action authorization.

## Decision gates

Each gate declares exact purpose, decision class, materiality, client/entity/engagement/jurisdiction boundary, prerequisites, evidence/analysis/challenge/risk-QA/compliance references, conflicts, SOD constraints, approval requirements, expiry, and failure route. Gates use conjunction. Missing or failed Material inputs cannot be averaged, waived by scoring, or treated as passed.

## Requests and records

A Decision Request preserves the recommendation, alternatives including no-action, rationale, expected benefits, inherent/residual risk, obligations, uncertainties, limitations, dissent, reversibility, consequences, expiry, and exact gate result. It cannot substitute for the Decision Record's independent snapshot.

A Decision Record contains exhaustive typed exact-ID/version/hash input collections for evidence, analytical/professional, risk, compliance, jurisdiction, policy, contract, conflict, approval requirements, approvals, exceptions, waivers, dependencies, and Human Principal references. Each entry snapshots boundary, source authority, lifecycle/currentness, and validation. Counts, manifest hash, snapshot time, empty-category validations, and reconciliation records make completeness independently auditable against the exact Gate and Request sets.

Decision provenance is deterministic: `Decision Record -> typed input manifest -> exact canonical source objects -> complete approval requirements -> exact Approval Records -> effective-actor/SOD evaluation -> exception/waiver evaluation -> conflict state -> canonical authorities -> Human Principal reference where applicable -> Layer 11 workflow reference`. Layer 11 alone governs execution flow; separate action-specific authority remains required.

## Approval validity

Approval Requirements name exact category, minimum authority, independence, scope, sequence or concurrency, required information, validity window, and non-delegability. Approval Records bind exact requirement, decision request/version, approver identity/role/authority record, client/entity/engagement/jurisdiction, decision category, disposition, conditions, time, expiry, and audit.

The Decision Record reconciles its exhaustive Approval Requirement set to obtained, missing, rejected, expired, revoked, superseded, and conditionally valid approvals. Counts, exact membership, ordering, authority, effective actor, SOD, conditions, and boundaries must all pass. `APPROVED` is impossible unless the chain is `COMPLETE_CURRENT_SOD_VALID`; truncation or omission is blocking.

Stale, replayed, copied, wrong-category, wrong-version, out-of-scope, expired, revoked, conditional-but-unsatisfied, cross-client, cross-engagement, circular, self-approved, or unverified approvals are invalid. Silence, timeout, urgency, retry, CRM status, consensus, majority, confidence, or workflow completion is never approval.

Decision reversal or amendment creates a new governed record. It cannot delete prior decisions or undo external effects. Expiry suspends future reliance; supersession and revocation preserve full lineage and propagate revalidation.
