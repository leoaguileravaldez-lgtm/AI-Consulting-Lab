# Identity, Entity, and Instance Boundaries

## Canonical identity

A client instance consumes an exact `VALIDATED_CURRENT` Layer 09 client account version. Layer 10 never creates, merges, splits, or resolves client identity. Display names, domains, addresses, brands, external CRM IDs, and aliases are non-canonical attributes.

Potential duplicates, ambiguous identity, conflicting ownership, or missing `client_id` place the instance in `IDENTITY_BLOCKED`. No fuzzy match, shared domain, parent name, or commercial urgency selects a client.

## Legal entities and relationships

Every applicable instance binds exact Layer 09 `legal_entity_id` versions. Parent, subsidiary, affiliate, fund, joint venture, successor, franchise, government unit, and portfolio-company relationships are typed `Related-Party Edge` references. A relationship never merges security domains, confidentiality, engagements, access, conflicts, evidence, or reuse rights.

Merger, acquisition, rename, redomiciliation, dissolution, conversion, ownership change, or contracting-party substitution triggers new binding versions; identity, jurisdiction, conflict, confidentiality, scope, retention, access, and reuse impact review; and blocks affected use pending canonical resolution.

## Prospect and client segregation

`PROSPECTIVE`, `QUALIFICATION_BLOCKED`, `ACTIVE`, `RESTRICTED`, `INACTIVE`, `REJECTED`, `CLOSED`, `RETENTION_HOLD`, `DISPOSITION_PENDING`, `DISPOSED_REFERENCE_ONLY`, and `QUARANTINED` are descriptive Layer 10 lifecycle states. They do not replace Layer 09 operational state or Layer 03 engagement lifecycle.

A prospect is not an active client and an opportunity is not an engagement. Prospective materials use a separate security domain and cannot enter an active client or engagement by stage change. Conversion requires canonical Layer 09 identity/entity resolution, Layer 03 engagement creation where applicable, conflict/confidentiality review, new boundary manifests, and Human Principal authorization under certified policy.

Rejected, inactive, and closed instances retain confidentiality, conflict, audit, retention, legal-hold, and reuse restrictions. Status never authorizes deletion or reuse.

