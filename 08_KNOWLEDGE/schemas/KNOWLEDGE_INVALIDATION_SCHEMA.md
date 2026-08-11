# Knowledge Invalidation Schema

## Required fields

- `knowledge_invalidation_id`, `schema_version`, `record_version`;
- `knowledge_id`, `knowledge_version`, `trigger_type`, `trigger_ref`, `detected_at`;
- `source_evidence_old_new_refs`, `method_old_new_refs`, `client_boundary_ref`;
- `affected_dependency_refs`, `affected_reuse_decision_refs`, `materiality`;
- `advisory_status`, `reason`, `required_review`, `impact_notice_ref`;
- `created_by`, `created_at`, `audit_refs`, `superseded_by`.

Triggers include source correction/revocation/expiry/supersession, contradiction, freshness or applicability failure, method change, professional-review change and client leakage. Layer 08 may update only knowledge-record metadata and notify `01`; it cannot alter canonical engagement state, blockers, approvals or evidence.

