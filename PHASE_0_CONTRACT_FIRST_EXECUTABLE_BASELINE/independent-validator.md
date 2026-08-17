# Independent-validator specification

The validator is the independent PASS/FAIL authority for this acceptance baseline. It must be organizationally and technically separate from implementation ownership, derive requirements from certified sources without accepting constructor labels or conclusions as evidence, and use no authority-bearing implementation library.

## Black-box reconstruction

For every manifest contract, the validator must start at each commit-bound source path, SHA-256 digest, JSON Pointer and source identity. It must reconstruct the material requirement and invariant, then independently compare that derivation with the behavioral preconditions, operation, authority/currentness conditions, expected outcome, prohibited false-pass outcome and future pass condition.

The validator must invoke only the public contract interface with the mapped synthetic fixture. It must verify that the expected and prohibited outcomes are distinguishable and independently generate at least one materially equivalent adversarial input for the recorded failure classification. Constructor tests and implementation conclusions may be compared only after independent derivation.

## Procedure and authority boundary

1. Verify `main`, local `HEAD`, live `origin/main`, tracking, and 0/0 divergence equal the certified commit.
2. Verify Layers 00–19 against the certified Git tree; verify no Layer 20 and no changes to protected V1/V2/V3 artifacts.
3. Validate the manifest with the schema and executable cross-record validator. Independently resolve every source locator and reconcile exact contract, test, fixture and interface sets with no duplicates or orphans.
4. Run `cargo test --no-run`, `cargo test -- --list`, the harness tests separately, and the conformance tests with one test thread.
5. Classify a conformance failure as `EXPECTED_FAIL_NOT_IMPLEMENTED` only when the mapped fixture reached its public interface and that interface returned the explicit absence sentinel. A behavioral mismatch after implementation is `OTHER_FAIL`; a correct behavioral outcome is PASS for that contract and, during Phase 0, `UNEXPECTED_PASS`.
6. Treat compile, discovery, schema, source resolution, fixture, duplicate/orphan, panic unrelated to the explicit sentinel, or prohibition failure as `HARNESS_FAIL`.
7. Confirm there are no databases, connectors, credentials, secrets, external actions, Layer 18 execution, Layer 19 reconciliation, deployment artifacts, services, workers, queues, schedulers, webhooks, daemons, or Layer 20.

Acceptance requires complete independently reconstructed chains, zero semantic mismatches, all harness controls passing, ten meaningful expected-absence results, and zero unexpected, harness or other failures. This specification neither performs nor claims independent runtime certification, operational readiness, deployment authority, or Phase 1 completion.

