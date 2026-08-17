# Validation A — Construction Review

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.0`  
DERIVATION_TYPE: construction-team validation evidence

## Method and independence limit

The construction review parsed the sole normative model, enumerated identifiers, enums, objects, transition-source edge tuples, invariants, prohibitions, fixed falsification classes, historical families, predecessor contracts and deferred allocations, and compared every derived projection to it. This is not independent certification and cannot certify itself.

## Structural reconstruction

Reconstruction found 13 identifier types, 7 enum families, 15 Material objects, 18 transitions, 39 source-edge tuples, 25 invariants, 25 fixed falsification classes, 10 historical families, 11 Category 2 allocations, 18 exact predecessor contracts and one normative source. Duplicate object, transition and identifier IDs: 0. Unresolved sources/destinations: 0. Rank violations: 0. Same-rank edges: 0. Self-loops: 0. Cycles: 0. Competing normative sources: 0.

The initial construction contained one Category 1 backward dependency: Layer 18's reconciliation handoff was represented as an authority source for retry registration. It was removed. Retry now enters only through a new predecessor-authorized intent generation carrying an exact future independent reconciliation disposition and any separately required authorization. The Layer 18 handoff and executor evidence cannot authorize retry. Retest: PASS.

Fresh independent challenge then identified a second Category 1 defect: a registered attempt unable to bind a current Layer 14 barrier could not reach pre-dispatch revalidation and therefore had no route to explicit no-dispatch evidence or mandatory handoff. Transition T12 was remediated to require every terminal registered attempt without a dispatch to produce an explicit evidence generation directly, including barrier-binding, revalidation or invalidation failure. T15 then requires its complete non-authoritative handoff. Retest: PASS.

## Fixed Material falsification

Each class was challenged by substituting, omitting, replaying, conflicting or authority-escalating the named Material input in an otherwise eligible abstract history. Expected behavior was denial before dispatch or, after possible dispatch, immutable evidence plus unresolved-effect hold and independent handoff.

| # | Fixed class | Required blocker | Result |
|---:|---|---|---|
| 1 | Authorization identity/version/category/scope/authority substitution | exact authorized-intent and binding equality | PASS |
| 2 | Stale/expired/superseded/revoked/condition-failing authorization | currentness and revocation dominance | PASS |
| 3 | Action/domain/effect-slot substitution | stable identity invariant and Layer 14 equality | PASS |
| 4 | Target/provider/endpoint/recipient/operation/generation/redirect substitution | canonical target binding | PASS |
| 5 | Parameter/payload/amount/unit/currency/scope/purpose/boundary mutation | complete typed membership and hash equality | PASS |
| 6 | Originator/executor/adapter/workload/credential/attestation substitution | Layer 13 identity intersection and exact assignment | PASS |
| 7 | Human/capital/legal/public/privacy/irreversible/compensation bypass | consequence-class Human validation | PASS |
| 8 | Attempt replay/duplicate/ordinal laundering | unique immutable attempt identity | PASS |
| 9 | Duplicate dispatch/barrier reuse/new effect slot | at-most-one dispatch and Layer 14 barrier ownership | PASS |
| 10 | Concurrent execution/competing winner | global effect-slot/barrier and exact winner binding | PASS |
| 11 | Retry from executor claim/handoff/timeout/silence/ack/prior success | new external disposition and authorization requirement | PASS |
| 12 | Crash before dispatch/forged dispatch evidence | explicit no-dispatch evidence and provenance | PASS |
| 13 | Crash after possible dispatch before durable evidence | possible-effect hold; no retry | PASS |
| 14 | Timeout/transport ambiguity converted to no-effect/success/retry | mandatory unresolved-effect hold | PASS |
| 15 | Partial report suppressed/upgraded/aggregated/overwritten | append-only evidence, hold and handoff | PASS |
| 16 | Contradictory evidence suppressed/averaged/voted/selected | conflict preservation and hold | PASS |
| 17 | Missing evidence represented complete/empty | explicit membership; missing is not empty | PASS |
| 18 | Forged receipt/completion/effect/outcome/external-state claim | executor evidence is non-authoritative | PASS |
| 19 | Compensation/remediation/rollback/cancellation laundering | new consequential-action authorization required | PASS |
| 20 | Workflow/decision/release/assurance/admission/access/capability/credential/audit laundering | predecessor authority separation | PASS |
| 21 | Predecessor identity/version/hash/owner/boundary/currentness/SOD bypass | exact eighteen-contract manifest | PASS |
| 22 | Layer 14 permit/PEP/consumption/target/revalidation/closure/barrier bypass | exact E23 lineage and equality | PASS |
| 23 | Executor self-approval/validation/reconciliation/closure | categorical non-authority and independent receiver | PASS |
| 24 | Invalid/incomplete/cross-boundary/non-independent handoff | complete handoff and receiver-authority requirement | PASS |
| 25 | Audit/hold/invalidation/handoff converted to effect/outcome/retry/closure | descendant-only prohibitions | PASS |

Successful structural counterexamples after remediation: 0.

## Historical regression

| Source family | Applicable retained attacks | Result |
|---|---|---|
| Layers 00–01 | tier/category/exact-object/replay/delegation/Human proxy/irreversible action/external-tool ambiguity | BLOCKED |
| Layers 02–07 | producer self-validation/evidence contradiction/QA-risk-dissent/release-delivery/recipient substitution | BLOCKED |
| Layers 08–10 | reuse/client/entity/engagement/confidentiality/jurisdiction/cross-boundary laundering | BLOCKED |
| Layer 11 | dependency omission/concurrency/duplicate/retry/timeout/cancellation/rollback/completion laundering | BLOCKED |
| Layer 12 | decision/approval/exception/waiver/SOD/legal/compliance/risk-acceptance/decision-to-execution | BLOCKED |
| Layer 13 | actor/deputy/executor/authorization/access/revocation/break-glass/replay/credential possession | BLOCKED |
| Layer 14 | action/domain/slot/permit/PEP/target/revalidation/barrier/uniqueness/crash/recovery/audit authority | BLOCKED |
| Layer 15 | recommendation/decision/delivery/non-executing handoff/observational outcome escalation | BLOCKED |
| Layer 16 | false implementation/conformance/atomicity/uniqueness/fencing/recovery/telemetry/attestation | BLOCKED |
| Layer 17 | assurance/admission/rollback-readiness/authority concentration | BLOCKED |

Still-successful applicable historical counterexamples: 0.

## Findings and construction verdict

- Category 1 discovered: 2; remediated: 2; remaining: 0.
- Category 2: 11 explicitly allocated families with all six required fields.
- Category 3: 0.
- Category 4: 0.

Invariants A–Y: PASS. Human Principal boundary: PASS. Exact predecessor mapping: PASS. Executor self-certification paths: 0. Effect/outcome/reconciliation/closure authority inside Layer 18: 0. Runtime or external-action artifacts: 0.

Construction-team verdict: PASS under the fixed architecture-only boundary, subject to fresh independent certification. Further attack-count growth is optional unless a materially new class appears.
