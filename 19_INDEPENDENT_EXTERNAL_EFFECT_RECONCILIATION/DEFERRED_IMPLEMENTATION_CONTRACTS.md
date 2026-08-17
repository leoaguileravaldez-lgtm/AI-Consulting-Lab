# Deferred Implementation Contracts

The canonical model contains exactly 11 Category 2 obligations:

1. read-only target adapters;
2. independent observer infrastructure;
3. durable evidence and reconciliation storage;
4. target identity and generation registry;
5. canonical serialization, cryptographic integrity, and timestamping;
6. production identity, credentials, and attestation;
7. read-only query and observation orchestration;
8. reliable freshness, revocation, and time transport;
9. concurrency, crash, and partition realization;
10. telemetry and immutable audit collection; and
11. empirical security, independence, recovery, and adversarial certification.

For each, `CANONICAL_MODEL.json.deferred_implementation` records the future owner, rationale, prerequisites, required evidence, fail-closed absence behavior, and future certification condition. Absence never pretends a mechanism exists: it leaves observation unavailable, incomplete, invalid, stale, unknown, quarantined, revalidation-required, or operationally uncertified as specified.

These deferrals contain no architectural authority gap. They realize already-defined contracts and are the presumptive next phase after Layer 19 certification. They authorize no connector, credential, query, storage, service, or external action in this package.
