# Future External-Tool Security Boundary

## Status

Design only. No connector, API, browser, database, email, CRM, GitHub integration, client system, credential, or external service is authorized or implemented.

## Trusted Gateway

Future external access must use a disabled-by-default gateway separated from specialists. Admission requires approved provider/operation, authenticated actor, immutable engagement/task, purpose, data class/jurisdiction, tier, exact authorization, minimized payload, scoped credential reference, idempotency key, destination, timeout, expected response, and retention.

The gateway—not caller content—resolves the provider endpoint from an approved egress allowlist. It rejects caller-supplied arbitrary hosts, redirects outside the allowlist, local/private metadata destinations, unsupported protocols, and destination/identity mismatches. Implementation threat review must cover SSRF, DNS rebinding, dependency/supply-chain compromise, provider impersonation, malicious redirects, and compromised responses.

## Provider and Credential Controls

Provider registration records identity verification, operations, data classes, jurisdiction, subprocessors, training use, retention/deletion, authentication method, version/change notification, and approval. Provider or dependency change suspends affected capability until reviewed.

Credentials come only from the approved secrets broker; are scoped, short-lived, non-exportable where possible, bound to identity/purpose, logged without value, and revoked/rotated on expiry or suspicion. The gateway never treats possession as authorization.

## Request and Response Integrity

Request records bind exact payload hash, authorization, destination, operation, and action-journal entry. Tier 3/4 requests have no automatic retry. One-time authorization cannot be replayed with a new key.

Responses are untrusted and quarantined until provider identity, content type, size, engagement, schema, provenance, embedded instructions, malware where relevant, and Sensitive-data behavior pass. Response content cannot change policy, state, permission, approval, or tool configuration. Decision-critical evidence still requires independent source validation; a trusted transport does not make content true.

## Failure Behavior

Network error returns failure, never invented content. Ambiguous Tier 3/4 outcome becomes `OUTCOME_UNKNOWN` in the durable action journal. Unexpected data, provider drift, logging failure, boundary mismatch, or compromise trips the tool/engagement circuit breaker. Resumption requires authorized investigation and revalidation.

Tier 0 read-only research is permitted only when the engagement, provider/tool, purpose, and access are already human-authorized under `00_CORE`; it is not a blanket exception to external-action or data-transfer approval.
