# Quality Standards

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

## Scope and Completion Standard

This policy applies to all analyses, models, research outputs, recommendations, and deliverables. Definitions and Material/Critical thresholds in `OPERATING_PRINCIPLES.md` are controlling.

A work product is complete only when it is accurate, traceable, transparent, reproducible, balanced, decision-useful, secure, independently validated where required, and properly approved. Facts, assumptions, estimates, hypotheses, and recommendations must remain visibly distinct. Conflicting evidence, uncertainty, alternatives, limitations, and downside cases must not be hidden.

## Review Classification

| Level | Scope | Mandatory control |
|---|---|---|
| Routine | Work that meets no Material or Critical trigger | Author self-check; source and security controls still apply |
| Material | Any Material trigger in `OPERATING_PRINCIPLES.md` | Complete audit record and independent validation before reliance or delivery |
| Critical | Any Critical trigger | Material controls plus qualified human specialist review where the subject requires licensed or specialist judgment and explicit Human Principal approval |

The preparer proposes the level, but an independent reviewer or Human Principal must confirm Material and Critical classifications. Uncertainty requires the higher level. Work may be reclassified upward at any time; downstream reliance must pause until added controls are complete.

## Quantitative Reproducibility

Every Material or Critical model, calculation, forecast, or estimate must record:

- purpose, scope, preparer, version, as-of date, unit, currency, geography, population, and period;
- each input's value, definition, source, date, and validation status;
- formulas, methodology, calculation sequence, and software/tool version where relevant;
- all assumptions, owners if known, rationale, and whether verified;
- cleaning, exclusions, mappings, adjustments, transformations, allocations, and imputation;
- base and downside scenario logic, plus upside when decision-useful;
- sensitivity analysis for inputs capable of changing the recommendation, classification, or expected outcome materially;
- reconciliations, control totals, boundary tests, sign/unit/denominator checks, and missing-data treatment;
- rounding, materiality thresholds, limitations, and confidence level;
- exact output version and sufficient authorized artifacts or instructions for reproduction.

A qualified reviewer must be able to recreate material outputs from recorded inputs and instructions within ordinary rounding tolerance. If proprietary tools or unavailable data prevent reproduction, the limitation must be recorded and the work cannot receive High confidence.

## Qualitative and Causal Quality

Material qualitative conclusions must document evidence-selection criteria, coding or synthesis method where applicable, contrary evidence, representativeness limits, and the chain from evidence to conclusion. Anecdotes cannot be generalized without support. Causal claims must address timing, mechanism, confounding, selection effects, and credible alternatives; otherwise they must be labeled association or hypothesis.

## Independent Validation

Independent validation is mandatory before a Material or Critical conclusion, financial model, regulatory claim, or major recommendation is relied upon, approved, or delivered.

The validator must:

1. be identifiable, competent for the task, free of relevant conflicts, and different from the preparer;
2. verify material claims against underlying sources and test source fitness and freshness;
3. reproduce material calculations or use a genuinely independent calculation/method;
4. test assumptions, sensitivities, downside and failure cases, alternative explanations, and contrary evidence;
5. evaluate economics, feasibility, implementation constraints, regulatory exposure, and confidence;
6. record procedures, findings, corrections, unresolved issues, conclusion, identity, and timestamp.

Financial models require formula/input checks and independent reproduction of decision-driving outputs. Regulatory or legal claims require current official authority, jurisdiction and as-of date, plus review by a qualified human when they may affect rights, obligations, filings, or compliance. Critical work cannot be validated solely by AI. The preparer cannot validate or approve their own Material or Critical work.

If independent validation is genuinely impracticable, the work must be labeled unvalidated, confidence cannot exceed Low, and the Human Principal must approve a documented exception before internal reliance. No exception permits external delivery of an unvalidated Critical conclusion or substitutes AI for required legal, regulatory, audit, or licensed professional review.

## Mandatory Audit Record

Each Material or Critical analytical process must create or update an authorized engagement audit record. Routine work must create a record if it is later used by Material or Critical work.

The record must contain:

- unique record identifier in the form `ACL-[ENGAGEMENT]-[YYYYMMDD]-[SEQUENCE]` or an approved equivalent;
- ISO 8601 timestamp with time zone for creation and each material event;
- identified human, agent, model, tool, or other actor and its permission tier;
- task, decision question, scope, classification, and engagement identifier;
- sources used, source locations, access dates, and source-validation results;
- assumptions, estimates, hypotheses, owners if known, and validation status;
- methodology, formulas, transformations, scenario logic, and limitations;
- inputs and outputs, including artifact names, versions, and integrity hash where supported;
- confidence level and rationale;
- independent validation performed, validator, timestamp, findings, and disposition;
- human approval status, approver, scope, conditions, timestamp, and linked artifact/version;
- exceptions requested or granted, rationale, compensating controls, approver, and expiration;
- final disposition: draft, rejected, revised, approved-internal, approved-external, executed, archived, or closed.

Records must be chronological, attributable, access-controlled, and append-only or tamper-evident where the approved system supports it. Corrections must preserve the original entry and explain the change. Records must remain linked to the engagement throughout retention, archival, and closure. Secrets and unnecessary Sensitive information must not be copied into audit records.

## Release Checklist

Before reliance, approval, or delivery, confirm and record:

- [ ] Scope, audience, decision question, classification, and as-of date are stated.
- [ ] Claim types are labeled and Material claims are traceable.
- [ ] Sources satisfy `SOURCE_VALIDATION.md`; conflicts and freshness are addressed.
- [ ] Assumptions, methodology, calculations, scenarios, sensitivity, and limitations are documented.
- [ ] Correlation is not presented as causation without adequate support.
- [ ] Major-recommendation fields in `OPERATING_PRINCIPLES.md` are complete.
- [ ] Required independent validation is complete and independent.
- [ ] Confidence is assigned under the formal rubric and unresolved issues are disclosed.
- [ ] Security, engagement segregation, conflict, permission-tier, and approval controls are satisfied.
- [ ] Audit record is complete and linked to the exact artifact/version.
- [ ] Factual, numerical, logical, and presentation checks passed.

Failed checks must be corrected or handled under `APPROVAL_POLICY.md`. Accuracy and defensibility take precedence over speed.

## Revision History

| Version | Date | Change | Approval |
|---|---|---|---|
| 0.1.0-draft | 2026-08-10 | Initial quality framework | Not approved |
| 0.2.0-draft | 2026-08-10 | Added classification, reproducibility, validation, and audit controls | Approved for Design by Leonel Aguilera Valdez; operational use not authorized |
