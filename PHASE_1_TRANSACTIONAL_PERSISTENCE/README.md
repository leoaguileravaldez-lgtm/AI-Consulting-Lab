# Phase 1 transactional persistence empirical slice

This isolated workspace implements only `BC_TRANSACTIONAL_PERSISTENCE` against the frozen Phase 0 contract. It uses the installed SQLite CLI as disposable, file-backed empirical infrastructure behind a replaceable persistence boundary.

The database owns the atomic boundary for authoritative object, audit-event, and provenance inserts. Integration tests use fresh private temporary databases, synthetic data, WAL mode, `synchronous=FULL`, foreign keys, constraints, and append-only triggers. Every database is destroyed after its test.

This workspace claims only local SQLite-backed evidence for the frozen atomic-rollback behavior. It does not claim PostgreSQL behavior, production or distributed durability, concurrency correctness, crash recovery, audit-platform certification, deployment readiness, or independent certification. The six downstream contracts remain explicitly not yet implemented.

