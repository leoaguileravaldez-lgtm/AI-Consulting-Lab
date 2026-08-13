# Deferred Implementation-Readiness Contracts

NORMATIVE_SOURCE: `FORMAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.1`  
DERIVATION_TYPE: deferred-runtime contract projection

| Architectural requirement | Runtime mechanism options | Later validation |
|---|---|---|
| Canonical serialization and hashing | formally specified binary/text canonical encoding and domain-separated hashes | cross-language vectors, ambiguity and collision review |
| Atomic transition acceptance | transactional store, CAS, unique constraint, serialized log | concurrency, crash and duplicate-successor tests |
| Global effect-slot uniqueness | composite unique index or linearizable state machine | cross-instance and partition races |
| Closure persistence | append-only ordered log plus independently authorized closure certificates | fork, omission, rollback and equivocation tests |
| Consensus/locking | consensus, single-writer lease or equivalent linearizable authority | partition and failover tests |
| Fencing/compare-and-effect | monotonic fence or target-native conditional operation | stale writer and target-mutation injection |
| Revocation transport | ordered revocation events and bounded propagation | loss, reorder, stale-cache and latency tests |
| Workload attestation/isolation | signed workload identity, isolated client trust domains | impersonation and cross-boundary tests |
| Observability | immutable event/transition metrics and lineage traces | missing/duplicate/reordered telemetry tests |
| Recovery | authoritative unknown/partial outcome reconciliation | crash recovery without second effect |
| Empirical security/concurrency | model-based, property, penetration and chaos testing | demonstrate conformance to all guards/exclusivity keys |

No mechanism is implemented or claimed operational in this design.
