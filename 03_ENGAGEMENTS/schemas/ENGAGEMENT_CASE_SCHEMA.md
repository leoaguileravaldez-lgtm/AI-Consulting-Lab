# Engagement Case Schema

## Purpose

This is a design contract for a referential aggregate, not a canonical Engagement replacement. All source-owned fields are immutable references or derived views.

## Fields

| Field | Requirement / owner |
|---|---|
| `case_schema_version` | Required; `03` schema version |
| `engagement_id` | Required immutable reference to canonical `01` Engagement |
| `canonical_engagement_version` | Required version/hash and freshness |
| `client_project_reference` | Reference only; canonical/security owner controls |
| `title`, `engagement_type`, `sector`, `jurisdictions` | Source references with observed version |
| `problem_contract_id` | Required `03` record |
| `scope_reference`, `exclusions_reference`, `constraints_reference` | Canonical/source references |
| `materiality`, `confidentiality`, `exposure_profile` | Derived display of canonical classification; not editable here |
| `requested_outcome`, `required_deliverables`, `target_date` | Planning values linked to approved scope/change records |
| `specialist_routing_request_ids` | Capability needs only; activation remains `01` |
| `gate_profile` | Applicable/N/A gates and rationale; no transition authority |
| `workstream_ids` | `03` projections linked to canonical tasks |
| `open_blocker_refs`, `contradiction_refs` | Canonical/source references, never locally cleared |
| `evidence_package_refs`, `audit_refs` | References; no validation ownership |
| `measurement_plan_ref` | Certified transversal capability reference or governed N/A |
| `derived_phase_view`, `canonical_task_status_refs` | Deterministic view plus source versions |
| `decision_history_refs` | Exact canonical decision/approval references |
| `change_record_ids` | Append-only `03` records and source effects |
| `closure_readiness` | Derived checklist; never closure authority |
| `source_refresh_at`, `stale_fields` | Required integrity metadata |

## Invariants

The engagement ID and client/security boundary must match every child. Source mismatch, missing version, stale Material field or unmapped canonical state invalidates readiness. No field may encode an approval, state transition, specialist assignment, external action or success certification.

## Future Interfaces

Evidence, challenge, Risk/QA, deliverable, client and automation references map to future independent modules. They remain opaque identifiers until those modules are separately authorized.
