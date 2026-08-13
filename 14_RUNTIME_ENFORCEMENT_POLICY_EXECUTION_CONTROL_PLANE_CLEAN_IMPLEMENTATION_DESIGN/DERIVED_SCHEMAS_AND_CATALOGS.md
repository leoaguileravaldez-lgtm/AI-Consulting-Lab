# Derived Schemas and Catalogs

NORMATIVE_SOURCE: `FORMAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.1`  
DERIVATION_TYPE: exact catalog/schema projection

The model projects exactly:

- 5 formal primitives;
- 13 non-interchangeable identifier types;
- 12 authority classes and 2 discriminated authority unions;
- 32 typed event schemas;
- 5 stage-specific closure-witness schemas;
- 5 stage-specific projection schemas;
- 42 transition-rule schemas;
- 9 global invariants;
- 8 deferred-runtime contracts.

Material schema objects are the 42 event/closure/projection node classes. Each event schema is the TypedEvent primitive plus its exact declaration. Each closure schema is the ClosureWitness primitive plus its exact stage and input-event set. Each projection schema is the StateProjection primitive plus its exact closure and closed output enum. Each transition schema is the TransitionRule primitive plus one normative rule.

Authority-critical optionality is discriminated. Human approval versus certified non-applicability, external approval versus certified non-applicability, active break-glass versus not invoked, and initial/retry/recovery attempts are different event types or explicit union members. Generic `COMMITTED`, `CLOSED`, `CURRENT`, `VALIDATED`, `PROJECTED`, or `RESOLVED` is not a valid standalone event type.

Canonical business identity uses the exact tuple declared by `CanonicalBusinessActionId`. Canonical serialization and hashing are future implementation contracts; no production hash is claimed.
