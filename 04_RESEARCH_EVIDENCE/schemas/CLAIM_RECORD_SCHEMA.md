# Claim Record Schema

## Canonical Compatibility

The canonical claim owner and allowed types remain in `01_ORCHESTRATOR`. This schema is a research/evidence projection and relation contract only.

## Required Fields

- `claim_id`, `canonical_claim_ref`, `canonical_claim_version`, `engagement_id`, `task_id`, `workstream_ref`.
- `claim_text`, canonical `claim_type`, subordinate `research_label`, `materiality`, `decision_critical`.
- `supporting_relation_refs`, `contradicting_relation_refs`, `context_relation_refs`, `invalid_relation_refs`.
- `assumption_refs`, `method_ref`, `calculation_model_refs`, `source_lineage_refs`.
- `jurisdiction`, `time_scope`, `population`, `definitions`, `units`.
- `evidence_assessment`: `SUPPORTED`, `PARTIALLY_SUPPORTED`, `CONTRADICTED`, `INSUFFICIENT_EVIDENCE`, or `NOT_VERIFIABLE`.
- `canonical_confidence_ref`, `validation_ref`, `limitations`, `contradiction_refs`, `impact_notice_refs`.
- `source_versions`, `freshness_as_of`, `created_at`, `created_by`, `audit_ref`.

## Invariants

Evidence assessment is not a task state, approval, confidence level or authorization. The canonical claim controls over this projection. A changed claim text or evidence basis creates a new version and reassessment.

An assumption cannot silently become fact. An inference remains linked to premises and method. Supporting evidence for a narrower proposition cannot support a broader claim. Missing, stale, revoked, superseded, contaminated or materially contradicted support causes impact notification and fail-closed reliance.
