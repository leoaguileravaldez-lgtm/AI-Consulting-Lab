# Engagement Lifecycle and Classification

## Derived Lifecycle

Lifecycle phases are non-authoritative views deterministically derived from canonical `01_ORCHESTRATOR` Task states. They are neither states nor transition permissions.

| Derived phase | Canonical state basis |
|---|---|
| `INITIATION` | `DRAFT_INTAKE`, `CONFLICT_CHECK`, `ENGAGEMENT_AUTHORIZATION` |
| `DEFINITION` | `CLASSIFY`, `PLAN`, `PLAN_REVIEW`, `DELEGATE` |
| `DISCOVERY` | authorized research/evidence child tasks, represented by their canonical states |
| `ANALYSIS` | `ANALYZE` |
| `ASSURANCE` | `CHALLENGE`, `REMEDIATE`, `VALIDATE_EVIDENCE`, `VALIDATE_ANALYSIS`, `RISK_REVIEW`; blocked/failure states derive from their canonical provenance rule below |
| `DECISION` | `SYNTHESIZE`, `SYNTHESIS_INTEGRITY_REVIEW`, `RELEASE_CHECK`, `HUMAN_REVIEW` |
| `DELIVERY` | `DELIVERY_AUTHORIZATION`, `DELIVER`, `DELIVERY_VERIFICATION` |
| `OUTCOME_AND_CLOSURE` | linked measurement follow-up tasks and `RETAIN`, `ARCHIVE`, `CLOSE_REVIEW`, `CLOSED` |

For multiple tasks, the engagement phase view shows every active phase plus the earliest phase containing a Material/Critical blocker. It must not falsely collapse parallel workstreams into one progress state. Terminal tasks remain visible. Unknown or unmapped states yield `DERIVATION_ERROR`, not a guessed phase.

### Deterministic Provenance Rule

Each task phase is derived from the task's current canonical state plus its canonical task purpose and transition/audit provenance. For any `BLOCKED_*`, failure, hold, invalidated, recovery, or other non-progress state, the phase is the phase of the canonical state from which that condition was entered. If that origin cannot be established uniquely, the result is `DERIVATION_ERROR` and the engagement is `NOT_READY`. A measurement, measurement-follow-up, benefit-realization, revalidation, required-evidence, gate-dependency, Human Principal-decision, or delivery task retains its explicit task-purpose flag and remains visible in its applicable phase until its canonical completion requirement is satisfied.

The engagement phase is the earliest derived phase containing any canonically relevant nonterminal, blocked, unresolved, invalidated, revalidation-required, or required-follow-up task. It cannot derive forward, appear complete, or become closure-ready while any such task exists. A later completed task cannot mask an earlier unresolved task. When any derived view conflicts with a canonical task state or provenance, the canonical task record wins, the conflict is exposed as `DERIVATION_ERROR`, and affected progression fails closed.

## Progression Derivation

Readiness equals the conjunction of applicable canonical task prerequisites, fresh required references, satisfied dependencies, resolved or disclosed contradictions, required exact approvals, and absence of every canonically relevant unresolved task described above. Only `01` executes a transition. `03` may display `NOT_READY`, `READY_FOR_01_EVALUATION`, or `NOT_APPLICABLE`; none authorizes progression, approval, delivery authorization, terminal completion, or closure.

## Engagement Classes

Use only `ROUTINE`, `MATERIAL`, and `CRITICAL`, derived from the controlling `01` materiality/risk classification. `03` cannot lower a class.

| Dimension | Routine | Material | Critical |
|---|---|---|---|
| Intake | Minimum complete | Full exposure and uncertainty assessment | Full assessment plus critical dependencies and contingency |
| Team | Minimum sufficient | Required material-domain and assurance roles | Qualified multidisciplinary team and reserved independent capacity |
| Evidence | Fit for limited reliance | Decision-critical claims independently validated | Strongest feasible provenance, corroboration and sensitivity |
| Challenge / Risk-QA | Proportional cross-review where permitted | Independent challenge and applicable Risk/QA | Mandatory structural independence and enhanced Risk/QA |
| Human gates | A and F/G as applicable | All applicable Material gates | All applicable gates; no compression by deadline |
| Measurement | If outcome-producing | Mandatory if outcome-producing | Mandatory if outcome-producing, enhanced attribution review |
| Audit / closure | Proportionate | Complete material trace | Complete trace plus unresolved-risk and contingency record |

Aggregation, changed exposure, jurisdiction, uncertainty, external use or irreversibility may require upward reclassification through `01`. Gate count alone never defines class.

## NOT_APPLICABLE

A gate or measurement requirement may be `NOT_APPLICABLE` only when a rule names the inapplicability reason, cites the engagement profile and source facts, identifies the accountable reviewer, records impact, and receives any approval required by `01`. It is forbidden where the activity or decision actually exists, where uncertainty prevents the determination, or as a means to avoid assurance. A changed fact mechanically invalidates the designation.
