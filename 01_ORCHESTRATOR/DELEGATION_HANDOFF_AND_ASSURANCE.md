# Delegation and Assurance Architecture

## Canonical Scope

This document is the canonical design for delegation, independent challenge, validator qualification, and validator independence. `00_CORE` controls remain authoritative.

## Delegation Invariants

Delegation distributes bounded work, never authority. Every child task must reference one parent task and one immutable engagement ID and must inherit the more restrictive parent value for scope, classification, data class, permission ceiling, approvals, stop conditions, and retention. A delegate cannot widen scope, raise tier, change engagement, or recursively delegate unless the parent authorization explicitly permits a bounded depth.

## Handoff Contract

A handoff is valid only when its record contains an ID/version, parent and child task IDs, engagement ID, sender and recipient identities, issued timestamp, bounded scope, classification, risk domains, permission ceiling, data class, authorized input references, required outputs, acceptance criteria, known conflicts, stop conditions, deadline, and audit correlation ID.

The recipient must explicitly accept and attest that engagement, scope, tier, access, qualifications, and conflicts are correct. Unknown or mismatched values produce `BLOCKED_DELEGATION`. Silence is not acceptance.

## Assurance Roles

Material recommendations require distinct primary-analysis, challenge, evidence-validation, risk-review, synthesis-integrity, and human-decision records. The same eligible person may fill more than one assurance role only where `00_CORE` permits it and the role combination is not prohibited below. The primary preparer can never validate or approve its own Material/Critical work.

### Challenge mandate

The challenger must attempt falsification: construct the strongest alternative, seek disconfirming evidence, identify weak assumptions and causal alternatives, test downside/failure cases and reversal conditions, and record disagreements on classification, confidence, and risk. Every Material finding requires an owner and disposition. Unresolved Material dissent remains visible through Human Review.

### Validation mandate

The validator must retrieve and inspect decision-critical evidence independently, verify claim support and provenance, reproduce decision-driving calculations or use an independent method, test key assumptions and sensitivities, assess challenge remediation, and issue `PASS`, `PASS_WITH_LIMITATIONS`, or `FAIL`. `PASS_WITH_LIMITATIONS` cannot conceal an unresolved Material issue.

## Mechanically Verifiable Independence

Independence is evaluated by an `independence_record`, not by role name. All mandatory checks must be `PASS` before validation begins:

| Dimension | Required evidence | Failure result |
|---|---|---|
| Actor identity | Validator has a stable identity distinct from every primary preparer | `BLOCKED_VALIDATION` |
| Model/session | Validator execution/session ID differs from primary; model/provider/version recorded | Same session is prohibited; correlated model use is disclosed |
| Context lineage | Input manifest proves validator received only the approved validation packet | Hidden/shared primary context blocks validation |
| Assumptions | Validator creates an independent assumption inventory before viewing primary responses | Material identical assumptions require reconciliation, not automatic acceptance |
| Methodology | Validator records a fresh reproduction or independent method | Restatement or reuse without independent testing fails |
| Evidence provenance | Validator records independent retrieval/access proofs for decision-critical evidence | Primary-supplied metadata alone is insufficient |
| Conflicts | Signed/recorded conflict attestation and registry check | Actual/apparent unresolved conflict blocks |
| Qualifications | Registry version and qualification evidence match subject, method, jurisdiction, and data class | Missing/expired qualification blocks |

### Context lineage control

The validation packet is an explicit manifest containing task/engagement IDs, decision question, claim IDs, source identifiers, calculation inputs, and acceptance criteria. It excludes the primary recommendation and persuasive narrative during the validator's first-pass evidence and assumption review. After first-pass findings are sealed, the validator may compare the primary analysis and challenge responses. Packet contents, access order, session identity, and output hashes are audited.

### Correlated-agent control

Different actor IDs are never sufficient. Apply these explicit criteria:

| Correlation | PASS criteria | FAIL criteria |
|---|---|---|
| Same execution/session | None | Same session, shared hidden state, or reused conversation lineage always fails |
| Same model and provider | May pass only as `METHOD_INDEPENDENT_LIMITED` for Material work when session/context are isolated, evidence is independently retrieved, assumptions are independently derived, a genuinely different method is used, and the limitation is disclosed/escalated | Any missing condition; or representing it as model-diverse validation |
| Same provider, different model family | May pass when session/context/evidence/assumptions/method checks pass and provider-level correlated risk is recorded | Shared provider output cache, inherited reasoning, or undisclosed common failure mode |
| Different provider/model | Passes this dimension only; all other dimensions still required | Treating provider diversity as proof of full independence |
| Shared source list | Allowed only when validator independently retrieves and verifies original sources and independently searches for contrary evidence | Shared summaries, cached extracts, citations, or provenance metadata accepted without retrieval |
| Shared assumptions | Common externally mandated facts may be shared and labeled | Decision-critical assumptions inherited without an independently sealed assumption inventory and reconciliation |
| Shared prompt/template | Common policy and task schema may be shared | Shared persuasive reasoning, recommendation framing, chain-of-thought artifact, or answer template that anchors conclusions |

`METHOD_INDEPENDENT_LIMITED` is not full independent corroboration. If the Material conclusion depends materially on model judgment rather than reproducible evidence/method, same-model/provider validation fails and a qualified human or sufficiently heterogeneous eligible validator is required. Critical work always requires qualified human review and cannot use AI-only correlation controls as the final validation.

If true independence cannot be established, set the independence result to `FAIL` or `LIMITED`, cap confidence as required by `00_CORE`, enter `BLOCKED_VALIDATION`, disclose the exact correlations, and escalate according to Material/Critical classification. A limitation cannot be cured by changing the actor label.

### Anti-masquerade control

The trusted workflow controller—not the primary agent—assigns validator identity and creates the validation packet. The primary agent cannot write or modify the validator assignment, independence record, validation result, or qualification evidence. Any identity collision, session reuse, manifest alteration, or missing lineage event is a validation failure and security event.

## Qualification Control

Qualification records must identify subject and methodology scope, jurisdiction where relevant, evidence supporting qualification, approving human, effective/expiry dates, prohibited roles, and registry version. Qualification is reassessed when scope, method, jurisdiction, model/tool version, or risk classification changes materially.

Legal/regulatory claims affecting rights, obligations, filing, or compliance require qualified human review. No AI analysis substitutes for licensed or contractually required professional review.

## Assurance Completion Gate

Assurance completes only when:

- challenge findings are dispositioned without hiding dissent;
- independence checks pass;
- evidence and calculations pass or limitations are explicitly non-Material;
- risk review is complete;
- confidence is recalculated;
- validator output is sealed from primary-agent modification;
- all assurance records and audit events are complete.

Unavailable, compromised, or failed validation produces `BLOCKED_VALIDATION`, caps confidence at Low, and prevents Material/Critical reliance or delivery except for the narrow internal exception in `00_CORE` approved by the Human Principal. Unvalidated Critical conclusions can never be delivered externally.
