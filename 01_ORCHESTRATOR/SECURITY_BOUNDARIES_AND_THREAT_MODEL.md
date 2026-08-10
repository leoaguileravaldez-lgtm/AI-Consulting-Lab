# Security, Engagement Isolation, and Threat Architecture

## Canonical Scope

This document is the canonical design for Orchestrator trust boundaries, client-engagement isolation, contamination prevention/detection, and security threats. It specifies future requirements without claiming current technical enforcement.

## Engagement Security Object

The trusted intake authority may issue a provisional immutable engagement ID with status `PENDING_AUTHORIZATION` after a human authorizes intake/conflict-check preparation. The provisional object contains only the minimum authorized identity and conflict-check metadata; it grants no specialist access and cannot hold analytical client data. The canonical state machine activates it only at `ENGAGEMENT_AUTHORIZATION` after conflict clearance and full scope/data/tool/tier approval. This separates identifier creation from engagement activation and avoids circular authorization.

An engagement ID is never generated, selected, or changed by a specialist. A correction creates a new engagement identity and controlled migration; it never mutates the original ID.

The engagement security object binds:

- immutable engagement ID and client identity reference;
- authorized purpose and scope;
- data classes and jurisdictions;
- permitted identities, roles, tiers, systems, and tool classes;
- isolated namespaces for context, evidence, working artifacts, decisions, approvals, audit, cache, retrieval, backups, and retention;
- effective and closure dates;
- authorization and conflict-clearance records.

Missing, unknown, ambiguous, conflicting, or multiple engagement identities always produce `SECURITY_HOLD`. Context inheritance is permitted only from a verified parent task within the same engagement; otherwise explicit engagement binding is required.

## Isolation Enforcement Contract

Every access decision must evaluate actor identity, engagement ID, task ID, purpose, role, tier, data class, requested object namespace, and authorization version. All must match. Object identifiers alone are insufficient.

Future implementation must provide:

- deny-by-default authorization at every data and tool boundary;
- engagement-scoped storage and retrieval filters that callers cannot override;
- engagement-scoped encryption/access boundaries where technically appropriate;
- no global client-content memory, shared embeddings, or cross-engagement cache;
- output tagging and downstream propagation of engagement identity;
- separate audit sequence and retention policy per engagement;
- closure revocation and verified disposition.

No “safe inheritance” is presumed across sessions, actors, tools, or parent IDs that have not been verified.

## Cross-Client Prevention Controls

- Reject any input containing object references from another engagement.
- Minimize handoff context and use references instead of raw content.
- Prevent shared templates from containing client-derived Confidential/Restricted material.
- Prohibit cross-engagement retrieval, comparison, summarization, model training/fine-tuning, and context reuse without written client authorization plus Human Principal approval.
- Bind exports, citations, deliverables, and future tool requests to one engagement.
- Run a pre-synthesis and pre-release contamination scan against object lineage and client identifiers.
- Quarantine any artifact with unknown or mixed lineage.

## Detection and Assurance

Isolation must be tested before implementation approval and after Material changes through positive authorization tests and negative tests attempting:

- cross-engagement object lookup;
- forged or missing engagement IDs;
- cache/retrieval leakage;
- child-task rebinding;
- log/audit leakage;
- backup/restore cross-contamination;
- shared-template contamination;
- inference through error messages or metadata.

Any unexpected access is a Critical incident. Passing functional tests without negative isolation tests is insufficient.

## Trusted Security Boundaries

Future implementation must separate Human Principal approval, identity/permission authority, workflow control, specialist contexts, assurance contexts, engagement stores, approval store, audit store, registry administration, secrets broker, and external-tool gateway. No specialist may write identity, permission, approval, registry, or prior audit data.

## Threat Controls

| Threat | Required control | Failure state |
|---|---|---|
| Prompt injection | Instruction/data separation, provenance/taint labels, immutable control-plane decisions | `SECURITY_HOLD` |
| Privilege escalation | Human-controlled capability issuance, least privilege, non-forgeable authorization reference | `BLOCKED_PERMISSION` |
| Approval spoof/replay | Trusted approval writer, exact hash/scope, nonce, expiry, revocation | `BLOCKED_AUTHORIZATION` |
| Cross-client leakage | Isolation contract and negative tests | `SECURITY_HOLD` |
| Evidence laundering | Independent retrieval proof and claim mapping | `BLOCKED_EVIDENCE` |
| Audit tampering/omission | Privileged append authority, sequence verification, independent audit check | `INCIDENT_HOLD` |
| Registry poisoning | Human-controlled versioned changes and active-work recheck | `INCIDENT_HOLD` |
| Model/tool drift | Version compatibility and revalidation | `RECOVERY_REQUIRED` |
| Duplicate effect | Durable action journal and zero automatic Tier 3/4 retry | `INCIDENT_HOLD` |
| Sensitive logging | Structured minimization, redaction verification, secret scanning | `SECURITY_HOLD` |
| External compromise | Provider identity, egress allowlist, request/response validation, quarantine | `SECURITY_HOLD` |
| Orchestrator compromise | Circuit breaker, independent audit/approval planes, safe manual mode | `INCIDENT_HOLD` |

## Credential Controls

Credentials must remain outside repository, prompts, artifacts, audit, and logs. A future approved secrets broker must issue scoped, short-lived credentials; bind them to identity/purpose where supported; prevent specialist retrieval of raw secrets where possible; log use without values; rotate/revoke on expiration or suspicion; and test redaction. Possession never establishes authority.

## Safe State

The security safe state denies new delegation, synthesis, approval consumption, delivery, tool use, and cross-boundary access while preserving protected evidence and allowing minimum authorized containment. Recovery requires incident disposition, boundary verification, audit reconciliation, and Human Principal authorization where required by `00_CORE`.
