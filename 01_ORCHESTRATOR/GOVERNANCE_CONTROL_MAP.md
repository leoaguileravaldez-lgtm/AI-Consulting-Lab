# Governance Control Map

This map identifies canonical `00_CORE` authority. Design artifacts operationalize but do not amend policy.

| Control area | Canonical authority | Design artifact |
|---|---|---|
| Mission and control-plane boundaries | `00_CORE/operating_principles/OPERATING_PRINCIPLES.md` | `ORCHESTRATOR_ARCHITECTURE.md` |
| Definitions/materiality | Operating Principles | `TASK_INTAKE_CLASSIFICATION_AND_MATERIALITY.md` |
| Lifecycle/conflicts/stops/precedence | Operating Principles | State, task, approval, security documents |
| Classification/reproducibility/independence | `00_CORE/quality_standards/QUALITY_STANDARDS.md` | `DELEGATION_HANDOFF_AND_ASSURANCE.md` |
| Formal schemas, audit, concurrency/idempotency | Quality and Security policies | `RECORD_SCHEMAS_AND_AUDIT.md` |
| Tiers/approval categories/authority | `00_CORE/approval_policy/APPROVAL_POLICY.md` | `AUTHORITY_PERMISSIONS_AND_DUTIES.md` |
| Stops/escalation/exceptions | Operating and Approval policies | `APPROVAL_STOP_AND_ESCALATION.md` |
| Sources/freshness/confidence/conflicts | `00_CORE/source_validation/SOURCE_VALIDATION.md` | `EVIDENCE_CONFIDENCE_AND_RISK.md` |
| Segregation/security/threat controls | `00_CORE/security_policy/SECURITY_POLICY.md` | `SECURITY_BOUNDARIES_AND_THREAT_MODEL.md` |
| Synthesis/release/recovery/scalability | All five policies | `DELIVERABLE_LIFECYCLE_AND_RECOVERY.md` |
| Canonical lifecycle | All five policies | `STATE_MACHINE.md` |

If design conflicts with `00_CORE`, `00_CORE` controls. Record and escalate the conflict; never select the less restrictive rule.

Repeated policy language is explanatory. Canonical authority remains in `00_CORE`; canonical design ownership is assigned above. If two design documents differ, the mapped canonical design document controls unless it conflicts with `00_CORE`. A Core change requires mapped impact analysis before design reliance.

## Canonical Control Ownership

| Control | Accountable design role | Canonical specification |
|---|---|---|
| State vocabulary and transitions | `WORKFLOW_CONTROLLER` with Human Principal governance approval | `STATE_MACHINE.md` |
| Schema compatibility and referential integrity | `RECORDS_OWNER` | `RECORD_SCHEMAS_AND_AUDIT.md` |
| Audit append and completeness rules | `AUDIT_APPENDER`; independently checked by `AUDIT_VERIFIER` | `RECORD_SCHEMAS_AND_AUDIT.md` |
| Approval categories and trusted writes | `APPROVAL_WRITER`; authority remains with applicable human approver | `AUTHORITY_PERMISSIONS_AND_DUTIES.md` |
| Permissions and tier issuance | `PERMISSION_AUTHORITY` | Authority document plus `00_CORE` Approval Policy |
| Validator qualification and capacity | `REGISTRY_OWNER` and `CAPACITY_CONTROLLER`; no self-registration | `SPECIALIST_REGISTRY.md`, assurance and architecture documents |
| Evidence coverage rules | `VALIDATOR` control owner under `RECORDS_OWNER` schema governance | `EVIDENCE_CONFIDENCE_AND_RISK.md` |
| Engagement isolation certification | `SECURITY_REVIEWER` | `SECURITY_BOUNDARIES_AND_THREAT_MODEL.md` |
| Synthesis integrity | `SYNTHESIS_REVIEWER` | `DELIVERABLE_LIFECYCLE_AND_RECOVERY.md` |
| Circuit breakers and incident control | `SECURITY_REVIEWER`/`INCIDENT_AUTHORITY` by scope | Recovery and security documents |
| Recovery | `RECOVERY_COORDINATOR`, `RECOVERY_VERIFIER`, and `RECOVERY_AUTHORIZER` | `DELIVERABLE_LIFECYCLE_AND_RECOVERY.md` |
| Capacity/backpressure | `CAPACITY_CONTROLLER` | `ORCHESTRATOR_ARCHITECTURE.md` |
| External provider monitoring | `SECURITY_REVIEWER` and future gateway owner | `FUTURE_EXTERNAL_TOOL_BOUNDARY.md` |
| Specialist registry taxonomy and component classification | `REGISTRY_OWNER`; authority remains with the approving human | `SPECIALIST_REGISTRY.md` |
| Primary specialist practice boundaries | Future `02_SPECIALISTS` design owner; coordinated by `WORKFLOW_CONTROLLER` | `SPECIALIST_REGISTRY.md` until a future approved `02_SPECIALISTS` package exists |
| Research/evidence, challenge, validation, and risk/QA module boundaries | Future independent assurance design owner; workflow coordinated by `WORKFLOW_CONTROLLER` using the applicable canonical assurance roles | `SPECIALIST_REGISTRY.md` plus assurance, evidence, and risk documents |
| Deliverable-support module boundary | Future independent deliverables design owner; coordinated by `SYNTHESIZER` and checked by `SYNTHESIS_REVIEWER` | `SPECIALIST_REGISTRY.md` and `DELIVERABLE_LIFECYCLE_AND_RECOVERY.md` |
| Client-lifecycle module boundary | Future independent client-lifecycle design owner; controlled by intake, conflict, approval, security, and records roles as applicable | `SPECIALIST_REGISTRY.md` plus intake, authority, security, and records documents |

