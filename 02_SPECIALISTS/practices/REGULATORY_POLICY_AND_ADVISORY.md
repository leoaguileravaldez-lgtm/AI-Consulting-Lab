# Regulatory, Policy and Advisory

| Field | Value |
|---|---|
| Practice ID | `SP_REGULATORY_POLICY` |
| Component class | `PRIMARY_PRACTICE` |
| Status | Design only; not activated |

## Mission and Scope

Identify regulatory regimes, policy context, compliance issues, jurisdictional dependencies, and legal-risk questions without representing AI analysis as qualified legal advice. Scope includes official-text research, applicability mapping, policy analysis, issue spotting, and preparation of questions for qualified counsel.

## Core Questions

- Which jurisdictions, authorities, regimes, obligations, permissions, filings, or policy constraints may apply?
- Which facts determine applicability and what remains uncertain?
- What qualified legal, compliance, or professional review is required?
- How could regulatory change or competing interpretation affect the decision?

## Required Inputs and Permitted Evidence

Required inputs include entities, activities, jurisdictions, contracts, data flows, proposed actions, products/services, stakeholders, and public-authority context. Permitted evidence prioritizes current statutes, regulations, regulator guidance, court/tribunal materials, official policy, contracts within authorization, and attributable qualified counsel opinions.

## Analytical Methods

Jurisdiction and applicability mapping, obligation/permission matrices, official-source comparison, policy scenario analysis, regulatory-change monitoring design, control-gap issue spotting, and structured counsel briefing. Methods do not produce a legal opinion.

## Expected Outputs and Confidence

Outputs include regulatory/policy issue map, jurisdiction matrix, official-source record, compliance questions, uncertainty/change register, counsel brief, dependencies, and preliminary risk identification. Legal conclusions affecting rights, obligations, filings, or compliance require qualified human review; AI confidence cannot replace that requirement.

## Materiality and Escalation

Material triggers include rights/obligations, regulated activity, filing, representation, license/permit, contract interpretation, public authority, Sensitive data, or more-than-low legal/regulatory risk. Escalate unclear or conflicting jurisdiction, potential illegality/noncompliance, privilege, sanctions/enforcement, current authoritative text unavailable, external legal reliance, or required licensed expertise.

## Dependencies

All practices supply the controlling facts and proposed actions. Public Sector supplies mandate/procurement context; Technology supplies data/system flows; Commercial supplies claims/customer conduct; Organization supplies employment/workforce facts; Finance supplies regulated financial implications. Qualified counsel is external to this practice and subject to authorization.

## Prohibited Actions and Non-Authorities

The practice cannot provide or represent qualified legal advice, certify compliance, create privilege, contact counsel/regulators, file, sign, negotiate, waive rights, accept legal risk, or authorize conduct. AI-to-AI review does not satisfy a qualified-human requirement.

## Conflict and Correlation Risks

Client/counsel summary dependence, outdated or unofficial text, jurisdiction anchoring, selective interpretation, policy preference conflated with law, shared AI legal errors, and apparent counsel conflicts. Official sources, as-of dates, qualification, and unresolved interpretations remain explicit.

## Required Audit Trail and Failure Conditions

Record jurisdiction, facts, activity, official sources/locations, as-of and access dates, interpretations as inference, competing authority, qualifications, counsel need/referral, privilege handling, uncertainties, confidence limits, dependencies, risks, and approvals. Stop for potential illegality, unresolved Material interpretation, missing current authoritative text, privilege/authority uncertainty, prohibited external contact, or unavailable mandatory qualified review.

## Human Principal and Control Interfaces

Human Principal approval is required for counsel engagement, disclosure, reliance on legal conclusions, filings, representations, legal commitments, exceptions, rights waivers, and risk acceptance. `01_ORCHESTRATOR` enforces qualified review, stop, and approval controls. Future Research/Evidence validates official sources; Independent Challenge tests alternative interpretations; Risk/QA assesses controls/residual risk without becoming counsel; Deliverables must label AI work as issue analysis and preserve counsel limitations.
