# Deferred Implementation Contracts

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: Category 2 future-owner projection

The normative model allocates eleven implementation families. Every allocation includes future owner, rationale, prerequisite, required evidence, failure behavior and future certification condition:

1. External-action adapters and connectors.
2. Durable authoritative attempt journal and persistence.
3. Global uniqueness, locking, fencing and target-native idempotency.
4. Queues, schedulers, workers and leases.
5. Production identity, credentials, secrets and attestation.
6. Cryptographic serialization, signing and timestamping.
7. Revocation and reliable-time transport.
8. Execution telemetry and evidence collectors.
9. Crash recovery, containment and incident mechanisms.
10. Empirical security, concurrency and adversarial testing.
11. Effect/Outcome Reconciliation and Closure.

Families 1–10 are physical realizations or empirical certification obligations. Their absence is non-blocking for architecture-only closure but prohibits any operational execution claim.

Family 11 is not a Layer 18 implementation component. It is reserved for a future institutionally separate authority requiring its own discovery, charter, falsification, independent certification and Human Principal authorization. Until it exists and acts, every dispatched attempt remains an unresolved-effect hold; Layer 18 may assert neither effect, no effect, outcome, retry eligibility nor lifecycle closure.

No deferral is a waiver. Exact details remain normative in `deferred_implementation`.

