# Approval Policy

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

## Control Objective

Preparation is permitted; execution is not unless authorized. The system maximizes analytical autonomy while minimizing uncontrolled execution authority. This policy applies to all agents, automations, humans, tools, and delegated workflows. Materiality definitions in `OPERATING_PRINCIPLES.md` apply.

## Approval Authority

The Human Principal may appoint a human delegate only through a written, time-bounded record that states the delegate, engagement, action categories, permission tier, financial limit, conditions, and expiration. Delegates may approve only within that record and may not subdelegate. Silence, urgency, prior similar approval, client pressure, or a request from an agent is not approval.

The following authority may never be delegated:

- final strategic decisions and approval or amendment of Core governance policies;
- approval of policy exceptions or acceptance of Critical residual risk;
- waiver of client confidentiality, engagement segregation, or conflict-of-interest requirements;
- legal commitments, contract signature, regulatory filing or representation, and public statements on behalf of the Lab or a client;
- approval of irreversible external actions;
- financial commitments above a lower documented non-delegable threshold or, if none exists, USD 100,000.

An action that falls outside a delegate's written scope requires Human Principal approval. Required legal, regulatory, security, privacy, or other qualified review remains mandatory regardless of approver.

## Agent Permission Tiers

Permission is least-privilege, engagement-specific, and additive only when explicitly granted. Each agent and task must have a recorded maximum tier. A higher tier includes lower-tier analytical permissions but not unrelated system or data access.

| Tier | Name | Permitted activity | Approval boundary |
|---|---|---|---|
| 0 | Read/Research | Read authorized sources and engagement data; collect and organize evidence without changing source systems | Human authorization of engagement, data, tools, and access is required before use |
| 1 | Analyze/Draft | Analyze authorized data; test hypotheses; challenge assumptions; model scenarios; create internal drafts and recommendations | No external communication, commitment, production change, or final decision |
| 2 | Prepare Actions | Prepare communications, filings, transactions, system-change plans, publication packages, or execution instructions in a non-executing state | Preparation only; artifacts must be clearly marked draft/unapproved |
| 3 | Execute Reversible Internal Actions | Execute specifically authorized internal actions that are logged, bounded, recoverable through a tested method, and create no external or Material commitment | Prior task-specific or standing human authorization within recorded scope; action stops if reversibility or scope is uncertain |
| 4 | External or Material Actions | External communication/contact/publication, financial or legal commitment, production/material change, sensitive disclosure, client delivery, destructive action, or any Material/Critical execution | Explicit human authorization tied to the exact action or artifact is mandatory immediately before execution |

Agents cannot raise their own tier, grant permissions, appoint delegates, approve their own work, or treat access capability as authority. Delegation to another agent must preserve the engagement identifier, data classification, maximum tier, task scope, stop conditions, and audit linkage. The delegating agent remains within its own tier and cannot delegate a permission it does not hold. Recursive delegation is prohibited unless the Human Principal explicitly authorizes its depth and purpose.

## Prohibited Autonomous Actions

No agent may autonomously:

- perform a Tier 4 action;
- send external communications, contact customers or third parties, publish, deliver to a client, or represent approval;
- create a financial or legal commitment, accept terms, make a payment, execute a trade, sign, file, certify, or waive rights;
- expose, transfer, combine, or reuse Sensitive information outside its authorized engagement and purpose;
- delete material records, make destructive or irreversible changes, deploy to production, or disable a control;
- change access, security configuration, governance policy, approval record, audit history, or its own permissions;
- conceal uncertainty, validation failure, conflict, incident, or policy exception.

## Actions Requiring Approval

Explicit human approval is required for every Tier 3 action and every Tier 4 action; final strategic decisions; Material/Critical reliance or delivery; policy exceptions; acceptance of material residual risk; external AI/vendor onboarding; import or export of Sensitive information; access grants or changes; cross-engagement or cross-jurisdiction transfers; retention changes or disposal; core prompt, tool, model, permission, or governance-control changes; and acceptance of licenses or click-through terms.

Tier 4 approval must be action-specific and contemporaneous. Standing approval cannot authorize public statements, legal commitments, regulatory filings, Critical sensitive-data disclosure, irreversible external action, or actions reserved to the Human Principal.

## Valid Approval Record

Approval is valid only when recorded before execution and linked to the applicable audit record. It must identify:

- approver, authority basis, timestamp, and engagement;
- exact action, artifact/version or integrity hash, system, recipient, amount, and permission tier;
- evidence, assumptions, expected benefit, cost, alternatives, confidence, and validation status;
- financial, legal, regulatory, security, privacy, client, reputational, and reversibility implications;
- scope, conditions, limits, execution window, expiration, and recurrence if any;
- executor and required post-execution verification.

Any material change to content, recipient, system, amount, timing, scope, evidence, confidence, or risk invalidates approval and requires renewal. Approval may be revoked before execution.

## Exception Control

Only the Human Principal may approve an exception. An exception request must record the policy/control, reason, affected engagement and artifact, risk assessment, alternatives, compensating controls, owner, start and expiration, monitoring, approval timestamp, and closure disposition. Exceptions must be narrow, time-bounded, auditable, and reviewed before reuse.

No exception may authorize fabrication; violation of law or binding obligation; unapproved Tier 4 execution; repository storage of credentials; concealment of an incident or conflict; unauthorized cross-client disclosure; self-approval; alteration of audit history; or waiver of required qualified human review where law, regulation, contract, or professional standard requires it.

## Approval Workflow and Stop Conditions

1. **Prepare:** Complete the proposed action or exact artifact without execution.
2. **Validate:** Complete required source, quality, conflict, security, and independent review.
3. **Disclose:** Present evidence, assumptions, benefit, cost, risks, alternatives, confidence, permission tier, and reversibility.
4. **Request:** Obtain a clear approve/reject decision for precise scope and timing.
5. **Verify:** Confirm approver authority, conditions, exact version, and unchanged circumstances.
6. **Execute:** A named executor performs only the approved action within the approved window.
7. **Record:** Capture outcome, deviations, post-execution checks, and final disposition.

An agent must stop and escalate under `OPERATING_PRINCIPLES.md` whenever evidence, authority, tier, exposure, sensitivity, legal/regulatory interpretation, reversibility, or policy compliance is unclear. If approval is absent, expired, changed, ambiguous, or technically impossible to verify, the action is not authorized.

## Revision History

| Version | Date | Change | Approval |
|---|---|---|---|
| 0.1.0-draft | 2026-08-10 | Initial approval framework | Not approved |
| 0.2.0-draft | 2026-08-10 | Added authority, permission tiers, exceptions, and approval records | Approved for Design by Leonel Aguilera Valdez; operational use not authorized |
