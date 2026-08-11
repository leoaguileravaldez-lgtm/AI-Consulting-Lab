# Disclosure Requirement Schema

## Required fields

- `disclosure_requirement_id`, `schema_version`, `record_version`, `engagement_id`;
- `presentation_artifact_id`, `content_version_ref`, `audience`, `purpose`;
- `upstream_object_ref`, `disclosure_type`, `materiality`;
- `required_meaning`, `required_prominence`, `placement`;
- `confidentiality_class_ref`, `audience_eligibility_dependency_ref`;
- `treatment`, `redaction_ref`, `restricted_appendix_ref`, `alternate_version_ref`;
- `substitute_disclosure`, `semantic_impact`, `reviewer`, `result`;
- `limitations`, `created_at`, `audit_refs`.

Treatment may identify restricted appendix, alternate audience version, policy withholding, ineligible audience, incomplete context, required qualification or no safe deliverable. It grants no access and creates no state. A Material disclosure cannot be omitted silently.