Control-role appointments are formal records defined in `RECORD_SCHEMAS_AND_AUDIT.md`. A role name without an active assignment grants no authority.

## Component-Ownership Boundary

Component ownership does not transfer canonical control authority. The future `02_SPECIALISTS` package may define primary practices and transversal analytical capabilities only. Future assurance, deliverables, and client-lifecycle modules remain independent of that package. `01_ORCHESTRATOR` coordinates their work through canonical roles, handoffs, state guards, and records but does not become their substantive producer, independent reviewer, risk acceptor, approver, or external actor.

The canonical registry taxonomy distinguishes `module_id`, `practice_id`, `capability_id`, `role_type`, `actor_id`, and `session_id`. None may substitute for another. A directory name, component-class assignment, practice membership, capability entry, role label, actor identity, or session identity grants no permission or authority by itself. Eligibility requires the exact current registry, qualification, conflict, engagement, role-assignment, permission, and independence records applicable to the work.

Any future module may add stricter subordinate mandates and interfaces but may not redefine state transitions, approval categories, permission tiers, materiality, evidence standards, independence, engagement isolation, or trusted authority. A conflicting or duplicate rule fails canonical-drift review and blocks design or configuration release.

## Canonical Conformance Test Specification

These are future test requirements, not implemented tests. Each test records design/control version, fixture IDs, preconditions, action, expected result/state/audit event, actual result, evidence, tester independence, and pass/fail. Unknown or untestable results fail conformance.

