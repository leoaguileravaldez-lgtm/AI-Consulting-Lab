# Professional Review Evidence Schema

## Required fields

- `professional_review_evidence_id`, `schema_version`, `record_version`, `engagement_id`;
- `requirement_trigger_ref`, `professional_domain`, `jurisdiction`;
- `reviewer_identity_ref`, `qualification_or_license_metadata_ref`, `qualification_status_as_of`;
- `independence_and_conflict_ref`, `review_scope`, `question_refs`;
- `target_object_ref`, `target_version`, `target_hash` where available;
- `review_date`, `effective_as_of`, `documented_outcome_ref`, `limitations`, `conditions`;
- `authenticity_and_provenance_refs`, `expiry_or_rereview_trigger`;
- `qa_verifier_ref`, `verification_result`, `created_at`, `audit_refs`.

## Rules

This record proves only whether required review evidence exists and matches the governed requirement. It does not certify substantive correctness. Missing, fabricated, unverifiable, stale, wrong-jurisdiction, wrong-scope, or wrong-version evidence cannot satisfy the requirement. QA may not draft, infer, reinterpret, replace, or approve the professional conclusion.

