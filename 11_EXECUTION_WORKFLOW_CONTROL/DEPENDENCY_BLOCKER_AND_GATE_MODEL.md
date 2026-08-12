# Dependency, Blocker, and Gate Model

Dependency types are `HARD`, `SOFT`, `EVIDENCE`, `APPROVAL`, `PROFESSIONAL_REVIEW`, `CLIENT_INFORMATION`, `RISK_QA`, `DELIVERABLE`, `RELEASE`, and `HUMAN_PRINCIPAL`.

Every dependency records exact source owner, object ID/version/hash where applicable, required status/condition, purpose, client/engagement boundary, freshness, expiry/revocation, validation method, satisfaction reference, limitations, and audit.

A Material soft dependency is operationally hard until explicitly classified non-Material by the canonical owner. Missing Material dependencies block progress. Stale, superseded, ambiguous, contradictory, revoked, cross-client, cross-engagement, self-reported, or wrong-version objects cannot satisfy a dependency.

## Canonical Work Item dependency declaration

Every Work Item declares its complete prerequisite set through exact Dependency Record ID/version references and an explicit declared count. Dependency Records are the only schema objects that may populate this set; prose, labels, `dependency_state`, Transition Requests, Readiness Assessments, inferred relationships, and references directly to upstream objects cannot substitute for the declaration.

The set is reconciled deterministically as `Work Item ID/version -> Dependency Record ID/version -> source owner + source object ID/version/hash -> applicable canonical authority layer`. Reconciliation verifies uniqueness, completeness against the originating request and applicable canonical requirements, reverse binding to the same Work Item/version, type, materiality, required condition, freshness, status, boundary, authorization, contradictions, and audit lineage. Cycles and self-dependencies are invalid. A change to membership, type, source binding, or required condition creates new versioned records; it never mutates or launders the prior set.

An explicitly empty set requires the validated-empty representation defined by the Work Item schema. No omission, null, default, zero inferred from silence, or downstream absence means empty. Failure to reconcile every declared entry, or discovery of an undeclared required prerequisite, makes the Work Item `DEPENDENCY_BLOCKED` or `REVALIDATION_REQUIRED`.

## Release dependency boundary

`RELEASE` represents only an exact, current Layer 07 release-governance object and required condition. It is distinct from `DELIVERABLE`, which represents a deliverable artifact or readiness input. Recording or satisfying `RELEASE` means only that Layer 07's exact referenced state meets the declared dependency condition. Layer 11 cannot create, approve, waive, publish, deliver, execute, infer, or proxy a release; missing or disputed Layer 07 authority blocks the dependency.

Blockers are explicit records with type, severity/materiality proposal, affected scope, opened reason/time, owner, authority, required resolution, status, and audit. Layer 11 cannot close a substantive blocker; it records the canonical resolution reference.

Readiness gates use conjunction, never averaging or scoring compensation. Every Material guard must pass independently. A waived gate requires a valid upstream exception that is legally/policy permissible, exact-scope, current, and cannot waive truth, confidentiality, segregation, required professional review, or Human authority.
