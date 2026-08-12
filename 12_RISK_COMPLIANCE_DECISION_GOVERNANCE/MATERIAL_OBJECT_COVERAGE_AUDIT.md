# Material Object Coverage Audit

Each concrete schema was checked against identity/version, client/entity/engagement, jurisdiction, purpose/classification/confidentiality, lifecycle/status, provenance/source authority, freshness/effective/expiry, owner/reviewer/Human authority, dependencies/conflicts, supersession/retention, audit, limitations, and validation requirements inherited mandatorily from `MATERIAL_OBJECT_INVARIANT.md` and its object-specific fields.

| Object | Boundary/jurisdiction | Lifecycle/version/freshness | Provenance/dependencies | Authority/audit/limits | Result |
|---|---|---|---|---|---|
| Risk Record | Explicit | Explicit | Explicit | Explicit | PASS |
| Risk Assessment | Explicit | Explicit | Exact inputs | Effective actor and mandatory SOD | PASS |
| Risk Classification | Explicit | Explicit | Explicit | Explicit | PASS |
| Risk Exposure | Explicit | Explicit | Explicit | Explicit | PASS |
| Risk Control Reference | Explicit | Explicit | Exact control/assurance | Explicit | PASS |
| Residual Risk Assessment | Explicit | Explicit | Exact inherent/control set | Assessor/acceptor effective-actor SOD | PASS |
| Compliance Obligation | Explicit | Explicit | Exact instrument | Qualified authority | PASS |
| Compliance Assessment | Explicit | Explicit | Exact obligations/controls | Assessor/override effective-actor SOD | PASS |
| Jurisdiction Constraint | Exact | Explicit | Exact instrument/nexus | Qualified authority | PASS |
| Policy Constraint | Explicit | Explicit | Exact policy/version | Owner preserved | PASS |
| Contractual Constraint | Exact entities | Explicit | Exact contract/provision | Authority preserved | PASS |
| Decision Gate | Exact | Explicit | Conjunctive exact set | Non-authoritative | PASS |
| Decision Request | Exact | Explicit | Exact recommendation/inputs | Non-approval | PASS |
| Decision Record | Exact | Expiry/revocation/supersession | Exhaustive typed snapshot and manifest reconciliation | Complete approvals/SOD; execution separate | PASS |
| Approval Requirement | Exact | Explicit | Gate/request/version bound | Mandatory plus additional SOD | PASS |
| Approval Record | Exact | Expiry/revocation/supersession | Requirement/request bound | Effective identity/authority/SOD exact | PASS |
| Escalation Record | Exact | Explicit | Affected versions | Advisory only | PASS |
| Exception Request | Exact | Requested window | Exact requirement/risk | Effective requester; grants nothing | PASS |
| Exception Decision | Exact | Expiry/revocation | Request/version bound | Mandatory requester/decider SOD | PASS |
| Waiver Record | Exact | Time/scope bound | Exception/basis/approval exact | Mandatory requester/approver SOD and non-waivable check | PASS |
| Conflict Record | Exact | Explicit | All positions preserved | Subject/resolver effective-actor SOD | PASS |
| SOD Constraint | Exact | Explicit | Mandatory baseline plus extensions | Effective-actor resolution; baseline non-disableable | PASS |

Any inherited or object-specific Material omission produces a blocking validation state. No score compensates.

The Decision Record was additionally audited for all fourteen typed input categories, exact identity/version/hash and client/entity/engagement/jurisdiction/source-authority/currentness snapshots, category counts, validated empty sets, manifest hash, Gate/Request reconciliation, complete approval-state accounting, mandatory SOD, effective actors, exceptions/waivers/conflicts, Human references, and audit reconstruction. All are concrete required schema fields; indirect lineage cannot substitute.
