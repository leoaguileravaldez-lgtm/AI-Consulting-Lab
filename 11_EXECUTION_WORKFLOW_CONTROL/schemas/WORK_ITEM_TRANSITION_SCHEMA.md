# Work Item and Transition Schemas

## Work Item Record

Required fields:

- `work_item_id`, `schema_version`, `record_version`, `work_item_type`, `parent_work_item_id`;
- `client_instance_id`, `client_id`, `legal_entity_id`, `engagement_id`, `client_security_domain`, `engagement_security_subdomain` where applicable;
- `originating_request_ref`, `originating_layer`, `purpose`, `classification_ref`, `confidentiality_state`;
- `responsible_role`, `assigned_role`, `workflow_state`, `readiness_state`, `dependency_state`, `blocker_state`, `priority_classification`;
- `dependency_set_status`, `declared_dependency_count`, `dependency_record_refs_and_versions`, `dependency_set_validation_ref`, `dependency_set_validated_at`;
- `created_at`, `updated_at`, `created_by`, `retry_count`, `recurrence_count`, `timeout_policy_ref`;
- `exception_status`, `escalation_status`, `cancellation_status`;
- `required_evidence_refs`, `required_challenge_refs`, `required_risk_QA_refs`, `required_Human_Principal_decision_refs`;
- `completion_criteria`, `freshness_status`, `review_due`, `provenance_refs`, `limitations`, `canonical_authority_refs`, `audit_refs`;
- `invariant_status`, `invariant_validation_ref`, `supersedes`, `superseded_by`.

`dependency_record_refs_and_versions` is the canonical, exhaustive Work Item-level declaration of prerequisites. Every entry identifies one exact Layer 11 Dependency Record ID and record version. `declared_dependency_count` must equal the number of unique entries, and every declared record must bind back to this exact `work_item_id` and `record_version`. The declaration cannot be supplied, completed, or replaced by a Transition Request, Readiness Assessment, attempt, queue entry, or inferred `dependency_state`.

A Work Item with no prerequisites must carry `dependency_set_status: VALIDATED_EMPTY`, `declared_dependency_count: 0`, an explicitly empty `dependency_record_refs_and_versions` collection, and a current `dependency_set_validation_ref` proving that the originating request and applicable canonical authorities were checked for required prerequisites. A missing collection, null value, omitted count, asserted empty set without validation, or mismatch between count and references is not an empty set and is `DEPENDENCY_BLOCKED`.

The only dependency-set statuses are `DECLARED_UNVALIDATED`, `VALIDATED_COMPLETE`, `VALIDATED_EMPTY`, `INCOMPLETE_BLOCKED`, `CONFLICT_BLOCKED`, and `REVALIDATION_REQUIRED`. Operational eligibility requires `VALIDATED_COMPLETE` or `VALIDATED_EMPTY`. Every referenced Dependency Record must reconcile to the canonical dependency taxonomy, exact source object/version, applicable authority layer, boundary, freshness, status, and audit lineage. Missing, forged, duplicate, circular, self-referential, stale, superseded, incomplete, unauthorized, cross-client, or cross-engagement entries—and mutation of the declared set without a new Work Item record version—make the Work Item non-current and ineligible for readiness, transition, execution, retry, recurrence, and completion.

## Transition Request

Required fields:

- `transition_request_id`, `schema_version`, `record_version`, `work_item_id`, `work_item_version`;
- applicable client/entity/engagement/security boundaries, `purpose`, `classification_ref`, `confidentiality_state`;
- `predecessor_state`, `predecessor_record_version`, `requested_successor_state`, `transition_status`;
- `triggering_event_ref`, `guard_results`, `dependency_refs_and_versions`, `blocker_refs`, `readiness_ref`;
- `actor_ref`, `actor_role`, `reason`, `requested_at`, `Layer_01_authority_ref`, `Human_Principal_decision_ref` where required;
- `freshness_status`, `review_due`, `provenance_refs`, `contradiction_refs`, `limitations`, `audit_refs`;
- `invariant_status`, `invariant_validation_ref`, `created_by`, `created_at`.

An unlisted edge, stale predecessor, missing guard, or absent authority is `AUTHORITY_BLOCKED`.

## Readiness Assessment

Required fields include `readiness_assessment_id`, schema/record/work-item versions, boundaries, purpose/classification/confidentiality, `assessment_status`, `invariant_result`, `work_item_dependency_set_status`, `work_item_declared_dependency_count`, `dependency_record_refs_and_versions`, exact blocker/queue/lease refs, certified owner statuses, freshness/review, contradictions, limitations, assessor/time, canonical authority refs, and audit/invariant validation.

The assessment must reconcile its exact dependency references to the Work Item's same-version canonical `dependency_record_refs_and_versions`; it cannot add, omit, replace, or repair that declaration. Readiness is operational only and cannot certify substantive validity.
