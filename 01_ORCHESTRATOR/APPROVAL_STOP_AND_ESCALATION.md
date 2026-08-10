# Stop, Escalation, and Exception Protocol

## Canonical References

Approval categories, authority, and record integrity are defined only in `AUTHORITY_PERMISSIONS_AND_DUTIES.md`. States and recovery transitions are defined only in `STATE_MACHINE.md`. This document defines stop detection, escalation, and exception handling.

## Mandatory Stop Conditions

Fail closed when evidence is insufficient; confidence is below the applicable gate; Material sources conflict; required assurance is unavailable/failed/non-independent; engagement/client/data purpose is uncertain; authority or approval is ambiguous; security or audit integrity fails; a request exceeds tier; legal/regulatory judgment is unresolved; exposure exceeds a limit or no limit exists; irreversible action lacks required approval; conflicts remain; state/version/idempotency is stale; tool behavior is anomalous; or policies conflict without a clear more-restrictive result.

Stop scope follows the circuit-breaker boundaries in `DELIVERABLE_LIFECYCLE_AND_RECOVERY.md`. A stopped task cannot advance through an unlisted or direct transition.

## Escalation Record

Required fields: escalation ID, engagement/task IDs, trigger, classification, affected state, detector identity/time, facts, unknowns, affected objects, containment, options, safest recommendation, required approval category/authority, response target if approved, resolution, resumption conditions, audit references, and status.

## Protocol

1. Freeze the affected transition and downstream reliance.
2. Trip the narrowest sufficient circuit breaker.
3. Preserve evidence and perform only minimum authorized containment.
4. Append escalation and containment events through the trusted audit path.
5. Route to the authorized human without broadening recipients.
6. Present facts, uncertainty, alternatives, risk, and non-waivable controls.
7. Treat timeout, silence, unavailable approver, and urgency as no approval.
8. Resume only through the canonical recovery transition after conditions are verified.

## Exception Control

Only the Human Principal may approve an exception. The exception record requires policy/control, engagement/task/artifact, rationale, risk, alternatives, compensating controls, monitoring, owner, effective/expiry dates, approval identity/category, and closure. Exceptions are narrow, time-bounded, non-precedential, and cannot waive the non-waivable controls in `00_CORE/approval_policy/APPROVAL_POLICY.md`.

An exception record does not itself approve business action, technical access, deployment, or delivery. Those categories require separate valid approval records.
