# Conclusion Binding Root-Cause Review

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.2`  
DERIVATION_TYPE: localized identity/binding correction and regression evidence

## Counterexample reconstruction

All four failures shared one cause: model 1.0.1 required a Conclusion type at transitions but did not preserve its exact identity/version in independent validation or recommendation.

1. **Cross-engagement laundering.** Engagement `EA`, Workstream `WA`, Mandate `MA`, authority domain `ADA`, current Conclusion `CA-v1`, Findings `FA`, Claims `CLA`, Evidence `EVA`, Analysis `AA`, and Challenge/validation `CHA/XVA` were valid. Engagement `EB` had `WB/MB/ADB`, Recommendation `RB`, decision and delivery target `DB`. Sequence `EA evidence → FA → CA-v1`; `EB synthesis + validation → RB → DB` could use `CA-v1` as the type-level Conclusion source because `RB` stored no exact Conclusion reference.
2. **Missing typed identity.** In one `E1/W1/M1/AD1`, two locally valid Conclusion records could reuse an untyped `conclusion_id=CX` while carrying different finding/claim/evidence/analysis lineage. A validation and Recommendation bound only synthesis `S1`, so identity collision was not detectable.
3. **Unrelated substitution.** In `E1/W1/M1/AD1`, current Conclusions `C1-v1` and `C2-v1` used different `F1/CL1/EV1/A1` and `F2/CL2/EV2/A2` lineages under synthesis `S1`. Cross-validation and Challenge were valid. Validation `V1` and Recommendation `R1` named `S1` but not the chosen Conclusion, allowing arbitrary substitution before delivery.
4. **Stale substitution.** `C1-v1` was valid, then changed evidence/finding state produced `C1-v2`, making `v1` stale/superseded. Validation `V1`, Recommendation `R1`, and delivery `D1` could still consume `v1` because no exact Conclusion version or freshness basis was recorded.

Each history was valid under 1.0.1 and could reach an invalid governed delivery without a deferred runtime failure. Confirmed Category 1: 4; reclassified: 0.

## Common root cause and correction

The common root cause was incomplete exact predecessor identity binding. Model 1.0.2 distinguishes:

- identity: `ConclusionId` and `ConclusionUseDomainId`;
- provenance: `FindingSetIdentity`, `ClaimLineageIdentity`, evidence and analysis lineage;
- scope: Engagement, Mandate, authority domain, workstream scope, and use domain;
- freshness/state: lifecycle generation, `CURRENT/SUPERSEDED/STALE/REVOKED`, freshness basis, and typed root or exact predecessor Conclusion;
- authority: analytical production remains separate from independent validation and recommendation authority.

Independent validation and Recommendation now store the same exact Conclusion ID/version set, freshness basis, Engagement, Mandate, authority domain, use domain, and synthesis. Guards require exact equality and current predecessor state. Changed lineage stales the prior generation; revocation marks it revoked; substitution denies.

## Structural and adversarial results

- normative truth sources: 1
- manual synchronization sources: 0
- Material objects / graph nodes: 23
- transition rules: 35
- graph edges: 51
- cycles, self-loops, future references, unresolved mandatory nodes, rank violations: 0
- ambiguous authority edges: 0
- ambiguous protected predecessor bindings: 0
- new targeted attacks: 1,152
- fail closed: 1,152
- successful structural counterexamples: 0
- original counterexamples blocked: 4 of 4
- historical Conclusion/predecessor families retested: 18
- historical still successful: 0
- invariants A–P: PASS
- Category 1: 0
- Category 2: 8 deferred implementation families
- Category 3: 1 optional generated-schema hardening item
- Category 4: 1 non-material reconciliation wording item

Layers 00–14 are unchanged. Layer 16 is absent. No runtime, executable, credential, connector, worker, scheduler, queue, webhook, daemon, external-action, persistence, CAS, consensus, cryptographic, or deployment artifact was introduced.
