# Source Record Schema

## Canonical Status

This is a subordinate design schema compatible with `01_ORCHESTRATOR/RECORD_SCHEMAS_AND_AUDIT.md`. It does not replace the canonical Evidence entity or grant source authority.

## Required Fields

- `source_id`, `schema_version`, `record_version`, `engagement_id`, `task_id`, `created_at`, `created_by`, `audit_ref`.
- `client_security_domain`, `data_classification`, `authorized_purpose`, `access_policy_ref`, `retention_ref`.
- `issuer_owner`, `title`, `source_type`, `provenance_class`, `content_form`, `primary_secondary_status`.
- `publication_date`, `reporting_period`, `effective_date`, `accessed_at`, `as_of_date`.
- `canonical_location_or_identifier`, `authorized_snapshot_ref`, `content_fingerprint` where lawful.
- `jurisdictions`, `population`, `definitions`, `units`, `methodology`, `limitations`, `sponsorship_conflicts`.
- `source_authority`, `quality_dimensions`, `methodological_transparency`, `freshness_status`.
- `root_source_refs`, `parent_source_refs`, `independence_group_id`, `corroboration_group_id`, `common_source_risk`.
- `correction_ref`, `revocation_ref`, `supersedes_ref`, `superseded_by_ref`, `status_reason`.
- `retrieval_proof_refs`, `evidence_record_refs`, `prior_version_ref`.

## Rules

Source category, authority and quality are separate. Unknown lineage does not imply independence. Updates append a version. A source record alone does not prove access, claim support, corroboration, validation, confidence or approval.

Freshness labels are evidence metadata, not canonical workflow states. Cross-engagement references fail unless the source is separately authorized public evidence with engagement-specific retrieval and applicability review.
