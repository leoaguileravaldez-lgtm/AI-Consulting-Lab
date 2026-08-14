# Deferred Implementation Contracts

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: Category 2 implementation-allocation projection

The normative model allocates eleven implementation families. Each has a future owner class, prerequisites, evidence and future certification condition. Their absence is non-blocking for architecture-only closure but cannot be represented as operational capability, readiness or certification.

1. Serialization and cryptographic realization.
2. Transactional persistence and authoritative logs.
3. Consensus, locking, CAS, fencing and physical uniqueness.
4. Revocation transport and reliable time.
5. Workload isolation and runtime attestation.
6. Backup, restore, restart, failover and disaster recovery.
7. Telemetry and immutable audit realization.
8. Production identity, authorization and secrets infrastructure.
9. Production services, networks, compute and data infrastructure.
10. Deployment and operational assurance governance.
11. External-action execution and effect/outcome reconciliation.

Families 10 and 11 are reserved for separately discovered and authorized future responsibilities. No layer number is assigned. Layer 16 specifies only the predecessor and evidence obligations they must ultimately satisfy.

No deferral is a waiver. Before the relevant capability is used, its exact realization candidate must be independently tested against the applicable Layer 16 contract and all retained predecessor authority. Missing, failed or unknown evidence keeps the capability ineligible and fail-closed.
