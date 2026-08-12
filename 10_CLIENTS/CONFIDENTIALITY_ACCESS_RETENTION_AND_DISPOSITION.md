# Confidentiality, Access, Retention, and Disposition

## Confidentiality

Every object references the applicable Layer 00 classification and exact Layer 09 confidentiality profile/version. The most restrictive known legal, contractual, regulatory, professional, client, entity, engagement, ownership, and purpose restriction governs.

Unknown, missing, malformed, conflicting, or stale classification/confidentiality is treated as the highest plausible restriction and blocks affected use. A lower-classification copy, de-identification claim, client request, lifecycle closure, new jurisdiction, or system default cannot downgrade confidentiality.

Downgrade requires exact object/version and scope, authoritative basis, legal/contractual/confidentiality review, impact analysis, Human Principal decision where allowed, effective time, and audit. It is never retroactive and cannot cure prior disclosure.

## Access architecture

Layer 10 records required access attributes and references only: named subject, role, client, entity, engagement, purpose, object/version, classification, confidentiality, action, time window, authorization, and audit. It grants no identity, role, permission, access, encryption, key, credential, storage, or execution capability.

Technical visibility, CRM ownership, prior access, cached access, shared employment, client instruction, or possession is not authority. Future enforcement defaults deny, least privilege, purpose/time limitation, separation of duties, and immediate invalidation on change.

## Retention states

States are `ACTIVE_RETENTION`, `LEGAL_HOLD`, `CONTRACTUAL_HOLD`, `REGULATORY_HOLD`, `CLOSURE_RETENTION`, `REVIEW_DUE`, `DISPOSITION_ELIGIBILITY_REVIEW`, `DISPOSITION_BLOCKED`, `DISPOSITION_AUTHORIZED_NOT_EXECUTED`, and `DISPOSED_REFERENCE_ONLY`.

Retention state does not create deletion authority. Missing rule, conflicting requirements, unknown jurisdiction, unresolved dispute, legal hold, open incident, audit dependency, reuse dependency, or incomplete lineage blocks disposition.

## Disposition

A Disposition Request inventories exact source and all known copies, derivatives, exports, caches, indexes, embeddings, backups, prompts, contexts, audit dependencies, and holds. It records authoritative rules, qualified reviews, reversibility, required approval, proposed executor, verification, and residual limitations.

Layer 10 cannot delete, purge, anonymize, destroy, revoke, or execute disposition. Destructive action remains governed by Layer 00/01 and separate runtime authority. `DISPOSED_REFERENCE_ONLY` is a historical metadata state requiring execution and verification references; it is not proof created by Layer 10.

