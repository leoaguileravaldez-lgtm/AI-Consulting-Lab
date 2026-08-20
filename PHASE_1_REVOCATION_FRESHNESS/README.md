# Phase 1 revocation and freshness empirical slice

This isolated workspace implements only `BC_REVOCATION_FRESHNESS`. It consumes the frozen Phase 0 contract and certified transactional-persistence and global-uniqueness/concurrency predecessors through explicit crate dependencies.

The authoritative SQLite row distinguishes `CURRENT`, `REVOKED`, and `SUPERSEDED` at a monotonic generation. Revocation changes authority and appends its record in one database transaction. A derived projection remains non-authoritative: stale, unknown, revoked, or superseded currentness fails closed and cannot restore authority.

Claims are limited to local disposable SQLite authoritative-currentness behavior. This is implementation evidence, not distributed propagation, time-bound cache coherence, recovery, runtime isolation, audit certification, deployment authority, or independent certification. Four downstream contracts remain deferred.

