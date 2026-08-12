# Implementation Readiness and Adversarial Tests

## Certification criteria

- [x] Thirty-two Material schemas physically incorporate the common invariant.
- [x] Identity, authentication, role, permission, authority, access, delegation, approval, ownership, and Human authority remain distinct.
- [x] Effective actors resolve aliases, accounts, delegation, models/agents, and common control.
- [x] Authorization decisions own exhaustive exact-version/hash manifests and default deny.
- [x] Client/entity/engagement/jurisdiction, purpose, and confidentiality are exact and conjunctive.
- [x] Delegation is non-transitive by default; elevation is exceptional and expiring.
- [x] Layer 12 `MSOD-01`–`MSOD-08` and Layer 13 `ASOD-01`–`ASOD-08` are mandatory.
- [x] Emergency access is narrow, expiring, audited, reviewed, and non-waiving.
- [x] Revocation dominates caches, retries, copies, sessions, and prior decisions.
- [x] Human Principal authority is exact and non-proxyable.
- [x] No executable, credential, runtime, connector, agent, automation, or external-action capability exists.
- [x] Access Decision state/scope is a non-permissive exact projection of an authoritative Authorization Decision.
- [x] Every acting-on-behalf-of flow preserves the complete ordered originator-to-executor chain.
- [x] Deputy authorization is the least-privilege intersection across every chain member and non-waivable control.
- [x] Access audit preserves complete chain, intersection, and Authorization-to-Access reconciliation hashes.

## Adversarial suite

Every Material attack must produce denial, quarantine, revocation, revalidation, or escalation—never access or authority.

1. Unknown identity requests access.
2. Duplicate identities avoid resolution.
3. Alias collision selects convenient identity.
4. Shared account supplies Material attribution.
5. Terminated identity is reused.
6. Suspended identity is reused.
7. Compromised identity remains eligible.
8. Stale authentication assurance passes.
9. Authentication is treated as authorization.
10. Role assignment is treated as authority.
11. Permission becomes professional authority.
12. Admin privilege becomes business authority.
13. Cross-client object is accessed.
14. Same client, wrong engagement is accessed.
15. Cross-legal-entity access is inferred.
16. Parent/subsidiary relationship transfers access.
17. Affiliate relationship transfers access.
18. Related-party relationship transfers access.
19. Similar organization or person name resolves identity.
20. Purpose A is laundered into Purpose B.
21. Confidentiality is implicitly downgraded.
22. Object/action scope silently expands.
23. Delegation expands source scope.
24. Delegation becomes transitive by default.
25. Circular delegation passes.
26. Unknown identity receives delegation.
27. Delegation creates Human Principal proxy.
28. Privilege requester self-approves.
29. Elevation omits expiry or becomes permanent.
30. Service identity impersonates human.
31. Model identity impersonates human.
32. Agent identity impersonates human.
33. Machine identity creates its own grant.
34. Stale grant is reused.
35. Revoked grant is reused.
36. Expired grant is reused.
37. Superseded role assignment is reused.
38. Authorization decision is replayed.
39. Authorization is reused across clients.
40. Authorization is reused across engagements.
41. Authorization is reused across object versions.
42. Cache resurrects authorization.
43. Retry resurrects authorization.
44. Workflow state grants access.
45. CRM state grants access.
46. Client urgency grants access.
47. Commercial priority grants access.
48. Majority vote grants access.
49. Silence grants access.
50. Timeout grants access.
51. Missing policy fails open.
52. Missing role fails open.
53. Missing purpose fails open.
54. Missing jurisdiction fails open.
55. Missing confidentiality fails open.
56. Access grant bypasses Layer 12 SOD.
57. Same effective actor uses aliases.
58. Administrator self-grants.
59. Grantor and beneficiary collapse.
60. Break-glass launders privilege.
61. Break-glass becomes permanent.
62. Break-glass crosses clients.
63. Break-glass bypasses confidentiality.
64. Human Principal is proxied.
65. External-action authority leaks from approval.
66. Missing input-manifest category means empty.
67. Falsely empty permission or authority set passes.
68. Hash-mismatched object version passes.
69. Stale session/context is replayed.
70. Copied search result loses source boundary.
71. Shared embedding or cache loses engagement boundary.
72. Email domain establishes client-administrator authority.
73. Title or seniority establishes approval role.
74. Connector identity infers access from connectivity.
75. Audit subject becomes sole verifier.
76. Delegated actor approves its delegation.
77. Machine identity becomes its authorizer through another label.
78. Revocation propagation remains unknown but access continues.
79. Unknown clock/time validity passes.
80. Aggregate score conceals a Material identity or authority defect.

## Remediation-focused adversarial suite

