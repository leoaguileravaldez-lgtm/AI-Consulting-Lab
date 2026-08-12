# Authentication Context and Session Assurance

Layer 13 represents but does not perform authentication. Assurance states are `UNVERIFIED`, `ASSERTED`, `VERIFIED`, `STRONG_VERIFIED`, `EXPIRED`, `REVOKED`, and `UNKNOWN`.

An Authentication Assurance Reference binds exact identity/principal/effective actor, method reference, issuing authority, verification and assessment times, scope, client/entity/engagement/jurisdiction, freshness, expiry, revocation, limitations, provenance, and audit. Technology names do not prove implementation or strength.

A Session/Context Reference is metadata about a separately governed context: exact identity/effective actor, assurance reference, purpose, object/action scope, boundaries, issued/observed/expiry times, revocation, originating authority, context integrity reference, and limitations. It contains no cookie, token, secret, certificate, key, or credential.

Authentication or session presence never grants role, permission, authority, approval, access, or Human Principal status. Unknown, mismatched, stale, expired, revoked, copied, replayed, cross-boundary, or context-changed assurance denies Material authorization.
