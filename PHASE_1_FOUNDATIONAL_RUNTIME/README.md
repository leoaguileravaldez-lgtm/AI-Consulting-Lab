# Phase 1 foundational runtime — first slice

This non-layer workspace supplies implementation evidence for the first coherent pure dependency slice: canonical representation, typed identity/version/provenance binding, and domain-separated integrity hashing. It imports the frozen Phase 0 contract crate by read-only path and reconstructs bindings from the frozen manifest and fixture set.

Canonical serialization is a bounded RFC 8949 deterministic CBOR subset: null, booleans, integers, UTF-8 strings, arrays, and maps are supported; floats are rejected; map keys use encoded-length then bytewise ordering. SHA-256 hashes have explicit algorithm, semantic domain, schema-version, and payload-length boundaries. No keys, signatures, or credentials exist.

UUIDv7-compatible validation is available without generation. UUIDv7-compatible values expose approximate creation time in their leading 48 bits; they create no authority.

All state is pure and process-local. This slice makes no claim of durable persistence, atomic database commitment, cross-process concurrency, crash recovery, network-partition behavior, production readiness, deployment authority, or independent certification. PostgreSQL is not used and is not required for this slice. The seven dependent contracts remain explicitly not yet implemented.