81. Access Decision state conflicts with Authorization Decision state.
82. Forged `APPROVED` Access Decision references denied authorization.
83. Access Decision expands authoritative scope, purpose, or validity.
84. Access Decision references stale, revoked, expired, superseded, or hash-mismatched authorization.
85. Originating principal is omitted from a deputy flow.
86. Intermediary deputy is omitted from the ordered chain.
87. Unauthorized deputy relies on a privileged service identity.
88. Deputy chain amplifies privilege or object/action scope.
89. Deputy substitutes purpose, jurisdiction, client, entity, engagement, or confidentiality.
90. Alias/shared account substitutes a deputy effective actor.
91. Circular delegation is hidden in the deputy chain.
92. Stale, revoked, expired, or superseded deputy authority remains in the intersection.
93. Model/agent deputy impersonates Human Principal or inherits Human authority.
94. Audit lineage truncates originator, intermediary, executor, delegation, intersection, or decision hashes.

All remediation cases must fail through state reconciliation, chain reconciliation, effective-actor resolution, least-privilege intersection, or audit validation. No service possession or prose lineage is sufficient.

## Second-recertification remediation suite

95. Zero-deputy request substitutes a different final executor.
96. Zero-deputy request substitutes a different effective actor.
97. Deputy chain omits an intermediary present in the Access Request.
98. Authorization substitutes an intermediary.
99. Authorization reorders the requested deputy chain.
100. Authorization duplicates a deputy while preserving a plausible hash.
101. Declared and authorized deputy counts differ.
102. Authorization truncates the request chain.
103. Authorization expands the request chain or any authority/scope.
104. Authorization uses a stale request or Authorization version.
105. Request/Authorization ID, version, or hash mismatches.
106. Final executor differs between request and authorization.
107. Service, agent, or model appears as executor under a validated-empty chain.
108. Alias or shared account hides executor/deputy substitution.
109. Break-glass lacks a canonical Authorization Decision.
110. Break-glass references a mismatched, stale, revoked, expired, or superseded canonical decision.
111. Break-glass expands object/action, purpose, boundary, jurisdiction, confidentiality, or validity scope.
112. Break-glass changes effective actor or final executor.
113. Break-glass replaces or bypasses the canonical deputy chain.
114. Audit lineage omits a required request/identity/delegation/decision/executor stage.
115. Audit chain hashes validate while a canonical member is missing.
116. Reconstructed lineage conflicts with the authoritative Access Request.

All twenty-two attacks must fail through the zero-deputy identity equality manifest, typed request/Authorization comparison manifest, canonical/emergency intersection, unchanged actor/executor/chain validation, or lineage completeness manifest. Hash equality never substitutes for source-object and membership completeness.

## Third-recertification emergency remediation suite

117. Break-glass lacks its exact canonical Authorization Decision.
118. Canonical Authorization Decision is stale, revoked, expired, superseded, or unreconciled.
119. Authorization Decision ID, version, or hash is substituted.
120. Exact reconciled Access Decision is missing or substituted.
121. Authorization/Access reconciliation manifest is missing or mismatched.
122. A consumed Break-Glass Decision is replayed.
123. Two executions attempt to use the same authorization within its validity window.
124. Emergency-use identifier is duplicated within or across execution instances.
125. Execution-instance ID, version, or hash is altered.
126. Break-glass names another Break-Glass Decision as authority.
127. Emergency-to-emergency delegation or recursive nesting is attempted.
128. Generic authority source substitutes for typed canonical Authorization authority.
129. Audit lineage is assembled after use from independently valid records.
130. Access and Break-Glass Decisions disagree on state or authorization-chain identity.
131. Actor, effective actor, or final executor is substituted.
132. Deputy chain is substituted, truncated, reordered, or duplicated.
133. Emergency scope expands object/action.
134. Emergency scope expands purpose, jurisdiction, confidentiality, or boundaries.
135. Emergency intersection uses union or otherwise amplifies privilege.
136. Revocation/freshness changes between reconciliation and use.
137. Client, legal entity, or engagement is substituted.
138. Break-glass or emergency approver proxies Human Principal.
139. Audit lineage truncates or reorders execution, consumption, executor, or outcome.
140. Hashes agree but authorization-chain or execution-instance identity differs.

All twenty-four attacks must fail through exact Authorization and Access bindings, typed contemporaneous reconciliation, canonical-source typing, zero emergency-delegation depth, single-use consumption state, execution-instance equality, least-privilege intersection, or audit completeness. Expiry alone, generic authority, matching hashes, and post-hoc record assembly never validate emergency use.

## Residual limitations

Markdown cannot authenticate, enforce access, inspect credentials, resolve real identities, evaluate policies, isolate tenants, provide clocks, revoke runtime sessions, or make audit storage immutable. Those require separately certified implementations, authoritative directories, qualified reviewers, and security testing. Schema validity is not operational authorization.
