# Specialist Registry and Responsibilities

## Registry Purpose

The registry is a design-time control catalog describing eligible specialist capabilities and constraints. It is not an agent launcher. Registration does not grant access, authority, or permission.

## Registry Schema

```text
specialist_id
registry_version
division
role_name
capabilities
prohibited_capabilities
subject_qualifications
jurisdiction_qualifications
maximum_permission_tier
permitted_data_classes
permitted_engagements
permitted_tool_classes
validation_eligibility
incompatible_roles
conflicts_and_restrictions
actor_identity_authority
model_provider_and_version
permitted_session_classes
qualification_record_refs
owner
approval_reference
status
effective_from
expires_at
```

Only a human registry owner authorized by the Human Principal may activate, change, or deactivate an entry. Specialists cannot self-register or expand their entry.

## Division Responsibilities

| Division | Primary responsibilities | Required challenge focus | Boundaries |
|---|---|---|---|
| `02_STRATEGY` | Strategic choices, positioning, portfolio, scenarios, value-creation logic | Strategic alternatives, competitive response, path dependence | Cannot approve strategy |
| `03_FINANCE` | Economics, forecasting, valuation, business cases, sensitivities | Input quality, model risk, downside, liquidity and capital exposure | Material models require independent reproduction |
| `04_COMMERCIAL` | Markets, customers, pricing, channels, pipeline, sales economics | Selection bias, willingness-to-pay evidence, channel conflict | Customer contact is Tier 4 |
| `05_MARKETING` | Segmentation, positioning, brand, demand and campaign hypotheses | Attribution, causality, audience harm, brand risk | Publication/activation is Tier 4 |
| `06_OPERATIONS` | Process, capacity, service, supply chain, implementation | Bottlenecks, failure modes, capability and change constraints | Production/system change requires approval |
| `07_RESEARCH` | Source discovery, evidence records, freshness, corroboration | Source incentives, contradictory evidence, missing populations | Discovery sources do not verify Material claims alone |
| `08_RISK_COMPLIANCE` | Risk taxonomy, control review, regulatory issue spotting | Legal/regulatory uncertainty, residual risk, control failure | Not a substitute for qualified legal advice; cannot self-validate |
| `09_DELIVERABLES` | Narrative, exhibits, consistency, decision-package assembly | Unsupported claims, distorted emphasis, audience clarity | Cannot change analytical conclusions for presentation |
| `10_CLIENTS` | Engagement metadata, scope, authorization, conflicts, lifecycle | Boundary mismatch, consent, retention, recipient authorization | No autonomous client contact or cross-engagement access |

## Selection Rules

The Registry Resolver must filter candidates in this order:

1. engagement and data eligibility;
2. permission-tier compatibility;
3. capability and qualification match;
4. conflict and incompatible-role exclusion;
5. validation independence;
6. jurisdiction suitability;
7. approved tool compatibility;
8. workload and deadline fit.

Failure at steps 1–6 is disqualifying. Efficiency cannot override eligibility.

## Validation Eligibility

A validator must satisfy every mechanically verifiable check in `DELEGATION_HANDOFF_AND_ASSURANCE.md`. Registry identity alone never proves independence. Sharing a division does not automatically defeat independence, but shared actor/session, hidden context lineage, authorship, untested assumptions, primary-supplied evidence metadata, or restatement fails the applicable check.

For Critical work, the registry must identify required qualified human review. If no eligible validator exists, the workflow enters `BLOCKED_VALIDATION`; it cannot substitute a primary specialist, challenger, synthesizer, alternate actor ID using the same session, or unqualified human.

## Registry Change Control

Registry changes require a change record, rationale, owner, review, Human Principal or authorized human approval, version, effective date, and affected workflow analysis. Active Material workflows must re-evaluate eligibility after a relevant registry change.
