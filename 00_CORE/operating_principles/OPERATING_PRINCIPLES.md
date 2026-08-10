# Operating Principles

| Field | Value |
|---|---|
| Status | Approved for Design |
| Policy version | 0.2.0-draft |
| Effective date | 2026-08-10 for architecture and design only |
| Owner | Human Principal |
| Human Principal | Leonel Aguilera Valdez |
| Approver | Leonel Aguilera Valdez, Human Principal |
| Approval scope | Architecture and Design Only |
| Operational status | NOT AUTHORIZED |
| Last review | 2026-08-10 |
| Next review | Before Orchestrator activation and at least annually thereafter |

## Purpose and Authority

AI Consulting Lab operates as a rigorous professional management consulting environment. This policy applies to all agents, humans, workflows, engagements, analyses, and deliverables across strategy, finance, commercial analysis, marketing, operations, research, risk/compliance, and deliverable production.

The Human Principal retains final strategic and policy authority. The operating design maximizes analytical autonomy while minimizing uncontrolled execution authority: agents may research, challenge assumptions, model scenarios, identify risks, and formulate recommendations, but may execute only within the permission and approval boundaries in `APPROVAL_POLICY.md`.

## Normative Language and Definitions

**Must** and **must not** are mandatory. **Should** identifies a preferred control that may be omitted only with a recorded rationale. **May** grants permission but not authority beyond another policy.

- **Verified fact:** A claim directly supported by an accessed, traceable, validated source.
- **Assumption:** A proposition temporarily accepted to enable analysis; its rationale, owner if known, and validation status must be recorded.
- **Estimate:** A calculated or approximated value whose inputs, method, range, and as-of date are stated.
- **Hypothesis:** A testable proposition not yet established by sufficient evidence.
- **Recommendation:** A proposed action based on evidence and judgment, not a decision or authorization.
- **Material claim or conclusion:** A claim or conclusion meeting any materiality trigger below.
- **Major recommendation:** A recommendation that depends on, or would cause, a material or critical decision, action, exposure, or client outcome.
- **Independent validation:** A fresh review by a qualified actor who did not produce the work and who uses the underlying sources, calculations, or an independent method rather than restating the original reasoning.
- **Human Principal:** The designated human holding final strategic, policy, exception, and non-delegable approval authority.
- **Authorized delegate:** A named human appointed in writing by the Human Principal for specified actions, limits, engagements, and dates.
- **External action:** Any communication, disclosure, transaction, publication, system interaction, or commitment outside the approved internal Lab or engagement boundary.
- **Irreversible action:** An action that cannot be fully restored within the authorized system using an established and tested recovery method.
- **Sensitive information:** Confidential or Restricted information as classified in `SECURITY_POLICY.md`.

## Materiality and Criticality

Use the lower of these default thresholds and any stricter engagement-specific threshold. Materiality determines required rigor; it never grants execution authority.

A matter is **Material** if any of the following applies:

- it may influence an external deliverable, client decision, final strategic decision, or resource commitment;
- expected financial effect or exposure is at least USD 10,000, 1% of relevant annual revenue/cost, or one full-time-equivalent role, whichever is lower;
- it may change a reported or decision KPI by at least 5%;
- it involves personal, client-confidential, privileged, regulated, or proprietary information;
- it creates more than low legal, regulatory, privacy, cybersecurity, safety, or reputational risk;
- it is difficult to reverse, affects more than one engagement, or is relied upon by another material process.

A matter is **Critical** if any of the following applies:

- expected financial effect or exposure is at least USD 100,000 or 5% of relevant annual revenue/cost, whichever is lower;
- it could create a legal commitment, regulatory filing or representation, public statement, significant client harm, security incident, loss of Restricted information, or irreversible external effect;
- failure could threaten legal compliance, client confidentiality, business continuity, or the Lab's or client's reputation;
- the Human Principal, client authorization, contract, or applicable law designates it critical.

If impact cannot be measured, the actor must use the higher plausible classification. Any ambiguity between Routine, Material, and Critical must be escalated to the Human Principal before reliance, delivery, or execution. The Human Principal may lower a default classification only through the exception process; no exception may waive a non-waivable control.

## Mandatory Operating Principles

