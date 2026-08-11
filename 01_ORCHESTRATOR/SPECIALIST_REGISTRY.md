# Specialist Registry and Responsibilities

## Registry Purpose

The registry is a design-time control catalog describing eligible specialist capabilities and constraints. It is not an agent launcher. Registration does not grant access, authority, or permission.

## Canonical Taxonomy

Repository modules, organizational practices, logical capabilities, workflow roles, attributable actors, and runtime sessions are distinct concepts. They must use separate identifiers and must not be substituted for one another.

| Identifier | Meaning | Authority effect |
|---|---|---|
| `module_id` | Design-package or repository-module ownership, such as the future `02_SPECIALISTS` package | None; a filesystem location is not eligibility or authority |
| `practice_id` | Stable organizational subject-matter practice | None; practice membership does not assign a workflow role |
| `capability_id` | Bounded subject, method, or task capability | None; capability does not grant access, permission, or approval authority |
| `role_type` | Canonical workflow responsibility defined in the Orchestrator record and state model | None without a current engagement-scoped role assignment |
| `actor_id` | Stable attributable human, AI, or service identity | Identity alone does not grant a role, permission, qualification, or independence |
| `session_id` | Particular runtime or execution context linked to an actor | A new session does not create a new actor or cure a conflict or qualification defect |

Top-level numeric prefixes identify architecture packages or phases only. Specialist practices and capabilities use stable nonnumeric identifiers. The prior design labels `02_STRATEGY` through `10_CLIENTS` are superseded as registry taxonomy and must not be interpreted as filesystem paths, authority domains, or future runtime divisions. Historical references, if any, remain preserved through versioned supersession records rather than silent renaming.

## Registry Schema

