# Exception, Escalation, Cancellation, and Rollback

## Exceptions

Exceptions distinguish transient failure, Material failure, dependency failure, evidence invalidation, client-state change, confidentiality change, jurisdiction change, Human intervention, external-system unavailability, credential failure, connector failure, malformed input, and unauthorized-action request.

Material exceptions stop affected execution, preserve state/output/audit, open a blocker, and route to the canonical owner. Layer 11 cannot resolve evidence disputes, professional disagreement, risk acceptance, client conflict, confidentiality conflict, release dispute, or Human decision.

Credential/connector/external-system references describe failures only; no credential or capability exists here. Unauthorized action requests are rejected, recorded, and escalated.

## Escalation

Escalation is advisory routing with issue, materiality, affected scope, exact records/versions, recipient authority, requested disposition, time, status, and audit. Escalated never means resolved or approved. Timeout, consensus, priority, or nonresponse cannot close escalation.

## Cancellation

Cancellation ends future operational eligibility for the exact scope. It preserves work item, attempts, outputs, evidence/analysis references, failures, dissent, risks, QA, decisions, limitations, and audit. It cannot erase or manufacture completion, revoke upstream truth, or undo an external effect.

## Rollback

Rollback is a new governed transition/compensating-action proposal referencing original action, observed outcome, desired state, reversibility evidence, authority, risks, and verification. It never deletes history, rewrites audit, erases evidence/decisions/failures, or pretends execution did not occur. Unknown external outcome or reversibility fails closed and escalates.

