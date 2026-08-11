# Reuse Decision Schema

## Required fields

- `reuse_decision_id`, `schema_version`, `record_version`;
- `receiving_engagement_id`, `knowledge_id`, `knowledge_version`, `injection_class`;
- `intended_use`, `requester`, `target_question_or_workstream`;
- `applicability_assessment_ref`, `freshness_result`, `client_boundary_result`;
- `source_evidence_status_refs`, `contradiction_refs`, `method_compatibility_ref`;
- `professional_dependency_ref`, `challenge_requirement`, `qa_requirement`;
- `limitations`, `required_new_research`, `reuse_authorization_ref`;
- `advisory_result`, `created_by`, `created_at`, `audit_refs`.

Advisory results cannot grant permission, validate evidence, set confidence, approve a recommendation or change state. `UNKNOWN`, missing authorization or Material unresolved weakness is non-reusable.

