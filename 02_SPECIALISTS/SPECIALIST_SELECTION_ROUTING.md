# Specialist Selection and Routing

## Canonical Rule

The Registry Resolver and Planner select specialists from the decision question and dependency graph. Availability, prestige, or broad relevance is not sufficient. The Orchestrator must not activate every practice by default.

## Routing Inputs

Every routing decision uses:

- engagement type and decision question;
- sector and jurisdiction;
- Material/Critical classification and risk domains;
- financial exposure and capital dependence;
- regulatory, legal, security, privacy, safety, and reputational exposure;
- technology, data, and AI dependence;
- operational and supply-chain complexity;
- workforce intensity and change impact;
- public-sector authority, funding, procurement, or citizen impact;
- evidence and strategic uncertainty;
- client-requested output and reliance audience;
- qualifications, conflicts, data eligibility, tools, capacity, and independence requirements.

## Team Classes

### Minimum team

Every engagement has one accountable primary practice, the Orchestrator coordination path, applicable source/security self-checks, and a defined human decision route. Material/Critical work also reserves eligible challenger, validator, risk-review, synthesis-integrity, and human-review capacity; these are assurance roles, not automatic practice activations.

### Mandatory specialists

| Trigger | Required practice or capability |
|---|---|
| Enterprise strategy, portfolio, market entry/exit, partnership, transaction thesis | `SP_STRATEGY_CORP_DEV` |
| Valuation, financing, credit, capital, liquidity, Material economics | `SP_FINANCE_VALUATION_CAPITAL` |
| Market size, demand, competitor behavior, customer evidence is decision-critical | `SP_MARKET_COMPETITIVE_INTELLIGENCE` |
| Process, capacity, supply, service, resilience, safety, or implementation feasibility | `SP_OPERATIONS_SUPPLY_CHAIN` |
| Technology, data, AI, integration, vendor, cyber, privacy, or technical feasibility | `SP_TECHNOLOGY_DATA_AI` |
| Pricing, channel, sales, marketing, acquisition, retention, or monetization | `SP_COMMERCIAL_GROWTH` |
| Regulatory regime, policy, jurisdiction, license, filing, rights, or obligations | `SP_REGULATORY_POLICY`; qualified human review where required |
| Government client, public authority/funds, procurement, policy, or citizen impact | `SP_PUBLIC_SECTOR_INSTITUTIONAL` |
| One-FTE impact, organization design, workforce data, roles, incentives, or adoption | `SP_ORGANIZATION_WORKFORCE_CHANGE` |
| Material forecast, optimization, simulation, causal claim, experiment, or complex decision model | `TC_QUANTITATIVE_DECISION_SCIENCE` attached to a primary practice |
| Material recommendation requires baseline, KPI, target, attribution, realized-benefit, or post-implementation measurement | Measurement/KPI/Benefits Realization sub-capability of `TC_QUANTITATIVE_DECISION_SCIENCE`, attached to one accountable primary practice |

### Optional specialists

An optional specialist may be added only when a bounded question has material expected decision value, cannot be answered by the current team without a qualification or domain gap, and does not duplicate an assigned question. The routing record states the marginal value, scope, deliverable, cost/capacity effect, and dependency.

### Escalation-triggered specialists

The Orchestrator reassesses routing after any Material change. A new practice or qualified overlay becomes mandatory when evidence reveals a new jurisdiction, threshold, regulated activity, public authority, sensitive-data flow, operating constraint, workforce effect, technical dependency, or decision-reversing model requirement.

## Routing Algorithm

1. Verify engagement, authority, data, tier, and conflict prerequisites.
2. Identify the single primary decision question and accountable practice.
3. Decompose decision-critical claims and feasibility dependencies.
4. Apply mandatory trigger rules.
5. Attach qualification overlays only where scope requires them.
6. Determine whether Material outcome measurement, benefit attribution, or post-implementation review is required and assign one accountable domain owner per KPI.
7. Reserve downstream assurance capacity before delegation.
8. Exclude conflicts, incompatible roles, ineligible data/tool access, and insufficient qualifications.
9. Evaluate optional additions by marginal decision value and duplication risk.
10. Issue exact handoffs and obtain recipient acceptance.
11. Re-route only through a versioned decision with dependency impact and audit linkage.

Failure of eligibility through jurisdiction suitability is disqualifying under the certified registry. Capacity cannot cure ineligibility.

## Anti-Duplication Rules

1. Each work-item question has one accountable primary owner.
2. A second specialist must have a distinct input, feasibility, challenge, validation, or risk question.
3. Same question plus same evidence plus same method is duplicate work unless independent reproduction is explicitly required.
4. Shared evidence is referenced by exact record; it is not rediscovered unless freshness, independence, or contrary-evidence search requires it.
5. Shared source lineage counts once for corroboration.
6. Metrics and definitions use one engagement data dictionary.
7. Each KPI and benefit has one accountable domain owner; cross-specialist dependencies cannot create duplicate ownership or double counting.
8. Downstream work consumes versioned inputs and states validation status.
9. Probable duplicates are reviewed, not silently merged.
10. Client content is never compared across engagements for duplicate detection.
11. Optional activation is removed when its question becomes immaterial or fully covered.

## Common Engagement Profiles

| Engagement | Initial team | Common triggers |
|---|---|---|
| Market entry | Strategy + Market | Commercial, Finance, Operations, Regulatory, Public Sector |
| Credit or valuation | Finance | Market, Operations, Strategy, quantitative support |
| AI transformation | Technology + accountable business practice | Organization, Operations, Commercial, Finance, Regulatory |
| Operational transformation | Operations | Organization, Finance, Technology, Regulatory |
| Public-sector strategy | Public Sector + Regulatory | Finance, Operations, Technology, Organization |
| Pricing/growth | Commercial + Market | Finance, Operations, quantitative support |
| Business plan | Strategy + Market + Finance | Commercial, Operations, Technology, Regulatory |

Profiles are starting hypotheses, not authority. The actual routing record controls.
