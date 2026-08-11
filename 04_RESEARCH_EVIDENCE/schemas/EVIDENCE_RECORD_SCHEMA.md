# Evidence Record Schema

## Required Fields

- `evidence_id`, `schema_version`, `record_version`, `engagement_id`, `task_id`, `source_id`, `created_at`, `created_by`, `audit_ref`.
- `source_version_ref`, `retrieval_proof_ref`, exact `content_coordinates`, and support description.
- `research_label`: `FACT`, `SOURCE_CLAIM`, `ASSUMPTION`, `INFERENCE`, `ESTIMATE`, `SCENARIO`, or `EXPERT_JUDGMENT`.
- `canonical_claim_mapping`, `client_information_status`, `jurisdiction`, `time_scope`, `population`, `definitions`, `units`.
- `quality_dimension_refs`, `freshness_status`, `lineage_ref`, `independence_group_id`, `corroboration_group_id`.
- `applicability_result`, `limitations`, `conflicts`, `validation_status`, `validator_ref`.
- `claim_relation_refs`, `contradiction_refs`, `impact_notice_refs`, `prior_version_ref`.

## Rules

The research label cannot create a canonical claim type. `FACT` is illegal unless mapped to a valid canonical `VERIFIED_FACT` after required independent validation. Model output and expert judgment retain their inferential status.

Evidence statuses never approve reliance or establish recommendation confidence. Missing identity, retrieval proof, content match, jurisdiction, provenance, freshness or validation fails affected Material support.

Corrections, revocations, expiry and supersession append new versions and initiate dependency impact. Evidence is always engagement-bound even when its public source identity is reusable.
