# Synthesis Integrity, Deliverable Release, and Recovery

## Canonical Relationship

Lifecycle state authority resides only in `STATE_MACHINE.md`; record shapes reside in `RECORD_SCHEMAS_AND_AUDIT.md`. This document defines the synthesis-integrity gate, release semantics, failure containment, and recovery.

## Synthesis Integrity Gate

An independent synthesis-integrity reviewer must compare the proposed decision package and deliverable against:

- primary analysis and decision-critical claims;
- independent challenge and every Material finding disposition;
- evidence coverage, retrieval proofs, freshness, corroboration, and conflicts;
- analytical validation and reproduced outputs;
- risk review, residual risks, mitigations, and acceptance authority;
- decision record, alternatives, downside/failure cases, and confidence;
- unresolved contradictions, dissent, limitations, and exceptions;
- Human Principal request, scope, audience, and required decision fields.

The reviewer must produce a coverage manifest mapping every Material deliverable assertion to claim IDs and verifying that emphasis, numbers, confidence, caveats, and recommended action are faithful to validated records. The synthesizer cannot perform this review.

The gate fails when the deliverable adds unsupported claims, omits or minimizes Material dissent/risk, changes causal or confidence meaning, alters validated figures, lacks required alternatives, or uses persuasive language beyond the evidence. Failure produces `SYNTHESIS_FAILED`; Human Principal review cannot begin.

## Deliverable Approval and Authorization

`APPROVE_FOR_DELIVERY` is the Human Principal/business decision on the exact deliverable. `DELIVERY_AUTHORIZATION` is the state that verifies all required approval categories—including technical authorization where needed—immediately before action. A single combined approval record may satisfy both only if it explicitly contains every required category, authority, field, and exact artifact/action scope.

Any Material change to content, evidence, confidence, contradiction, risk, recipient, channel, system, conditions, or version/hash invalidates approval. Pure formatting changes still create a new artifact version and require presentation/security comparison plus exact-version approval, but do not require full analytical revalidation when an independent impact check proves no semantic change.

## Circuit Breakers and Containment

Circuit breakers stop new transitions within the narrowest safe boundary:

- task boundary for evidence, confidence, validation, state, or concurrency failure;
- engagement boundary for identity, authorization, cross-context, or client-data anomaly;
- tool/provider boundary for compromised or anomalous external behavior;
- Orchestrator-wide boundary for identity, permission, approval, audit, or control-plane integrity failure.

Tripped breakers deny new delegation, synthesis, approval consumption, delivery, and affected tool use. They preserve protected records and permit only minimum authorized containment and investigation. Reset requires root-cause disposition, control verification, audit reconciliation, affected-work impact analysis, and authorized recovery.

## Checkpoints and Recovery Points

A verified checkpoint is created after each successful canonical transition and references exact state/version, record versions/hashes, actors, authorization, audit sequence, and dependency set. A checkpoint is usable only if its audit and referenced records pass independent integrity verification.

Recovery point selection chooses the latest checkpoint before the earliest corrupted or uncertain dependency. Downstream records are quarantined, never silently reused. Restoration creates a new recovery lineage linked to the prior attempt; history remains intact.

## Canonical Recovery Authority

Recovery authority is separate from business, technical, deployment, and exception approval.

| Role | Authority | Prohibited authority |
|---|---|---|
| Recovery initiator | Detects corruption/partial state, opens containment, prepares recovery record | Cannot select final recovery point, reset breaker, or resume Material work |
| Recovery coordinator | Reconciles journals/dependencies and proposes checkpoint/rollback plan | Cannot independently verify or authorize own plan |
| Independent recovery verifier | Validates checkpoint, corruption boundary, external outcome, invalidations, and restored state | Cannot be failed transition owner or recovery coordinator |
| Recovery authorizer | Authorizes exact recovery plan within written scope | Cannot approve business recommendation, new deployment/action, or exception |
| Human Principal | Required when recovery affects Material/Critical decisions, approvals, risk, client/external outcome, or non-delegable matter | Recovery approval does not imply technical/deployment/business authorization |

The recovery initiator may act without prior approval only to fail closed, preserve evidence, and perform minimum authorized containment. Recovery execution requires an approved recovery record, current technical authorization, independent verification, and a canonical R02/R03/R05 transition.

### Checkpoint validation and reconciliation

The verifier confirms checkpoint audit continuity, record versions/hashes, approval consumption, lease/fence history, transition/action journal outcome, dependency graph, engagement isolation, and downstream invalidations. A checkpoint created or verified only by the failed owner is insufficient for Material recovery.

### Rollback and partial execution authority

Logical rollback before external execution requires recovery authorization and independent verification; Human Principal authorization is added when a Material decision/approval is affected. Partial Tier 3/4 or external execution requires outcome reconciliation and Human Principal approval where Material/non-delegable. Any compensating external action is a new action requiring its own ordinary approvals.

### Corrupted control state

Corruption of identity, engagement, permission, approval, audit, or recovery records opens an incident breaker. Recovery cannot rely only on the corrupted control plane: use an independently protected backup/manual path, verify integrity and scope, reconcile queued events, and re-establish trusted authority before reset.

### Recovery audit requirements

Record detection, initiator, breaker scope, corrupted/uncertain objects, checkpoint candidates, chosen recovery point, journal reconciliation, external outcome evidence, invalidated dependencies, verifier, authorization categories, execution, post-recovery checks, and breaker reset. Recovery events retain original audit lineage plus a recovery correlation ID.

Implementation planning must propose recovery-time and recovery-point objectives by risk class for Human Principal approval. Missing objectives keep work blocked; they do not permit uncontrolled recovery.

## Corrupted and Partial State

- Corrupted audit, approval, identity, permission, or engagement state causes Orchestrator-wide or engagement-level `INCIDENT_HOLD`.
- Partial analytical output is quarantined and cannot enter synthesis.
- Partial state transition rolls back logically to the last committed state after reconciliation.
- Partial Tier 3/4 action is never assumed rolled back; action journal status becomes `OUTCOME_UNKNOWN` and requires independent reconciliation.
- Material recovery that could change a decision, risk, approval, or external outcome requires independent review and Human Principal authorization where `00_CORE` requires it.

## Safe Manual Mode and Single-Point-of-Failure Control

If Orchestrator control is unavailable or untrusted, no automated state advancement occurs. Authorized humans may use a documented manual safe mode to inspect protected records, contain incidents, preserve evidence, and prepare recovery decisions. Manual mode cannot bypass tiers, validation, audit, or approval; every action is recorded through a separately trusted path or queued for reconciled append before normal operation resumes.

Future implementation planning must define availability architecture, backup integrity, recovery time and recovery point objectives, restore testing, and independent failover validation. These are not presumed by this design.

## Scalable Assurance Profiles

Governance scales through three canonical profiles defined by task class, not ad hoc waiver:

- Routine: source/security/self-check and audit escalation if reused materially.
- Material: independent challenge, validation, relevant risk review, synthesis integrity, and human gates.
- Critical: Material controls plus qualified human review and Critical authority.

Independent evidence validation and risk review may run in parallel after stable primary/challenge inputs. Automated future controls may perform schema, source-access, freshness, reconciliation, secret, lineage, duplicate, and audit-continuity checks, but may not replace required human approval or qualified Critical review. Exceptions and failed controls remain human-governed.

Queue, WIP, backpressure, capacity, and Human Principal routing are canonical in `ORCHESTRATOR_ARCHITECTURE.md`; this document does not redefine them.
