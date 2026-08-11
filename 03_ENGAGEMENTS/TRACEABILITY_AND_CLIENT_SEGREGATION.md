# Traceability and Client Segregation

## Decision Lineage

Every Material conclusion must support reconstruction through:

`Engagement -> Problem Contract -> Workstream -> Canonical Task -> Question -> Claim/Assumption -> Evidence -> Analysis -> Specialist Output -> Challenge/Validation/Risk -> Contradiction -> Synthesis -> Human Decision -> Deliverable/Delivery -> KPI/Outcome/Benefit -> Closure`.

Each edge records source and target IDs, versions/hashes, relationship type, creator, time, engagement boundary and audit reference. Missing lineage makes the affected item not ready. Facts, assumptions, inferences, scenarios and recommendations retain their certified `02_SPECIALISTS` labels across handoffs.

The exact Human Principal decision record includes authority, category, object, version/hash, conditions, scope, time, expiry/revocation status and rationale/reference. A narrative cannot replace or reinterpret it.

## Structural Segregation

Client-specific content must be contained by immutable engagement ID and client/security domain across storage namespace, identity/access group, encryption/key boundary where applicable, evidence store, assumptions, models, prompts/context manifests, workstreams, audit, deliverables and KPI/outcome records.

Every object and link carries engagement ID, client/security domain and data classification. Cross-boundary reads default deny and require canonical exact-purpose authorization, conflict review, permission, provenance and audit. A link cannot be used to bypass physical access controls.

Reusable methods, blank schemas and sanitized generic knowledge may be shared only after de-identification and authorization under the future Knowledge boundary. Client evidence, conclusions, assumptions, model inputs, outputs, deliverables and measurements may not be treated as reusable facts.

## Negative Tests

- Different engagement ID or security domain: deny and audit.
- Same apparent industry/problem but different client: no evidence, assumption or conclusion inheritance.
- Copied ID without valid authorization/context manifest: deny as mismatch.
- Cross-client semantic retrieval or prompt contamination: suppress result, open security/incident handling as applicable.
- Deliverable or KPI query returning an unrelated engagement object: fail the boundary check and block use.
- Authorized reusable method containing client-specific residue: reject sanitation and sharing.

## Summary Integrity

Reports are computed views over cited source versions. They cannot edit sources. Stale, unavailable or conflicting source fields display as such; they are never silently filled from another engagement or an analyst's memory.
