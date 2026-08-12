# Material Object Coverage Audit

Every concrete schema was independently checked for physical incorporation of identity/version; actor/principal/effective actor; class/aliases; client/entity/engagement/jurisdiction; role/authority/permission/access; purpose/confidentiality/object/action; time/freshness; provenance/issuer/grantor/approver/reviewer; revocation/supersession; dependencies/limitations; audit; and validation.

| Object group | Objects | Key structural controls | Result |
|---|---:|---|---|
| Identity resolution | 4 | Canonical identity, aliases, verification, effective actor | PASS |
| Actor/principal | 2 | Identity/principal/controller attribution | PASS |
| Role governance | 2 | Definition separated from assignment and grants | PASS |
| Grants and policy | 3 | Authority/permission separation; default-deny policy | PASS |
| Access request/decision | 2 | Typed request chain, zero-deputy identity equality, exact request/Authorization comparison, state-locked projection | PASS |
| Delegation/revocation | 2 | Source subset, non-transitive default, propagation | PASS |
| Privilege elevation | 2 | Distinct effective actors, expiry, SOD | PASS |
| Purpose/context | 2 | Non-transferable purpose; exact reconciled context | PASS |
| Client/engagement/entity boundaries | 3 | Exact nested scope; relationships grant nothing | PASS |
| Jurisdiction/confidentiality | 2 | Exact constraints; no implicit downgrade | PASS |
| SOD | 1 | Mandatory Layer 12 and Layer 13 sets | PASS |
| Machine identity | 1 | Human separation, self-authorization prohibition, caller/deputy intersection | PASS |
| Session/authentication | 2 | Reference only; no artifacts; no authority inference | PASS |
| Authorization/audit | 2 | Exhaustive typed request comparison, zero-deputy proof, ordered chain/intersection, completeness lineage manifest | PASS |
| Break-glass | 2 | Exact Authorization + Access binding, typed contemporaneous execution reconciliation, single use, non-nesting, intersection, expiry, non-waivable controls | PASS |
| **Total** | **32** |  | **PASS** |

Missing or invalid common or specific fields prevents `VALIDATED_CURRENT`. Empty manifest categories require exact validated-empty records; absence never means none. No score compensates.

Remediation audit confirms Access Decision cannot independently assert state or scope; all projection fields reconcile to exact authoritative Authorization identity/version/hash and cannot be more permissive. Authorization and audit schemas require exact Access Request ID/version/hash, a typed per-field request comparison, originator, every ordered deputy/intermediary and adjacency, final executor, per-member authority and boundary state, chain count/hash, least-privilege intersection/hash, and a completeness manifest. Direct access requires both validated-empty proof and canonical requester/originator/executor/effective-actor equality. Break-glass is subordinate to the same canonical Authorization Decision and cannot change actor, executor, chain, scope, or boundaries.

Emergency remediation additionally requires the exact reconciled Access Decision, same authorization-chain and execution-instance identity, canonical source typing, validated absence of parent emergency authority, zero emergency-delegation depth, one unique use ID, permitted count one, immutable append-only consumption state, replay rejection, and audit completeness through outcome. A post-hoc or hash-only assembly cannot validate use.
