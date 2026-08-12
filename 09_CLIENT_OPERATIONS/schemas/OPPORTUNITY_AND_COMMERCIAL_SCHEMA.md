# Opportunity and Commercial Schemas

## Opportunity Record

Required fields:

- `opportunity_id`, `schema_version`, `record_version`, `client_id`, `legal_entity_id`, `client_security_domain`;
- `purpose`, `opportunity_name`, `operational_need_summary`, `stage`, `stage_basis_ref`, `stage_as_of`;
- `value_range`, `currency`, `probability_estimate`, `estimate_method`, `estimate_as_of`;
- `owner_ref`, `stakeholder_refs`, `proposed_scope_ref`, `jurisdiction_refs`;
- `classification_ref`, `conflict_refs`, `pressure_event_refs`, `contradiction_refs`, `limitations`;
- `source_refs`, `freshness_status`, `verified_or_assessed_at`, `review_due`;
- `engagement_creation_decision_ref`, `engagement_id` if separately created by Layer 03;
- `schema_invariant_status`, `schema_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Stage, value, and probability are operational metadata and grant no analytical, approval, access, or action authority.

## Commercial Metadata Record

Required fields:

- `commercial_record_id`, `schema_version`, `record_version`, `client_id`, `legal_entity_id`, `client_security_domain`;
- `engagement_id` where applicable, `commercial_metadata_type`, `value_or_status_label`;
- `purpose`, `commercial_status`;
- `currency`, `effective_from`, `effective_until`, `authoritative_system_ref`, `source_ref`;
- `contract_status_ref`, `purchase_order_ref`, `billing_cadence`, `invoice_status_ref`, `payment_status_label` as applicable;
- `classification_ref`, `freshness_status`, `verified_or_assessed_at`, `review_due`, `contradiction_refs`, `limitations`;
- `schema_invariant_status`, `schema_validation_ref`, `supersedes`, `superseded_by`, `created_by`, `created_at`, `audit_refs`.

Restricted financial values, payment instruments, credentials, substantive contracts, invoices, and signatures are not stored in this record. Metadata creates no authority to bind, bill, collect, pay, or recognize revenue.
