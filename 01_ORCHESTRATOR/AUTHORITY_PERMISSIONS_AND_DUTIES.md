# Human Authority, Permissions, and Approval Integrity

## Canonical Scope

This document is the canonical design for Human Principal control, approval categories, permission tiers, trusted authorization, and separation of duties. Approval does not cure a failed non-waivable control.

## Approval Categories

| Category | Meaning | Authorized decision-maker | Cannot authorize |
|---|---|---|---|
| Business approval | Accepts a recommendation or internal business direction | Human Principal or scoped human delegate | Technical access, deployment, policy exception unless separately authorized |
| Governance approval | Approves design/policy baseline or governance change | Human Principal | Violation of law, binding obligation, or non-waivable control |
| Technical authorization | Grants bounded identity, system, data, tool, and tier access | Human Principal or specifically authorized human technical owner | Business decision, deployment, or tier above scope |
| Deployment authorization | Authorizes exact version into an approved environment | Human Principal where non-delegable, otherwise explicitly scoped human | Changed artifact/environment or external commitment beyond scope |
| Exception approval | Accepts a narrow time-bounded deviation with compensating controls | Human Principal only | Any non-waivable control |
| Recovery authorization | Authorizes an exact checkpoint, reconciliation, or logical rollback plan | Scoped recovery authorizer; Human Principal when Material/non-delegable | Business decision, new external action, deployment, or exception |

One category never implies another. A Human Principal business decision does not automatically grant technical access, deployment authority, security acceptance, or exception approval. Every required category must have its own record or an explicit multi-category record satisfying each category's fields and authority.

Recovery duties and additional authorization conditions are canonical in `DELIVERABLE_LIFECYCLE_AND_RECOVERY.md`.

## Human Principal Authority

The Human Principal retains final strategic/policy authority, exceptions, Critical residual-risk acceptance, confidentiality/segregation waiver where lawful and client-authorized, irreversible external action, legal commitments, filings/representations, public statements, and non-delegable financial commitments. Delegates require written scope, limits, engagement, categories, dates, and cannot subdelegate.

The approval interface must present failed controls, unresolved contradictions, confidence, validation independence, risk, and requested category. Approval cannot hide or automatically override them. If an action requires an exception, the exception must be separately and explicitly approved before the action approval can become valid.

## Permission Tiers

Tiers 0–4 are inherited unchanged from `00_CORE/approval_policy/APPROVAL_POLICY.md`. The trusted permission authority issues a bounded authorization reference containing actor, engagement, task/purpose, tier, data class, allowed capability, start/expiry, conditions, issuer, and revocation status. Specialists cannot create, edit, or self-issue authorization.

During design, the ceiling remains Tier 1 analysis and Tier 2 Markdown preparation. No design record is operational authorization.

## Approval Record Integrity

A valid approval record must contain:

- unique approval ID and category;
- authenticated approver identity and authority basis;
- engagement and task IDs;
- exact scope and decision;
- artifact/action ID, version, and content hash where applicable;
- recipient, system/environment, financial amount/limit, and tier;
- evidence/confidence, validation, risk, contradiction, security, and exception references;
- conditions, valid-from, expiration, and revocation status;
- timestamp and one-time approval nonce or equivalent replay-resistant reference;
- authorized executor and required verification.

The future trusted approval writer must be outside specialist and synthesis control. Agents may prepare an approval request but cannot create the approver authentication, set an approval to granted, change scope, or clear revocation. The transition controller independently verifies identity, authority, category, exact version/hash, nonce/replay status, conditions, time window, and unchanged risk before consuming approval.

Approval for one artifact, version, action, recipient, system, or decision never authorizes another. A Material change invalidates approval. Consumption for a one-time action marks the authorization consumed without deleting history.

This design requires integrity and replay resistance but does not claim cryptographic security; implementation must select and validate an enforceable mechanism.

## Separation of Duties

Material/Critical work maintains distinct accountable identities for primary preparer, challenger, validator, risk reviewer, synthesis-integrity reviewer, approver, executor, registry administrator, permission issuer, and audit verifier. Incompatible combinations:

- preparer cannot validate, approve, or perform synthesis-integrity review;
- synthesizer cannot approve or perform final synthesis-integrity review;
- conflicted actor cannot challenge, validate, risk-review, approve, or accept the affected risk;
- specialist cannot administer its own registry/qualification/permissions;
- executor cannot change the approval and must verify exact scope;
- audit producer cannot independently verify its own audit sequence.

Critical work cannot be validated solely by AI. The same human may hold multiple administrative roles only under a documented small-team control with independent review and never in a prohibited combination.

## Approval Failure Behavior

Missing, ambiguous, expired, revoked, replayed, wrong-category, wrong-authority, wrong-version, or condition-failing approval produces `BLOCKED_AUTHORIZATION`. No timeout or Human Principal availability failure becomes implicit approval.
