# Task Intake, Materiality, and Risk Classification

## Canonical Scope

This document is the canonical task-intake and classification design. `00_CORE` thresholds are controlling. Classification determines rigor and never grants action authority.

## Intake Gate

Intake requires a unique task ID, a trusted provisional or active immutable engagement ID, requestor, decision question, scope in/out, outputs, audience, jurisdictions, data class, authorized sources/tools, exposure estimates, approved limits, reversibility, deadline, assumptions, conflicts, retention, tier ceiling, approver, and authorization reference. A provisional engagement permits intake/conflict preparation only; analysis and specialist access require activation at `ENGAGEMENT_AUTHORIZATION`.

The workflow cannot leave `DRAFT_INTAKE` if engagement, purpose, scope, data class, authority, financial limit, tier, approver, or conflicts are absent or ambiguous.

## Controlled Classification Taxonomy

### Routine

All of these must be true: no external/client/final-strategic reliance; below every financial/KPI/people threshold; no Sensitive information; no more-than-low regulated/security/safety/reputational risk; readily reversible; no cross-engagement or Material dependency.

### Material

Any `00_CORE` trigger applies: external/client/final-strategic/resource influence; at least USD 10,000, 1% relevant annual revenue/cost, or one FTE, whichever is lower; at least 5% decision KPI effect; Sensitive information; more-than-low legal/regulatory/privacy/cyber/safety/reputational risk; difficult reversal; cross-engagement effect; or reliance by a Material process.

### Critical

Any `00_CORE` trigger applies: at least USD 100,000 or 5% relevant annual revenue/cost, whichever is lower; legal commitment, filing/representation, public statement, significant client harm, security incident, Restricted-data loss, irreversible external effect; threat to compliance/confidentiality/continuity/reputation; or Critical designation by Human Principal, client authorization, contract, or law.

Unknown impact or uncertainty between levels defaults to the more restrictive plausible classification until independent confirmation or Human Principal disposition. Lowering requires the `00_CORE` exception process.

## Risk Taxonomy and Escalation

Apply the canonical risk categories and likelihood/impact anchors in `EVIDENCE_CONFIDENCE_AND_RISK.md`. Escalation is mandatory when:

- any impact is Critical;
- residual risk meets a Material/Critical trigger;
- financial exposure exceeds the lower applicable approved limit;
- interacting Moderate risks plausibly produce Material impact;
- legal/regulatory jurisdiction or obligation is uncertain;
- evidence/methodological risk could reverse a Material recommendation;
- client confidentiality or security boundary is uncertain.

## Anti-Underclassification Controls

1. Evaluate the entire decision and dependency graph, not isolated subtasks.
2. Aggregate related tasks across the active engagement when they share decision, recipient, budget, implementation, data, or outcome.
3. Permit independent reviewer/risk reviewer to raise classification or mark a claim decision-critical.
4. Prohibit primary preparer from lowering classification or removing a decision-critical designation alone.
5. Reclassify at every transition involving Material scope, data, evidence, cost, recipient, system, confidence, or risk change.
6. Audit original classification, changes, actor, rationale, and confirmation.

## Mandatory Gates by Class

| Gate | Routine | Material | Critical |
|---|---|---|---|
| Source/security/self-check | Required | Required | Required |
| Complete analytical audit record | If later relied upon | Required | Required |
| Independent challenge | Proportionate/optional | Required | Required |
| Independent validation | Not mandatory unless special claim | Required | Required |
| Risk review | Relevant domains | All plausible Material domains | All domains with Critical-path analysis |
| Qualified human specialist | When legally/contractually required | When subject requires | Required where specialist judgment applies; AI-only validation prohibited |
| Human Principal approval | At applicable action gates | Material reliance/action gates | Explicit and non-delegable where `00_CORE` requires |

## Approved versus Proposed Parameters

Engagement-wide related-task aggregation and reclassification on Material change are adopted as design controls derived from anti-underclassification requirements. Numeric retry, delegation-depth, response-time, risk-appetite, or operational service thresholds remain **[PROPOSED—HP APPROVAL REQUIRED]** until separately approved.
