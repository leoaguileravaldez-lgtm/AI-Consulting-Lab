# Deliverable Record Schema

## Subordination

This schema is a Layer 07 presentation projection of the canonical deliverable record owned by `01_ORCHESTRATOR`. It cannot create identity, state, approval, release or external-action authority.

## Required fields

- `presentation_artifact_id`, `schema_version`, `record_version`;
- `canonical_deliverable_ref`, `canonical_deliverable_version`;
- `engagement_id`, `scope_version`, `deliverable_type`, `audience`, `purpose`, `materiality`;
- `content_version_ref`, `presentation_version_ref`, `artifact_hash`;
- `analysis_refs`, `recommendation_refs`, `claim_refs`, `evidence_refs`;
- `challenge_refs`, `dissent_refs`, `qa_refs`, `residual_risk_refs`;
- `professional_review_refs`, `human_disposition_refs`, `measurement_refs`;
- `lineage_manifest_ref`, `disclosure_manifest_ref`, `citation_manifest_ref`;
- `semantic_review_ref`, `numerical_review_ref`, `multiformat_review_ref`;
- `confidentiality_class_ref`, `intended_distribution_class_ref`;
- `canonical_approval_release_refs`, `dependency_manifest_ref`;
- `artifact_status`, `generated_at`, `generated_by`, `superseded_by`, `audit_refs`.

Artifact status is advisory metadata only. “Latest” references and silent approval inheritance are prohibited.

