# Control Compliance Model

## Control inventory

Each assessment uses a versioned control manifest derived from certified requirements and client-specific authorized requirements. Each control identifies its canonical source, applicability rule, owner, operator, frequency or stage, required evidence, test method, materiality, dependencies, exception eligibility, and failure behavior.

`06` does not create policy. A locally described test is subordinate to the cited canonical requirement. Conflicting, missing, or indeterminate requirements fail closed.

## Control testing

For every applicable control, QA distinguishes:

- design adequacy: whether the control as specified addresses its purpose;
- implementation status: whether it exists for the exact scope;
- operating effectiveness: whether it operated as required;
- evidence sufficiency: whether operation is supported by current, attributable records;
- exception status: whether a valid exact exception applies;
- residual exposure after operation.

Control results are `SATISFIED`, `PARTIALLY_SATISFIED`, `FAILED`, `NOT_TESTED`, `NOT_APPLICABLE`, or `UNKNOWN`. `NOT_APPLICABLE` requires a cited applicability rule, rationale, assessor, exact scope/version, and independent review where Material. Default, silence, template omission, or convenience cannot establish it.

## Required coverage

The manifest covers applicable stage completion, evidence controls, challenge controls, blockers, dissent, uncertainty, assumptions, contradictions, change control, traceability, reproducibility, client segregation, conflicts, commercial pressure, scope creep, jurisdiction, professional review, audit continuity, release guards, and governance regression.

Checklist completion is not operating effectiveness. Missing control evidence is `UNKNOWN` or `FAILED`, never inferred success.