| Test family | Required test cases | Required result |
|---|---|---|
| Canonical-description drift | Compare all secondary references, state names, tiers, thresholds, confidence, approval categories, and schemas with canonical owners; introduce a conflicting secondary rule | Exact match/reference passes; duplicate conflicting rule fails and blocks release of design/configuration |
| State-transition enforcement | Exercise every listed normal, blocked, failure, exception, recovery, cancellation, supersession, and terminal path; attempt every prohibited source-target class; omit each guard in turn | Listed path succeeds only with exact actor/evidence/approval/audit; every unlisted or incomplete path is rejected without state change |
| Engagement isolation | Valid same-engagement access; missing/forged/foreign IDs; cache/retrieval/log/backup/template leakage; child rebinding; mixed lineage | Only exact authorized same-engagement access succeeds; all negative cases deny, quarantine, audit, and hold as specified |
| Approval consumption | Exact valid approval; wrong category/authority/version/hash/recipient/system; expired/revoked/consumed nonce; replay under new key; Material change | Only exact current approval succeeds once; all others enter authorization block with no action |
| Validator independence | Same actor/session; hidden/shared context; shared summaries; same model/provider with and without independent method; expired qualification; conflict; sealed first-pass sequencing | Outcomes match the assurance pass/limited/fail matrix; false label changes cannot alter result |
| Claim-to-evidence integrity | Valid proof; fabricated metadata; inaccessible source; mismatched content; stale/superseded source; missing corroboration; lineage duplication; unresolved Material contradiction | Only adequately supported claim passes; failures block evidence/synthesis/release and remain visible |
| Audit continuity | Alter, delete, duplicate, reorder, insert, omit expected event, use stale sequence, submit unauthorized correction, restore archive | Integrity verifier detects every mutation/completeness failure and opens required hold; valid correction appends without overwrite |
| Recovery | Valid/invalid checkpoint; corrupt audit/approval/identity; failed-owner self-verification; missing invalidation; unauthorized rollback; partial external outcome | Only independently verified and authorized recovery path commits; all unsafe cases remain held or outcome-unknown |
| Idempotency/replay | Same key/same input; same key/different input; different key/same nonce; crash at every crash-matrix point; provider receipt lost | No duplicate committed transition/action; mismatches reject; unknown external outcome never automatically retries |
| Concurrency/fencing | Two workers acquire/renew/expire/take over; stale owner commits after lease loss; crash while committing; parallel child merge | One fencing token owns commit; stale owner cannot write/consume/dispatch; merge occurs only at canonical synthesis |
| Capacity/backpressure | Exhaust each stage/validator/Human queue; source expires while queued; P0 incident arrives under load; missing mandatory reviewer | Lower-priority admission stops without reducing controls; incident work is prioritized; stale work returns to evidence validation; missing capacity blocks/escalates |
| Fail-closed behavior | Remove identity, engagement, permission, audit, approval, registry, evidence, security, or capacity dependency; inject timeout/unknown result | No forward transition or external action; narrowest breaker opens; evidence and audit trail are preserved where available |
| Synthesis integrity | Omit dissent, alter validated number, inflate confidence, add unsupported claim, remove risk, change causal wording, formatting-only change | Material semantic changes fail synthesis; proven formatting-only changes follow limited reapproval path |
| External-tool boundary | SSRF/private target, malicious redirect, provider impersonation, poisoned response, credential leak, tool drift, ambiguous outcome | Gateway denies/quarantines, opens breaker, logs safely, and never auto-retries Material/external action |
| Registry taxonomy separation | Substitute a module, directory, practice, capability, role, actor, or session identifier for another; infer permission or authority from each; relabel a conflicted or unqualified actor/session | Every substitution and inferred-authority attempt is rejected; exact typed records and versioned supersession are required |
| Control-module ownership | Assign primary analysis, challenge, validation, risk acceptance, deliverable approval, client authority, and Orchestrator coordination to conflicting or duplicate owners | Incompatible combinations and duplicate authority fail; Orchestrator coordination never becomes substantive assurance or approval authority |

### Conformance gates

- Design/configuration baseline changes cannot proceed if canonical drift tests fail.
- Implementation planning must map each test to an enforceable control and evidence artifact.
- Activation is prohibited until all applicable Critical and Material negative tests pass independently.
- A Material control change reruns affected conformance families before reliance.
- Test automation may execute deterministic checks, but independent human/security review remains required where `00_CORE` requires it.

## Design Package Inventory

- `ORCHESTRATOR_ARCHITECTURE.md`
- `AUTHORITY_PERMISSIONS_AND_DUTIES.md`
- `SPECIALIST_REGISTRY.md`
- `TASK_INTAKE_CLASSIFICATION_AND_MATERIALITY.md`
- `STATE_MACHINE.md`
- `DELEGATION_HANDOFF_AND_ASSURANCE.md`
- `EVIDENCE_CONFIDENCE_AND_RISK.md`
- `APPROVAL_STOP_AND_ESCALATION.md`
- `RECORD_SCHEMAS_AND_AUDIT.md`
- `DELIVERABLE_LIFECYCLE_AND_RECOVERY.md`
- `SECURITY_BOUNDARIES_AND_THREAT_MODEL.md`
- `FUTURE_EXTERNAL_TOOL_BOUNDARY.md`
- `GOVERNANCE_CONTROL_MAP.md`

The package contains architecture/governance specifications only. It authorizes no executable agents, client processing, external connectivity, Tier 3/4 action, commit, push, release, or deployment.
