# Deliverable Object and Lineage

## Canonical projection

The canonical deliverable remains owned by `01_ORCHESTRATOR/RECORD_SCHEMAS_AND_AUDIT.md`. A Layer 07 artifact record is a presentation projection of that exact record, not another system of record.

Each Material artifact binds:

- presentation artifact ID and canonical deliverable ID/version;
- engagement ID, scope version, audience, purpose and materiality;
- content version and presentation version;
- exact analysis, recommendation and claim versions;
- evidence/source records from `04`;
- challenge and dissent records from `05`;
- QA and residual-risk records from `06`;
- professional-review references;
- Human Principal disposition and applicable canonical approval/release references;
- measurement references where applicable;
- confidentiality and intended-distribution metadata;
- dependency, review and audit references;
- artifact hash, creator, generation time and supersession link.

“Latest” is prohibited for Material dependencies.

## Statement lineage

Every Material statement has an internal lineage entry:

`rendered statement → exact claim/recommendation version → analysis/evidence/challenge/QA/risk/decision references`.

Factual lineage continues through the validated claim-evidence relation to the exact evidence and root source. Numerical lineage continues to the exact governed field, cell, series or output plus units, currency, basis, period, scenario and transformation. Recommendation lineage preserves challenge, dissent, residual uncertainty, reversal conditions and Human disposition.

Client-facing citation density may vary, but internal lineage must reconstruct every Material statement. A presentation artifact never becomes evidence or a replacement source of truth.

