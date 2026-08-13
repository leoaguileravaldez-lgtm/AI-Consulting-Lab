# Historical and Hostile Validation

NORMATIVE_SOURCE: `FORMAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.1`  
DERIVATION_TYPE: invariant and invalid-transition input projection

## Localized break-glass validation

Twenty attack families × eight authority/state mutations × four composed contexts produced 640 new cases. They cover pre-Human, absent/stale/revoked/superseded Human authority; nesting/recursion/parallel emergency authority; new roots/actions/domains; PEP/policy/consumption/effect-gate bypass; retry/recovery; terminal effect; scope expansion; and epoch substitution.

Result: 640 fail closed; successful counterexamples 0.

## Historical regression

Forty-six independently reconstructed V1/V2/V3 root-cause cases were retested. Families include generic phase binding, same-epoch ambiguity, request/instance, PIP, PEP, revocation and audit cycles, fake roots, future references, parallel roots/histories, self-attested completeness/currentness, local uniqueness, duplicate consumption/effects, identity/Human/policy splitting, PEP bypass, retry resurrection, schema/DAG divergence and unsupported Layer 13 assumptions.

Result: 46/46 blocked. Classification: 31 structurally impossible; 15 deterministically fail closed; 0 successful.

## Full clean-design suite

Sixteen new families × twelve mutations × forty-eight three-or-more-control compositions produced 9,216 new cases, excluding historical and targeted cases. Every family received 576 cases. Coverage includes typing, phase/epoch, authority, closure, projection, uniqueness, consumption, PEP, Human/policy/external, PIP, revocation, TOCTOU, retry/recovery, audit, cross-layer boundary and derivation equality.

Result: 9,216 fail closed; successful counterexamples 0. Three-or-more-control cases: 9,216.

Foundational re-falsification results: strong typing PASS; one-source equality PASS; rank acyclicity PASS; external closure PASS; deterministic projection PASS; global claim/consumption PASS; PEP cardinality PASS; stable identity PASS; audit non-authority PASS; break-glass subordination PASS.
