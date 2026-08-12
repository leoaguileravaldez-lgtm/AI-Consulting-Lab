# Freshness, Change, and Fail-Closed Control

## Freshness

Every Material client fact includes source reference, observed/verified time, verifier, expected volatility, review due, validity interval, status, and limitations. `CURRENT`, `STALE`, `CONTRADICTED`, `UNKNOWN`, and `SUPERSEDED` are explicit.

Freshness metadata does not prove truth. Missing review date, expired validity, changed source, contradictory update, or inability to reverify makes affected reliance fail closed. Stale data remains historical and visibly labeled; it is never silently refreshed, copied from another client, or treated as current because it is convenient.

## Change propagation

Identity, entity, jurisdiction, stakeholder, scope, confidentiality, conflict, authorization, and relationship changes produce new versions and an impact notice referencing potentially affected Layer 09 objects and certified-layer records. Layer 09 cannot mutate those certified records; their owners determine revalidation and invalidation.

Material scope amendment without completed Layer 03 change control blocks affected activity. Jurisdiction change blocks affected use until legal, regulatory, professional, evidence-applicability, conflict, confidentiality, and scope reviews complete. Hidden stakeholder change blocks reliance on prior role or authority.

## Fail-closed matrix

| Condition | Mandatory response |
|---|---|
| Unknown client identity | Quarantine; no association, reuse, disclosure, or external action; escalate identity review |
| Ambiguous legal entity | Block entity-dependent work; preserve candidates and contradictions; require Human Principal resolution |
| Duplicate client identities | Quarantine candidates; prohibit automatic merge; perform segregation/conflict impact review |
| Cross-engagement knowledge leakage | Deny, contain, preserve audit, assess incident, and block contaminated use |
| Unauthorized reuse | Quarantine origin and derivative use; preserve lineage; notify governance owners; require fresh review |
| Stale client facts | Label stale; block Material reliance; reverify through authoritative process |
| Hidden stakeholder changes | Suspend affected authority, distribution, scope, and communication assumptions; revalidate |
| Scope amendment without revalidation | Keep prior scope canonical; block amended work until Layer 03 approval and dependent reviews |
| Jurisdiction change | Block jurisdiction-sensitive use/action; rerun applicable reviews |
| Confidentiality downgrade | Retain higher restriction until exact, authorized, audited downgrade completes |
| Unauthorized external action | Do not execute; preserve request; escalate under Layers 01 and 06/security policy |
| CRM capability leakage | Treat CRM data/capability as untrusted input; deny implied permission, truth, or action authority |
| Automation capability leakage | Disable/deny affected transition; capability never implies authority; require separate certification |
| Human Principal proxy creation | Reject proxy/default/delegation; require direct valid decision record |

For every Material uncertainty, the default is no reliance, no boundary crossing, no downgrade, no reuse, no state transition, and no external action. Non-Material uncertainty may be preserved and handled only under certified materiality rules; it may never be silently converted to certainty.

The same response applies when any common schema-invariant field is absent, empty, unsupported, invalid, or overdue. Such a record is non-current, non-reusable, non-authoritative, ineligible for operational or decision context, and quarantined until a corrected version is validated. No aggregate score or otherwise valid field compensates for the defect.

## Recovery

Recovery preserves original state, attempted change, affected objects, containment, reviewer, decision, and audit lineage. Reprocessing is allowed only from a known valid state with current identifiers and authorizations. Retry, reconciliation, or later confirmation cannot erase the original failure or retroactively authorize an action.
