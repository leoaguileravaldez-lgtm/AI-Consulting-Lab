# Risk and Quality Assurance Architecture

## Purpose and canonical subordination

`06_RISK_QA` operationalizes independent process/control assurance and residual-risk assessment. It asks whether governed work was performed correctly, required controls are satisfied, and residual risks are visible for canonical evaluation. It does not determine whether an analytical recommendation is correct.

This package is subordinate to `00_CORE` and to the canonical `RISK_REVIEW`, `RELEASE_CHECK`, blocker, exception, audit, role, readiness, and state controls in `01_ORCHESTRATOR`. `01_ORCHESTRATOR/STATE_MACHINE.md` remains the sole lifecycle specification. Conflict, missing authority, or ambiguous mapping makes the affected assessment `NOT_ASSESSABLE` and requires canonical escalation; `06` may not resolve the conflict.

## Two-dimensional assurance

Analytical validity belongs to `02_SPECIALISTS`, evidence validity to `04_RESEARCH_EVIDENCE`, and falsification/dissent to `05_INDEPENDENT_CHALLENGE`. Control/process assurance belongs to `06`. Neither dimension implies the other. A QA-ready result never establishes analytical correctness, evidence sufficiency beyond `04`, professional correctness, risk acceptance, release authorization, or canonical readiness. Strong analysis never cures failed process controls.

## Permitted activity

`06` may inspect exact-version records, test control design and operation, document findings, assess residual risk using canonical fields, verify remediation evidence independently, detect recurrence, verify that required professional-review evidence exists, and publish advisory impact notices to `01`.

It may not perform primary analysis; modify evidence, challenge, dissent, analysis, approval, exception, audit, or canonical records; create a blocker or state; accept risk; approve or reject a recommendation; authorize delivery, publication, execution, or closure; communicate externally; or act as the Human Principal.

## Interfaces and precedence

- `00_CORE` owns policy, materiality, Criticality, approval, exception, security, and professional boundaries.
- `01_ORCHESTRATOR` owns assignments, routing, state, blockers, readiness, transitions, risk-review completion, release checks, approvals, exceptions, audit, and external-action authorization.
- `02_SPECIALISTS` owns primary specialist analysis and governed computation.
- `03_ENGAGEMENTS` references QA records and reflects canonical readiness.
- `04_RESEARCH_EVIDENCE` owns evidence identity, validation, quality, provenance, lineage, freshness, applicability, corroboration, and contradiction.
- `05_INDEPENDENT_CHALLENGE` owns falsification, fragility analysis, alternative hypotheses, and dissent.
- Future professional, deliverable, connector, and automation systems remain separate and separately authorized.

## Fail-closed invariant

An unresolved Material/Critical failure, missing required control, unresolved blocker or dissent, evidence-control failure, required professional review, invalid exception, uncertain authority, cross-client contamination, incomplete audit, or failed independence test prevents clean QA readiness. No aggregate or preference can compensate.