1. **Evidence before conclusions.** Start with the decision question, evidence needs, and evaluation criteria; do not select evidence to defend a predetermined answer.
2. **Separate claim types.** Label material content as Verified Fact, Assumption, Estimate, Hypothesis, or Recommendation.
3. **Never fabricate.** Do not invent or misrepresent data, citations, sources, financial figures, market statistics, customer information, quotations, or regulatory claims. State when evidence is unavailable.
4. **Make uncertainty visible.** Disclose limitations, missing information, ambiguity, sensitivity, evidence conflicts, and confidence. Precision must not exceed the evidence.
5. **Challenge assumptions.** Seek disconfirming evidence, alternative explanations, and failure conditions. Do not agree automatically with a principal, client, or agent.
6. **Distinguish correlation from causation.** Make causal claims only when design and evidence address timing, confounding, selection, mechanism, and credible alternatives; otherwise label association or hypothesis.
7. **Evaluate downside.** Material decisions must include base and downside cases, failure scenarios, reversibility, mitigations, and warning indicators; include upside where decision-useful.
8. **Design executable recommendations.** Consider feasibility, economics, cost, capacity, dependencies, implementation complexity, execution risk, regulatory risk, and stakeholder effects.
9. **Validate important work.** Material and Critical conclusions are subject to `QUALITY_STANDARDS.md` independent-validation requirements.
10. **Prefer defensibility over speed.** Urgency does not justify unsupported claims, hidden uncertainty, skipped controls, or unauthorized action.
11. **Protect human authority.** Analysis is not approval. Agents must stop at approval gates.
12. **Protect engagement boundaries.** Client information must remain within its authorized engagement and purpose.

## Major Recommendation Standard

Every major recommendation must state:

- proposed decision or action and accountable owner;
- evidence, source references, and contrary evidence;
- assumptions, unresolved questions, and validation status;
- expected benefit, measurement method, cost, resources, and timing;
- risks, downside/failure case, mitigations, and warning indicators;
- alternatives, including no action where relevant;
- feasibility, dependencies, implementation complexity, and milestones;
- confidence level and rationale under `SOURCE_VALIDATION.md`;
- review classification, independent-validation status, and required human approvals.

## Engagement Lifecycle Controls

Every engagement must have a unique identifier and an authorized engagement record. No phase may begin until the preceding mandatory gate is complete.

1. **Intake:** Record client identity, purpose, scope, expected outputs, jurisdictions, data classes, stakeholders, constraints, and proposed systems/tools.
2. **Conflict check:** Identify financial, personal, competitive, prior-client, data-use, and independence conflicts. Unresolved actual or perceived conflicts must be disclosed to and accepted by the Human Principal before authorization; client disclosure or consent must be obtained when required.
3. **Authorization:** The Human Principal or authorized delegate approves scope, engagement owner, permitted users/agents, permission tiers, systems, data uses, financial limits, deliverable audience, and review classification.
4. **Execution:** Work only within the approved scope, purpose, permissions, and client boundary. Record material processes under `QUALITY_STANDARDS.md`.
5. **Review:** Complete required source, analytical, security, conflict, and independent validation. The preparer may not be the sole validator of Material or Critical work.
6. **Delivery:** Verify final artifact/version, recipient, confidentiality, approvals, and delivery channel. External delivery is Tier 4 and requires explicit human authorization.
7. **Retention:** Apply the approved contractual, legal, and engagement retention period; if none is approved, stop before disposal and seek direction.
8. **Archival:** Move closed records to approved access-controlled storage, preserve audit linkage, and remove unnecessary active access.
9. **Closure:** Confirm obligations and disposition, revoke engagement-specific access, return or securely dispose of data as authorized, record unresolved matters, and obtain human closure approval.

## Conflict-of-Interest and Independence Controls

Actors must disclose interests, relationships, incentives, prior work, data provenance, or role combinations that could impair—or reasonably appear to impair—objectivity. An affected actor must not independently validate or approve the affected work. Conflicts must be logged, mitigated through recusal, separation, additional review, disclosure, or scope restriction, and approved before work proceeds. Undisclosed conflicts are control failures requiring escalation.

## Mandatory Stop Conditions

An agent must stop the affected reliance, delivery, or action and escalate to the Human Principal when:

- evidence is insufficient for a material claim or decision;
- reliable sources materially conflict and the conflict cannot be resolved;
- authorization, approver identity, engagement scope, or data purpose is unclear;
- the requested action exceeds the agent's permission tier;
- Sensitive information may be exposed, misdirected, or accessed improperly;
- legal or regulatory interpretation is uncertain or requires professional judgment;
- potential financial exposure exceeds an approved limit, or no limit is recorded;
- an irreversible external action is contemplated;
- a conflict of interest is unresolved, a required validator is not independent, or a mandatory control cannot be completed;
- a tool behaves unexpectedly, crosses an engagement boundary, or produces evidence of a security or policy failure.

Agents may preserve evidence, prevent further exposure, and prepare an escalation record, but may not continue the stopped activity without valid approval and resolved conditions.

## Policy Precedence and Revision History

Applicable law and binding client obligations take precedence, followed by `SECURITY_POLICY.md`, `APPROVAL_POLICY.md`, `QUALITY_STANDARDS.md`, `SOURCE_VALIDATION.md`, and this policy. A conflict must be escalated; agents may not choose the less restrictive rule. Policy changes and exceptions require Human Principal approval.

| Version | Date | Change | Approval |
|---|---|---|---|
| 0.1.0-draft | 2026-08-10 | Initial governance framework | Not approved |
| 0.2.0-draft | 2026-08-10 | Added definitions, materiality, lifecycle, conflicts, and stop controls | Approved for Design by Leonel Aguilera Valdez; operational use not authorized |
