# Source Validation

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

## Standard

This policy applies to research, analysis, models, recommendations, and deliverables. Materiality definitions in `OPERATING_PRINCIPLES.md` apply. Every Material claim must be traceable to evidence fit for that claim; Critical claims require the strongest available authoritative evidence and independent validation.

## Source-Quality Hierarchy

Use the most direct, authoritative, methodologically sound, independent, and current source available for the specific claim:

1. original records and primary evidence, including verified transaction data, contracts, direct observations, and official datasets;
2. current law, regulation, court, regulator, and government statistical materials;
3. audited financial statements, securities filings, and official company disclosures;
4. peer-reviewed research and transparent original studies;
5. reputable multilateral, academic, standards, and professional institutions;
6. reputable industry research and established news organizations with transparent sourcing and methods;
7. secondary summaries and attributable expert commentary for context or discovery;
8. search snippets, aggregators, anonymous claims, social media, and AI-generated summaries, which never independently verify a Material claim.

Rank does not override fitness. Client-provided data must be labeled as such and tested for completeness, definitions, lineage, reconciliation, bias, and reliability. A secondary source cannot be represented as primary, and a source cannot be cited unless accessed and confirmed to support the claim.

## Required Source Record and Validation

For every Material claim, record the claim and type; issuer/owner; title; publication and reporting dates; access timestamp; stable URL, repository location, or identifier; page/table/section/cell where practical; primary/secondary classification; relevant geography, population, definitions, units, methods, transformations, limitations, sponsorship, and conflicts.

Assess and record:

- **Authority:** competence and accountability of the issuer;
- **Directness:** original evidence versus repetition;
- **Method:** transparency and fitness of sampling, definitions, calculations, exclusions, and limitations;
- **Currency:** freshness for the decision and whether superseded;
- **Relevance:** match to geography, population, time, metric, and decision;
- **Independence:** incentives, sponsorship, and conflicts;
- **Consistency:** agreement or explained disagreement with credible evidence;
- **Integrity:** verifiable provenance and absence of alteration or misquotation.

Material claims must be corroborated by at least one independent source unless a single source is the sole authoritative record. The sole-source condition and resulting limitation must be documented. Critical claims require two independent authoritative sources where they exist; otherwise the Human Principal must be notified and confidence cannot exceed Medium without a documented rationale from the qualified validator.

## Source Freshness

Every Material claim must have an as-of date and a documented freshness assessment at the time of reliance or delivery.

- Current law, regulation, regulator guidance, sanctions, rates, prices, market conditions, public officials, product specifications, and other changeable facts must be checked against an authoritative current source within 24 hours before external delivery or action.
- Company filings and government statistics must use the latest available release appropriate to the reporting period and must be rechecked within 30 days before external delivery.
- Other sources older than 12 months require an explicit continued-relevance rationale unless the claim is historical or the field changes slowly.
- A source known to be corrected, withdrawn, superseded, or materially outdated must not support a current claim without prominent qualification.

If a source can change or disappear, preserve an authorized snapshot, stable identifier, retrieval date, or integrity hash where lawful and practical. Delivery must pause if a decision-relevant freshness check cannot be completed.

## Conflicts and Gaps

When reliable sources materially conflict, do not conceal, average, or selectively choose among them without explanation. Compare definitions, dates, scope, methods, incentives, and lineage; state each credible position and its implications; document any resolution; preserve unresolved uncertainty; and reduce confidence. Unresolved conflict affecting a Material or Critical decision is a mandatory stop condition requiring Human Principal escalation.

When direct evidence is unavailable, label the result Assumption, Estimate, or Hypothesis; record the proxy, method, range, sensitivity, and evidence needed for validation. Never invent a value. An inadequately sourced Material claim must be excluded, prominently qualified, or stopped.

Regulatory and legal claims must use current official text or authoritative materials, specify jurisdiction and as-of date, and receive the validation required by `QUALITY_STANDARDS.md` before decision reliance or delivery.

## Confidence Framework

Confidence measures evidentiary support for an analytical conclusion or recommendation, not certainty that an outcome will occur. Assign confidence at the claim/conclusion level and state the rationale. The overall recommendation cannot exceed the confidence of a decision-critical supporting claim.

| Level | Mandatory evidentiary requirements |
|---|---|
| High | Decision-critical claims use current primary or authoritative evidence; Material claims are independently corroborated; sources and definitions are consistent or conflicts are resolved; key assumptions are verified; methodology and Material calculations are reproducible; sensitivity does not reverse the conclusion across credible ranges; mandatory independent validation passed with no unresolved Material issue |
| Medium | Evidence is credible and substantially relevant but includes limited proxies, sampling constraints, partially verified assumptions, or non-decision-critical conflict; methodology is documented and Material calculations are reproducible; sensitivity and alternative explanations are assessed; independent validation passed with limitations explicitly recorded |
| Low | Evidence is sparse, indirect, stale, single-source without sole-authority status, materially conflicting, or dependent on unverified decision-critical assumptions; methodology or reproduction is constrained; sensitivity may reverse the conclusion; or required validation is incomplete under an approved exception |

High confidence is prohibited when a decision-critical source is stale, a Material conflict remains unresolved, a key assumption is unverified, Material calculations are not reproducible, or independent validation is incomplete. Confidence must be lowered when new evidence weakens support and reassessed whenever evidence, assumptions, methodology, or decision context changes materially.

## Revision History

| Version | Date | Change | Approval |
|---|---|---|---|
| 0.1.0-draft | 2026-08-10 | Initial source framework | Not approved |
| 0.2.0-draft | 2026-08-10 | Added corroboration, freshness, conflict, and confidence controls | Approved for Design by Leonel Aguilera Valdez; operational use not authorized |
