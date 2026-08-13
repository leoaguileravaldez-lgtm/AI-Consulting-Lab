# Consulting Engagement Orchestration and Evidence Governance

NORMATIVE_SOURCE: `CANONICAL_MODEL.json`  
NORMATIVE_SOURCE_VERSION: `1.0.2`  
DERIVATION_TYPE: human-readable projection

## Responsibility

Layer 15 governs the case structure through which an already authorized input becomes a controlled consulting engagement. It coordinates mandate, problem structure, workstreams, routing requirements, evidence lineage, analytical records, validation, challenge, synthesis, recommendation, decision, delivery packaging, non-executing handoff, termination, outcome observation, and audit. It does not perform specialist work, external delivery, implementation, investment, or runtime enforcement.

The canonical path is:

`AUTHORIZED_INPUT → ENGAGEMENT → MANDATE → DECISION_PROBLEM → WORKSTREAMS → ROUTING → EVIDENCE_REQUIREMENTS → EVIDENCE → CLAIMS → ANALYSIS → FINDINGS → CROSS_VALIDATION → CHALLENGE → SYNTHESIS → CONCLUSION → INDEPENDENT_VALIDATION → RECOMMENDATION → DECISION → DELIVERY_PACKAGE → HANDOFF/OUTCOME → AUDIT`.

Authorized termination is a terminal branch after admission, mandate, problem structuring, findings, or independent validation. It invalidates downstream progression; a subordinate workstream cannot reverse it.

## Authority

Layer 15 consumes exact, versioned predecessor references and creates no predecessor authority. Layers 01 and 03 retain canonical Engagement ownership; Layers 01/02 retain specialist assignment and output ownership; Layer 04 retains evidence/claim source authority; Layer 05 retains challenge independence; Layer 06 retains QA/risk; Layer 07 retains deliverable governance; Layers 12–14 retain decision, authorization, and protected-action controls.

An engagement has one immutable `EngagementId` and exactly one authoritative current `MANDATE_CONTRACT`. Workstreams are subordinate children and cannot enlarge scope or delegate authority. Material and Critical findings require producer-distinct validation, challenge, and independent review. Independent reviewers may issue verdicts but cannot create engagement authority.

## Evidence and decision semantics

Evidence source type, identity, provenance, freshness, reliability, limitations, and verification are explicit. `CLIENT_PROVIDED`, `ASSUMPTION`, and `UNVERIFIED_ASSERTION` remain non-factual unless independently verified. `INTERNAL_CALCULATION`, `MODEL_OUTPUT`, and `EXPERT_JUDGMENT` remain derived evidence and cannot masquerade as primary sources.

Claims bind evidence and contradictions; analysis binds methods and inputs; findings keep analytical confidence distinct from decision attractiveness. A typed Conclusion preserves exact evidence, claim, analysis, finding, cross-validation, Challenge, and synthesis lineage without creating evidence or authority. `ConclusionId` is scoped by engagement, mandate, use domain, generation, proposition, finding set, and claim lineage. Freshness, lifecycle state, authority domain, workstream scope, and provenance remain separately explicit. Independent validation and Recommendation bind the same exact current Conclusion IDs/versions and freshness bases; substitution denies. A failed verdict may support rejection, deferral, restructuring, insufficient-evidence disposition, or termination; it cannot be laundered into approved delivery.

Handoffs are scoped contracts for future owners. They perform no action and cannot retroactively upgrade analysis. Audit is strictly descendant evidence.
