# Re-Challenge and Versioning

## Immutable Coverage

A challenge applies only to its exact target object/version and evidence/assumption/model manifest. The original challenge always remains linked to that version.

A Material change to evidence, assumptions, model/method, recommendation, option ranking, scope, jurisdiction, implementation condition, confidence or decision-reversal threshold triggers dependency review. Prior coverage becomes `SUPERSEDED` when no longer applicable or `RECHALLENGE_REQUIRED` when the revised object needs new challenge. These are record statuses, not canonical task states.

No old challenge may validate or clear a revised recommendation by similarity.

## Bounded Cycles

- Cycle 1: initial independent challenge.
- Cycle 2: challenge of a Material producer response or revision.
- Further cycle: only for a genuinely new Material object, evidence basis or decision condition.

Cycle count never resolves dissent. After two cycles without a new Material object, persistent disagreement is sealed as `UNRESOLVED_DISSENT` and routed through `01` to the Human Principal decision process. Silence, timeout, repeated iteration or majority agreement cannot imply resolution.

Human Principal may decide among disclosed alternatives or request additional work. The decision cannot rewrite sealed records, validate unsupported evidence, waive required assurance or call unresolved analytical conflict resolved.

## Change Propagation

New or revoked evidence follows `04` impact propagation. `01` decides canonical rework/revalidation/challenge state. `03` reflects readiness. `05` cannot reopen tasks, transition state or authorize re-challenge itself.
