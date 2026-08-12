# Dependency and Attempt Schemas

## Dependency Record

Required fields include `dependency_id`, `schema_version`, `record_version`, `work_item_id`, `work_item_version`, `dependency_type`, `materiality`, `source_authority_layer`, `source_object_id`, `source_object_version`, `source_object_hash` where applicable, `required_condition`, client/entity/engagement/security boundary fields, `purpose`, `classification_ref`, `confidentiality_state`, `dependency_status`, `freshness_status`, `expires_at`, `revocation_status`, `validation_method`, `satisfaction_ref`, `contradiction_refs`, `limitations`, `created_by`, `created_at`, `canonical_authority_ref`, `audit_refs`, `invariant_status`, and `invariant_validation_ref`.

Each record must be present exactly once in the same-version Work Item's `dependency_record_refs_and_versions`. Its reverse Work Item binding, source binding, and authority-layer binding must reconcile exactly. A Dependency Record cannot depend on itself or create a cycle through another Dependency Record. Forged, orphaned, duplicated, mutated, or unreconciled records are `DEPENDENCY_BLOCKED` and cannot support readiness.

For type `RELEASE`, `source authority layer` must be Layer 07 and the source must be an exact Layer 07 release-governance object/version. Dependency satisfaction records Layer 07 state only and grants Layer 11 no release, publication, delivery, approval, waiver, or execution authority.

## Blocker Record

Required fields include `blocker_id`, schema/record/work-item versions, blocker type/materiality/status, boundaries, purpose/classification/confidentiality, opened reason/time/by, affected scope, owner/canonical authority, required resolution, resolution reference/version, freshness/review, limitations, audit, and invariant validation.

## Execution Attempt

Required fields include `attempt_id`, schema/record/work-item versions, attempt number, prior attempt refs, retry count, boundaries, purpose/classification/confidentiality, attempt status, exact input/dependency/readiness versions, idempotency boundary, lease ref, start/end, failure/output refs, freshness/revalidation results, limitations, actor/role, canonical authority, audit, and invariant validation.

Prior failure records are immutable; repetition cannot establish success.

## Recurrence Record

Required fields include `recurrence_id`, series/work-item/schema/record versions, recurrence count, cadence/end/review policy, creation authority, boundaries, purpose/classification/confidentiality, status, prior occurrence refs, independently revalidated evidence/approval/client/engagement/jurisdiction/applicability/dependency refs, freshness, limitations, creator/time, audit, and invariant validation.
