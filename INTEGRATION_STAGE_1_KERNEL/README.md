# TITUS LAB Stage I Integration Kernel

This bounded, local, non-production workspace composes certified TITUS LAB primitives into the first executable integration path. It creates no institutional authority and does not modify or replace any certified contract.

The kernel accepts an exact runtime identity and authority envelope, validates context and current authority, applies a fixed operation-scope gate, dispatches deterministic synthetic work, commits an optimistic-concurrency state transition through the certified SQLite concurrency component, propagates provenance, appends and independently verifies certified audit evidence, constructs a truthful candidate-specific Stage I validation package, invokes the separate `stage1_independent_validator` process, and stops at `AWAITING_HUMAN_PRINCIPAL_DECISION` only after an independent fail-closed verdict.

The validation handoff is not certification. Audit evidence is not execution authority. A Human decision must bind the exact result and the fixed synthetic Human Principal identity; it changes only the local gate representation and triggers no external action. Post-transition audit or validation uncertainty produces an explicit failure, a non-authorizing recovery-required handoff and no Human gate; it does not roll back or conceal the already observed authoritative state transition.

Execution completion first yields `VALIDATION_REQUIRED` and exposes no Human Principal gate. Only a candidate/version/package-bound `CERTIFIED_PASS` returned by the separate validator transitions the result to `AWAITING_HUMAN_PRINCIPAL_DECISION`. Missing, failing, invalid, stale, infrastructure, or validator-failure outcomes remain ineligible for Human approval.

The frozen Phase 1 validator remains candidate-specific and unchanged. It correctly rejects the Stage I identity with `CANDIDATE_PROVENANCE_OR_VERSION_MISMATCH`. Stage I never substitutes the older Phase 1 candidate identity. Its candidate package uses deterministic JSON and a SHA-256 fingerprint over the package with `package_fingerprint` omitted; the separate validator independently binds the current source manifest, certified predecessor manifest, fixed Stage I expectations, authority observations, provenance, and Human Principal gate.

This slice does not implement an engagement domain, Control API, general orchestration runtime, external model/tool adapter, Workbench, software-delivery runtime, production tenancy, deployment, external action, or Layer 20. All data and execution are deterministic and synthetic.

## Deferred client access and IP boundary

Stage I implements no client access. A future pilot or client interface must preserve `CLIENT ACCESS != SOURCE ACCESS`: no client repository access or backend source distribution by default; proprietary institutional logic, secrets and credentials remain server-side; access is tenant-isolated, least-privilege, API-scoped and audited; production debug endpoints are disabled; production source maps are disabled unless explicitly required and controlled; browser code contains no institutional authority or critical logic that depends on secrecy; and minification or obfuscation is defense-in-depth only. Any future client-controlled or on-premise delivery requires separate evaluation of signed compiled or packaged artifacts, private registries, integrity verification, licensing and encrypted secret handling. No claim of mathematical impossibility of reverse engineering is permitted.

## Deferred Workbench language boundary

The future TITUS LAB Workbench remains unimplemented. It is a minimalist, clean operator control plane with initial working-language selections `ES` and `EN`. `WORKING_LANGUAGE` is a per-engagement or per-workspace presentation and operator-communication preference that may govern future analysis, normal system communication, reports, deliverables when requested, and applicable Workbench labels or content.

`WORKING_LANGUAGE` is not `SOURCE_LANGUAGE`. The original source, its source language, evidence identity and historical provenance remain preserved in original form. Translation or localized rendering is a derived artifact and cannot replace or mutate original evidence. Language selection creates no truth, certification, policy, evidence, provenance or institutional authority; `WORKING_LANGUAGE_TO_EVIDENCE_MUTATION_AUTHORITY = 0` and `LANGUAGE_SETTING_AUTHORITY_AMPLIFICATION = 0`.
