# Phase 1 global uniqueness and concurrency empirical slice

This isolated workspace implements only `BC_GLOBAL_UNIQUENESS_CONCURRENCY`. It consumes the frozen Phase 0 contract and the certified transactional-persistence predecessor through explicit crate interfaces.

The bounded local evidence uses two independent `/usr/bin/sqlite3` processes, separate connections and overlapping `BEGIN DEFERRED` snapshots synchronized by explicit CLI readiness markers. Database primary/unique constraints and atomic compare-and-swap predicates own uniqueness, stale-writer and replay rejection.

Claims are limited to the tested local disposable SQLite topology. This is implementation evidence, not PostgreSQL behavior, distributed consensus, partition tolerance, production durability, recovery, revocation, isolation, audit certification, deployment authority, or independent certification. Five downstream contracts remain deferred.

