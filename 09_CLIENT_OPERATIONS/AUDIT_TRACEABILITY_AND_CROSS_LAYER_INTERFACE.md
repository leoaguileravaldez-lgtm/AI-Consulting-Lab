# Audit, Traceability, and Cross-Layer Interface

## Audit requirements

Every create, read proposal, association, correction, state proposal, classification change, merge/split request, conflict event, reuse request, authority request, denied boundary attempt, pressure event, and invalidation notice requires an audit record with actor, role, time, action, exact object/version, client and engagement boundary, purpose, basis, outcome, reason, authorization reference, and correlation reference.

Audit records are append-only design requirements. Layer 09 cannot alter or delete them. Sensitive content is minimized and credentials are prohibited. Future audit storage and access remain separately governed.

## Traceability chains

Required chains include:

- client account -> legal entity -> identity verification -> engagement association -> Layer 03 engagement;
- stakeholder -> declared role/authority -> source -> validity -> communication/interaction;
- opportunity -> stage event -> commercial metadata reference -> engagement creation decision, if any;
- preference -> scope -> application event -> affected artifact reference, without content authority;
- client-bound object -> reuse request -> Layer 08 reviews -> Human Principal authorization -> destination validation;
- change/contradiction -> impact notice -> owner acknowledgements -> revalidation or continued block;
- conflict/pressure -> escalation -> Layer 06/01 disposition reference -> residual conditions.

Every edge carries source and target identifiers and versions, relationship type, boundary, creator, time, and audit reference. Missing or mixed lineage makes the affected object not ready.

## Operational context packet

A packet sent to another layer contains packet ID/version; purpose; client/legal entity/engagement identifiers and client security domain; source Layer 09 object/version references; source `schema_invariant_status` and validation reference; classification; freshness and review deadline; contradictions; limitations; prohibited interpretations; destination owner; authorization reference where required; created-by identity and created time; and audit reference.

Only exact-version `VALIDATED_CURRENT` source records may be included. Missing, stale, contradicted, superseded, incomplete, or boundary-mismatched records block packet creation and reliance. Packets are advisory inputs. They cannot carry commands to alter canonical evidence, analysis, challenge, QA, risk, knowledge, deliverables, approval, or external action. Receiving layers independently validate use.

## Contradictions

Contradictory operational records remain distinct and linked. Layer 09 does not synthesize false consensus. A contradiction records assertions, sources, versions, scope, materiality proposal, affected dependencies, status, and owner/escalation references. Until competent resolution, the affected field is `CONTRADICTED` and Material reliance fails closed.
