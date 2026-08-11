# QA Readiness and Gate Interface

## Advisory results only

QA may record:

- `READY_FOR_CANONICAL_EVALUATION`;
- `READY_WITH_NON_MATERIAL_LIMITATIONS`;
- `NOT_READY`;
- `NOT_ASSESSABLE`;
- `REVALIDATION_REQUIRED`;
- `PROFESSIONAL_REVIEW_REQUIRED`.

These are QA record results, not canonical states, gates, transitions, approvals, blocker resolutions, risk acceptance, release decisions, delivery authorization, or engagement closure. A record must display that limitation wherever its result is presented.

## Canonical interface

QA publishes its immutable assessment and advisory impact notice to `01_ORCHESTRATOR`. Only canonical authority decides whether to create a block/hold record, route remediation, change readiness, commit a transition, request Human Principal action, or perform `RELEASE_CHECK`. `03_ENGAGEMENTS` reflects that canonical result and must not promote the QA result into state.

If notification, referential integrity, or canonical acknowledgement is uncertain, clean reliance is prohibited. QA cannot route around unavailable canonical control.

## Decision packet treatment

Material packets preserve exact QA result, findings, limitations, residual risks, dissent references, professional-review status, exceptions, unresolved issues, assessor independence, object/version, and as-of time. Compression cannot omit a veto condition or translate `NOT_READY` into qualified approval.

Human Principal disposition is recorded separately under canonical semantics. It cannot retroactively change the QA record.

