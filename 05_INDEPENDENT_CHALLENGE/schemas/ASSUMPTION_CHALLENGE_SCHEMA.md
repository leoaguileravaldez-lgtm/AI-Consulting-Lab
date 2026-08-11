# Assumption Challenge Schema

## Required Fields

- `assumption_challenge_id`, `challenge_id`, `engagement_id`, `canonical_task_id`.
- `assumption_id`, `assumption_version`, `statement`, `owner`, `origin`.
- `registration_status`: `REGISTERED` or `DETECTED_HIDDEN`.
- `materiality`, `decision_criticality`, `evidence_refs`, `validation_status`, `freshness`.
- `base_value_or_condition`, `credible_range`, `expiry`, `confidence`.
- `dependent_claims`, `models`, `outputs`, `recommendations`.
- `attack_method`, `sensitivity`, `decision_switch_threshold`, `interaction_failures`.
- `finding`, `required_evidence`, `producer_response_ref`, `rechallenge_status`, `audit_refs`.

## Rules

The schema cannot create or verify a canonical fact. `DETECTED_HIDDEN` requires registration/remediation through the certified owners before Material reliance. Reuse, consensus, response or Human approval cannot convert an assumption to fact.

Changed evidence, range, model or dependency invalidates the affected challenge result and invokes canonical re-evaluation.
