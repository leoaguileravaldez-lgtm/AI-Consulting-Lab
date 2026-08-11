# QA Findings and Remediation

## Finding record

A finding is an immutable, versioned assurance record bound to the tested control, exact object/version, evidence, materiality, cause, observed condition, expected requirement, impact, owner, recommended response, independence metadata, and audit references.

Finding workflow labels are evidentiary record labels only:

`IDENTIFIED` → `CONFIRMED` → `REMEDIATION_PROPOSED` → `REMEDIATION_IN_PROGRESS` → `REMEDIATION_EVIDENCE_SUBMITTED` → `VERIFICATION_REQUIRED` → `VERIFIED`

Alternative labels are `UNRESOLVED`, `ACCEPTED_EXCEPTION`, and `SUPERSEDED`. These do not constitute canonical states or the canonical block/hold lifecycle. `VERIFIED` does not resolve a blocker, approve progression, or accept risk. `ACCEPTED_EXCEPTION` means only that an exact canonical exception was verified.

## Remediation verification

The accountable owner performs remediation under canonical authority. Evidence identifies the exact finding, changed objects, implementation actor, time, test result, dependencies, side effects, and audit lineage. An eligible independent QA verifier re-performs or inspects the required test.

Self-attestation, checklist completion, management assertion, elapsed time, or unchanged narrative cannot establish remediation. Failed verification creates a new finding version and remains unresolved.

## Recurrence

A recurring condition creates a new finding linked to prior occurrences, common cause, prior remediation, recurrence interval, affected scopes, and trend. Recurrence never silently reopens or resolves a canonical blocker. Material recurrence generates an advisory impact notice for canonical escalation and preserves every prior record.