```text
specialist_id
registry_version
module_id
component_class
practice_id
capability_ids
role_types_eligible
actor_id
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

## Component Classes and Ownership

`component_class` is one of `PRIMARY_PRACTICE`, `TRANSVERSAL_CAPABILITY`, `ASSURANCE_CONTROL`, `DELIVERABLE_SUPPORT`, or `CLIENT_LIFECYCLE_CONTROL`. Class determines architectural ownership and incompatible-role checks; it does not grant authority.

| Component class | Design ownership | Responsibility | Mandatory boundary |
|---|---|---|---|
| `PRIMARY_PRACTICE` | Future `02_SPECIALISTS` | Produces bounded subject-matter analysis and recommendations | Cannot approve, self-validate Material/Critical work, accept residual risk, or administer its own registry, qualification, or permission |
| `TRANSVERSAL_CAPABILITY` | Future `02_SPECIALISTS` | Supplies shared analytical methods to a named primary practice | Cannot own the domain recommendation or independently validate its own Material output |
| `ASSURANCE_CONTROL` | Future independent assurance module; coordinated by `01_ORCHESTRATOR` | Research/evidence validation, independent challenge, analytical reproduction, and risk/QA review | Must remain separate from affected primary work and synthesis as required by `00_CORE` and assurance rules |
| `DELIVERABLE_SUPPORT` | Future independent deliverables module; coordinated by `01_ORCHESTRATOR` | Assembles narratives, exhibits, and exact-version decision packages from validated records | Cannot change analytical conclusions, validated figures, confidence, dissent, risk, or approval status |
| `CLIENT_LIFECYCLE_CONTROL` | Future independent client-lifecycle module; coordinated by `01_ORCHESTRATOR` | Maintains engagement metadata, scope, conflicts, authorization, recipient, retention, and closure inputs | Cannot contact clients autonomously, broaden scope, infer consent, waive segregation, or approve delivery |

The Orchestrator owns workflow coordination, state and guard enforcement, routing, and synthesis interfaces. It does not become the substantive producer, challenger, validator, risk acceptor, deliverable approver, or client authority for any component class.

## Primary Practice Catalog

The future `02_SPECIALISTS` package is the sole umbrella for primary practices and transversal analytical capabilities. This catalog defines design identifiers and boundaries only; it neither creates that package nor activates a specialist.

| Practice ID | Primary responsibilities | Required challenge focus | Boundaries |
|---|---|---|---|
| `SP_STRATEGY_CORP_DEV` | Strategic choices, positioning, portfolio, scenarios, corporate development, and value-creation logic | Strategic alternatives, competitive response, founder/client anchoring, path dependence | Cannot approve strategy or certify financial, legal, market, or operating feasibility |
| `SP_FINANCE_VALUATION_CAPITAL` | Economics, forecasting, valuation, business cases, capital, liquidity, and sensitivities | Input quality, model risk, downside, liquidity, and capital exposure | Material models require independent reproduction; cannot validate market demand |
| `SP_MARKET_COMPETITIVE_INTELLIGENCE` | Market definition, demand, customers, competitors, external trends, and market-entry evidence | Source lineage, selection bias, contrary demand, competitor response, and definition sensitivity | Primary market analysis is not formal independent evidence validation |
| `SP_OPERATIONS_SUPPLY_CHAIN` | Process, capacity, service, supply chain, resilience, and implementation feasibility | Bottlenecks, failure modes, capability, safety, and execution constraints | Production or system change requires separate authorization |
| `SP_TECHNOLOGY_DATA_AI` | Technology, data, AI, integration, architecture, vendor, security/privacy issue spotting, and technical feasibility | Necessity, simpler alternatives, evaluation validity, lock-in, reliability, and security boundaries | Cannot deploy, onboard vendors, accept security/privacy risk, or transfer data autonomously |
| `SP_COMMERCIAL_GROWTH` | Segmentation, positioning, pricing, channels, marketing, sales economics, retention, and growth | Willingness-to-pay, attribution, causality, channel conflict, and customer harm | Customer contact, publication, and activation are Tier 4 |
| `SP_REGULATORY_POLICY` | Regulatory research, policy analysis, compliance issue spotting, and legal-risk identification | Jurisdiction, current authority, conflicting interpretation, rights/obligations, and enforcement exposure | AI-generated work is not qualified legal advice; mandatory qualified human review applies where required |
| `SP_PUBLIC_SECTOR_INSTITUTIONAL` | Public value, institutional mandate, public finance context, procurement, stakeholders, policy implementation, and state capacity | Authority, legitimacy, equity, capture, political durability, and public consequences | Cannot represent a public body, lobby, commit public funds, or substitute private-sector value for public mandate |
| `SP_ORGANIZATION_WORKFORCE_CHANGE` | Organization design, workforce planning, governance, incentives, capability, adoption, and change | Leadership bias, employee impact, incentives, power, fairness, and adoption assumptions | Cannot make employment decisions, contact employees, or authorize workforce actions |

## Transversal Capability Catalog

| Capability ID | Responsibility | Mandatory boundary |
|---|---|---|
| `TC_QUANTITATIVE_DECISION_SCIENCE` | Statistics, forecasting, optimization, simulation, causal inference, experimental design, uncertainty, and structured decision methods | Must attach to a named primary practice owner; cannot own the substantive recommendation or serve as the independent validator of its own Material output |

Sector, jurisdiction, and regulated-profession expertise are qualification scopes or bounded capabilities unless a later Human Principal-approved registry version establishes a distinct practice. Qualified legal, accounting, tax, actuarial, engineering, clinical, or other professional review remains mandatory where law, contract, client obligation, or `00_CORE` requires it.

## Identity, Role, and Authority Separation

- A module or filesystem path cannot be used as evidence of eligibility, qualification, permission, independence, or authority.
- Practice membership cannot be used as a workflow-role assignment.
- Capability possession cannot be used as approval, access, or execution authority.
- Role eligibility cannot be used without a current engagement-scoped role assignment and permission record.
- Actor identity and session identity remain separate; changing labels or sessions cannot cure a conflict, authorship, qualification, or correlated-analysis defect.
- Validation eligibility remains an independently evaluated qualification and independence result, never a practice or component-class attribute alone.
- Every assignment must reference the exact registry, qualification, permission, engagement, actor, role, and session records required by the canonical schemas and transition guards.

## Selection Rules

The Registry Resolver must filter candidates in this order:

1. engagement and data eligibility;
2. permission-tier compatibility;
3. component-class and role-type compatibility;
4. capability and qualification match;
5. conflict and incompatible-role exclusion;
6. validation independence where applicable;
7. jurisdiction suitability;
8. approved tool compatibility;
9. workload and deadline fit.

Failure at steps 1–7 is disqualifying. Efficiency cannot override eligibility.

## Validation Eligibility

A validator must satisfy every mechanically verifiable check in `DELEGATION_HANDOFF_AND_ASSURANCE.md`. Registry identity, component class, practice membership, role label, actor label, or session label alone never proves independence. Sharing a practice does not automatically defeat independence, but shared actor/session, hidden context lineage, authorship, untested assumptions, primary-supplied evidence metadata, or restatement fails the applicable check.

For Critical work, the registry must identify required qualified human review. If no eligible validator exists, the workflow enters `BLOCKED_VALIDATION`; it cannot substitute a primary specialist, challenger, synthesizer, alternate actor ID using the same session, or unqualified human.

## Registry Change Control

Registry changes require a change record, rationale, owner, review, Human Principal or authorized human approval, version, effective date, and affected workflow analysis. Active Material workflows must re-evaluate eligibility after a relevant registry change.
