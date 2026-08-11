# Presentation Version Schema

## Required fields

- `presentation_version_id`, `schema_version`, `record_version`, `engagement_id`;
- `presentation_artifact_id`, `canonical_deliverable_ref`, `content_version_ref`;
- `parent_presentation_version_ref`, `artifact_hash`, `format`, `renderer_or_method_version`;
- `change_description`, `change_classification`, `changed_locations`;
- `semantic_comparison_method`, `semantic_comparison_result`, `reviewer`, `independence_ref`;
- `disclosure_equivalence`, `risk_dissent_professional_equivalence`;
- `audience_purpose_equivalence`, `hidden_content_review_ref`;
- `prior_approval_ref`, `approval_impact`, `created_at`, `audit_refs`;
- `artifact_status`, `invalidation_reason`, `superseded_by`.

Only proven non-semantic changes qualify as presentation-only. `UNDETERMINED`, wording, emphasis, omission, redaction, compression, chart framing or disclosure-placement uncertainty requires review. Prior approval cannot silently transfer.

