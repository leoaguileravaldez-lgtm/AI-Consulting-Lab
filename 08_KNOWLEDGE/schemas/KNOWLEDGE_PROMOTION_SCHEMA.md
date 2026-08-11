# Knowledge Promotion Schema

## Required fields

- `promotion_id`, `schema_version`, `record_version`, `candidate_knowledge_ref`;
- `originator`, `candidate_owner`, `origin_engagement_ref`, `knowledge_type`;
- `source_lineage_ref`, `independence_group_refs`, `supporting_and_contradicting_refs`;
- `deidentification_producer`, `deidentification_reviewer`, `deidentification_result`;
- `generalization_producer`, `generalization_reviewer`, `generalization_result`;
- `evidence_validator_ref`, `challenge_review_ref`, `qa_review_ref`, `professional_review_ref`;
- `failure_memory_completeness`, `scope`, `limitations`, `known_exceptions`, `reuse_policy`;
- `independence_assessment_ref`, `reuse_authorizer_ref`, `authorization_ref`;
- `promotion_result`, `created_at`, `audit_refs`.

Promotion results are knowledge-record metadata only. The producer cannot be sole Material reuse validator. Passed reviews without exact canonical authorization do not permit reuse.

