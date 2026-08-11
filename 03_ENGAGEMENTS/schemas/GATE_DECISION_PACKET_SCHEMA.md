# Gate Decision Packet Schema

## Required Fields

- `packet_id`, `engagement_id`, `gate_id`, `packet_version`, `as_of`, `expires_at` where applicable.
- `applicability`: `APPLICABLE` or governed `NOT_APPLICABLE`, with rule, reason, reviewer and approval reference.
- `canonical_task_refs[]`, `canonical_state_refs[]`, `decision_requested`, `exact_object_refs[]` and hashes.
- `recommended_action`, `alternatives[]`, `consequences[]`.
- `evidence_refs[]`, `freshness`, `assumption_refs[]`, `analysis_refs[]`, `confidence`.
- `dissent[]`, `contradiction_refs[]`, `unresolved_uncertainties[]`, `blocker_refs[]`.
- `financial`, `strategic`, `operational`, `regulatory`, `technology`, `workforce`, `implementation` and `measurement` implications as applicable.
- `challenge_ref`, `validation_refs[]`, `risk_qa_ref`, `synthesis_ref` as applicable.
- `required_approval_category`, `authorized_decision_maker`, `permitted_presentation_labels[]`.
- `human_decision_ref`, `canonical_transition_ref`, `conditions[]`, `audit_refs[]`.

## Disposition Mapping

- `GO`: valid exact-object approval plus separate `01` evaluation of an existing transition.
- `REVISE`: N20 `PLAN`, N21 `ANALYZE`, or N22 `REMEDIATE` as selected by the canonical defect.
- `HOLD`: remain at `HUMAN_REVIEW` or use an existing applicable blocker/hold state.
- `STOP`: N19 `REJECTED` or C01 `CANCELLED` according to recorded rationale.

## Integrity Rules

The packet cannot approve, transition, accept risk, authorize delivery or reinterpret a Human Principal decision. Missing/stale evidence, Material unresolved contradiction, wrong object/version, invalid N/A or absent authority makes the packet not ready. The signed canonical decision record prevails over every summary.
