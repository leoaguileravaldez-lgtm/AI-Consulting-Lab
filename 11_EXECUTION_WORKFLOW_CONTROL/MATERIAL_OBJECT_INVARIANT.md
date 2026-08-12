# Material Object Invariant

Every Material object explicitly carries immutable ID, `schema_version`, `record_version`, client/legal-entity/engagement/security boundaries where applicable, purpose, classification, confidentiality, work-item reference, lifecycle/status, exact predecessor version, freshness, review deadline, provenance, dependencies, blockers, limitations, responsible/assigned roles, creator/time, canonical authority references, and append-only audit lineage.

Every Material Work Item additionally carries its canonical exhaustive dependency-set status, declared count, exact Dependency Record ID/version collection, validation reference, and validation time. Absence is never equivalent to an empty set. A valid empty set is explicit, zero-count, and independently validated. Downstream objects cannot complete or substitute for the Work Item declaration.

`NOT_APPLICABLE` requires reason and validation reference. It is invalid for ID/version, purpose, classification, status, provenance, limitations, creator, authority, or audit; and invalid for client/engagement boundaries on client-bound work.

Invariant outcomes are `VALIDATED_CURRENT`, `INCOMPLETE_BLOCKED`, `STALE_BLOCKED`, `DEPENDENCY_BLOCKED`, `AUTHORITY_BLOCKED`, `BOUNDARY_BLOCKED`, `CONFLICT_BLOCKED`, `REVALIDATION_BLOCKED`, and `SUPERSEDED_BLOCKED`.

Only `VALIDATED_CURRENT` may support an operational proposal. Any missing, malformed, stale, superseded, ambiguous, cross-boundary, contradictory, unauthorized, or unvalidated Material field makes the object non-current, non-authoritative, transition-ineligible, execution-ineligible, completion-ineligible, and quarantined.

For a Work Item, `VALIDATED_CURRENT` also requires a dependency set that is `VALIDATED_COMPLETE` or `VALIDATED_EMPTY`, count-consistent, uniquely enumerated, reverse-bound to the exact Work Item/version, non-circular, and deterministically reconciled through each Dependency Record to its exact source object/version and canonical authority. A summary state, prose description, inference, or downstream reference cannot meet this requirement.

No elapsed time, inference, model output, client urgency, commercial priority, retry count, recurrence, agent assertion, consensus, majority, downstream expectation, prior approval, technical capability, or aggregate score can compensate.

Invariant validation proves record completeness only. It cannot establish evidence, analysis, professional approval, risk acceptance, QA passage, release, client authorization, Human approval, access, or external-action authority.
