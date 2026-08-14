# Deployment Admission and Operational Assurance Governance

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: human-readable architecture projection

## Responsibility

Layer 17 determines whether one exact candidate artifact/configuration/dependency/environment tuple has complete, current and independently validated evidence sufficient for `ADMISSION_ELIGIBLE`. Eligibility means only that the candidate may request separately owned release and deployment authorization. Layer 17 does not release, authorize, deploy, operate, monitor, roll back, observe production as an operational authority, reconcile effects or execute an external action.

The canonical path is:

`AUTHORIZED INPUT → CANDIDATE → ARTIFACT PROVENANCE + CONFIGURATION/DEPENDENCIES + ENVIRONMENT IDENTITY → EXACT TUPLE BINDING → LAYER 16 CONFORMANCE REQUIREMENTS → ASSURANCE EVIDENCE REQUIREMENTS → EVIDENCE SET → OPERATIONAL + ROLLBACK READINESS EVIDENCE → INDEPENDENT REVIEW ASSIGNMENT → PASS/FAIL → ADMISSION_ELIGIBLE/INELIGIBLE → INVALIDATION/AUDIT`.

## Semantic and authority separation

Conformance is the Layer 16 requirement set. Assurance is Layer 17's independent evaluation of evidence. Admission is Layer 17's non-authorizing eligibility disposition. Authorization remains with Layer 01. Execution of a protected deployment or rollback is outside Layer 17 and constrained by Layer 14. Observation is evidence supplied by future owners, not Layer 17 monitoring. Reconciliation of partial or unknown effects remains outside Layer 17.

Layer 07 artifact/release semantics, Layer 15 engagement/conclusion/handoff semantics and every predecessor authority remain unchanged. A successful build, test, assurance PASS, eligibility record, audit entry, technical capability or Human request cannot migrate authority.

## State model

Four dimensions remain separate:

- evidence: `INCOMPLETE`, `COMPLETE_CURRENT`, `CONFLICTED`, `STALE`;
- review: `NOT_ASSIGNED`, `PENDING`, `COMPLETED`;
- verdict: `PASS`, `FAIL`;
- admission: `INELIGIBLE`, `ADMISSION_ELIGIBLE`;
- lifecycle applied to versioned objects: `CURRENT`, `STALE`, `SUPERSEDED`, `REVOKED`.

Absence of a verdict is not an authoritative `UNASSESSED` decision. Conditions not yet satisfied remain incomplete or FAIL; no hidden conditional PASS exists. PASS is necessary but insufficient for eligibility until every exact guard passes.

## Fail-closed behavior

Missing or ambiguous identity, version/hash, provenance, predecessor, membership, authority or independence; candidate/environment/configuration mismatch; stale, superseded, revoked, expired, incomplete or conflicting evidence; or a failed mandatory control denies eligibility. Material change creates an invalidation generation and cannot revive an older verdict.

Rollback-readiness evidence describes plan and recovery sufficiency only. It grants neither rollback authorization nor execution. Audit remains descendant evidence. External-Action Execution and Effect/Outcome Reconciliation remains separate, unnumbered and absent.
