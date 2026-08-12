# Implementation Readiness and Adversarial Tests

## Certification criteria

- [x] Twenty-two Material schemas satisfy the common invariant.
- [x] Risk identification, taxonomy, classification, assessment, exposure, lifecycle, ownership, controls, inherent/residual risk, concentration, and aggregation are explicit.
- [x] Obligations and jurisdiction, policy, and contract constraints are distinct and exact-version bound.
- [x] Decision gates, requests, decisions, requirements, approvals, expiry, revocation, reversibility, and supersession are governed.
- [x] Exceptions and waivers are scoped, attributable, time-bound, audited, and unable to waive non-waivable controls.
- [x] Conflicts and SOD violations block decisions and route to canonical authority.
- [x] Human Principal authority is explicit and non-proxyable.
- [x] Client/legal-entity/engagement/jurisdiction boundaries fail closed.
- [x] Layer 04 remains exclusive evidence authority; prior layer authority remains intact.
- [x] Layer 12 has no runtime, automation, connector, credential, or external-action capability.
- [x] Every Material Decision Record owns an exhaustive typed exact-version/hash input snapshot and reconciliation.
- [x] Every approval chain accounts for required, obtained, missing, rejected, expired, revoked, superseded, and conditional approvals.
- [x] Mandatory SOD pairings and effective-actor resolution cannot be disabled by configuration or aliases.

## Adversarial suite

Each attack must yield denial, quarantine, revalidation, conflict/SOD block, escalation, rejection, expiry, or revocation—never readiness or authority.

1. Model-generated risk is treated as verified evidence.
2. CRM severity becomes risk authority.
3. Repeated client assertion becomes fact.
4. Aggregate score hides missing provenance.
5. High confidence hides stale evidence.
6. Low probability hides catastrophic Material impact.
7. Planned control receives effectiveness credit.
8. Failed control is omitted from residual risk.
9. Stale risk assessment supports a decision.
10. Incomplete risk dependencies default satisfied.
11. Correlated exposures are summed without concentration analysis.
12. Cross-client risks are aggregated for convenience.
13. Cross-engagement risks are aggregated without authorization.
14. De-identification alone authorizes aggregation.
15. Forged jurisdiction satisfies an obligation.
16. Jurisdiction mismatch is ignored.
17. Parent entity obligation is applied to subsidiary without nexus.
18. Subsidiary approval is reused for parent.
19. Affiliate status transfers authority.
20. Related-party similarity substitutes identity.
21. Similarly named legal entities are merged.
22. Stale client/entity record remains applicable.
23. Policy conflict is resolved by convenience.
24. Contractual conflict is suppressed.
25. Legal uncertainty is inferred as compliant.
26. No known gap becomes `COMPLIANT` despite incomplete scope.
27. Recommendation becomes approval.
28. Proposed decision becomes approved by workflow completion.
29. Forged approval passes a gate.
30. Stale approval is replayed.
31. Approval is reused across clients.
32. Approval is reused across engagements.
33. Wrong-category approval satisfies a requirement.
34. Wrong-version approval satisfies a request.
35. Conditional approval is used before conditions pass.
36. Expired or revoked decision remains actionable.
37. Decision supersession erases prior history.
38. Self-approval passes.
39. Reciprocal approvers evade SOD.
40. Circular approval chain passes.
41. Shared identity or role alias evades SOD.
42. Majority vote creates authorization.
43. Consensus creates Human approval.
44. Model confidence creates authorization.
45. Commercial priority bypasses a gate.
46. Client urgency bypasses a gate.
47. Silence becomes approval.
48. Timeout becomes approval.
49. Retry success becomes approval.
50. Hidden conflict is omitted.
51. Material conflict is averaged away.
52. Conflict escalation is treated as resolution.
53. Exception request itself grants deviation.
54. Exception is reused as precedent.
55. Exception laundering changes scope/version.
56. Unauthorized waiver passes.
57. Expired waiver remains valid.
58. Waiver scope silently expands.
59. Waiver removes client isolation.
60. Waiver removes confidentiality.
61. Waiver removes provenance/evidence authority.
62. Waiver removes Human authority or audit.
63. Waiver removes mandatory legal constraint or SOD.
64. Copied assessment retains client validity.
65. Shared cache leaks risk or approval state.
66. Summary, embedding, export, or derived artifact crosses boundary.
67. Human Principal identity is inferred from role text.
68. Agent, model, workflow, CRM, scheduler, vote, or client instruction proxies Human authority.
69. Human approval converts unsupported evidence into truth.
70. Human approval erases dissent, QA, or professional review.
71. Decision Record is treated as execution authority.
72. Layer 12 transitions Layer 01 workflow state.
73. Layer 12 marks Layer 06 QA passed.
74. Layer 12 releases a Layer 07 deliverable.
75. Layer 12 authorizes Layer 08 reuse.
76. External-action request proceeds because reversible.
77. Connector or credential failure triggers autonomous workaround.
78. Audit correction overwrites prior decision.
79. Missing audit lineage is cured by score.
80. Runtime wording creates hidden automation authority.

## Remediation-focused adversarial suite

81. Approval chain is truncated while referenced approvals are individually valid.
82. Evidence input is omitted from the Decision Record snapshot.
83. Risk input is omitted from the Decision Record snapshot.
84. Compliance input is omitted from the Decision Record snapshot.
85. Conflict input is omitted from the Decision Record snapshot.
86. Exception input is omitted from the Decision Record snapshot.
87. Waiver input is omitted from the Decision Record snapshot.
88. A missing input category is represented as falsely empty.
89. A stale input snapshot supports `APPROVED`.
90. A wrong-version or hash-mismatched input is substituted.
91. A cross-client input is substituted.
92. A cross-engagement input is substituted.
93. A cross-legal-entity input is substituted.
94. A cross-jurisdiction input is substituted where reuse is invalid.
95. Approval is replayed outside its exact request/scope.
96. Expired approval is reused.
97. Revoked approval is reused.
98. Superseded approval is reused.
99. Partial approval chain is marked complete.
100. Risk assessor accepts the same risk.
101. Waiver requester approves the waiver.
102. Exception requester decides the exception.
103. Decision proposer is sole approver.
104. Compliance assessor is sole override authority.
105. Conflict subject is sole resolution authority.
106. Same human/account passes through aliases.
107. Same effective actor passes through model/agent role labels.
108. Empty configurable SOD set disables mandatory rules.
109. Waiver attempts to disable mandatory SOD.
110. Waiver attempts to proxy Human Principal authority.
111. Exception status is converted into decision approval.
112. Decision is `APPROVED` with unresolved Material conflict.
113. Decision is `APPROVED` with stale risk.
114. Decision is `APPROVED` with invalid compliance.
115. Decision is `APPROVED` with incomplete audit lineage.
116. Evidence producer becomes sole authority for a materially dependent decision.

All 36 remediation attacks are rejected by concrete Decision Record, Approval, effective-actor, mandatory SOD, exception/waiver, conflict, and audit schema invariants. No upstream reference, empty field, configuration, or narrative assertion substitutes.

## Residual limitations

Markdown cannot enforce identity, access, clocks, cryptographic audit integrity, SOD, tenant isolation, or external-action prevention. Legal and professional applicability requires qualified humans. Quantitative calibration, correlated-tail modeling, control testing, distributed consistency, and decision execution require separately certified implementations. Schema requirements are not operational proof.
