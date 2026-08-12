# Opportunity, Commercial, and Billing Boundaries

## Opportunity representation

An opportunity is a client-bound operational prospect record, not an engagement. Stages are `IDENTIFIED`, `QUALIFICATION_REVIEW`, `PROPOSAL_REVIEW`, `NEGOTIATION_REVIEW`, `WON_PENDING_ENGAGEMENT`, `LOST`, `WITHDRAWN`, `DORMANT`, and `UNKNOWN_BLOCKED`.

Stage changes preserve actor, basis, time, prior stage, contradictions, and audit reference. `WON_PENDING_ENGAGEMENT` does not create an engagement, approve scope, authorize work, grant access, validate feasibility, or establish analytical truth. Only Layer 03 creates and governs engagement state.

Probability, value, priority, strategic importance, relationship strength, and forecast are operational estimates with method and as-of date. They are not facts, evidence quality, risk tolerance, resource authority, or release priority.

## Commercial metadata

Permitted metadata includes currency, non-sensitive fee descriptor, pricing-model label, budget range, purchase-order reference, contract-status reference, billing cadence, invoice-status reference, payment-status label, tax-jurisdiction label, and authoritative-system reference.

Layer 09 does not store payment instruments, banking details, credentials, signatures, substantive contracts, tax secrets, invoice documents, or accounting ledgers. It cannot create or amend terms, price, issue invoices, recognize revenue, accept payment, sign, procure, or bind any party.

Commercial fields are informational references. Missing or contradictory commercial state blocks only the action that depends on it; it cannot alter evidence or conclusions. An authoritative finance, legal, contract, or approval process remains separately required.

## Pressure firewall

No revenue, margin, sales stage, deadline, renewal risk, executive relationship, client tier, or sunk cost may:

- suppress or modify negative evidence, dissent, risk, QA, limitations, or contradictory findings;
- change evidence confidence, freshness, applicability, or professional-review requirements;
- create scope, approval, access, delivery, publication, or external-action authority;
- accelerate a gate by treating missing state as satisfied.

Attempts are preserved as pressure events and escalated without silently changing the requested or canonical record.

