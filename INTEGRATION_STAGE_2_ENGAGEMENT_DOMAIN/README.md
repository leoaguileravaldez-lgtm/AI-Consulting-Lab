# TITUS LAB Stage II Engagement Domain

This bounded, local, synthetic Stage II slice represents one engagement domain on the frozen Stage I Integration Kernel. It adds no institutional authority, frontend, client access, production store, deployment capability, or Layer 20.

The domain preserves deterministic client and engagement identities, explicit versions and predecessor history, bounded lifecycle transitions, engagement-bound authority, stale-write and cross-engagement rejection, and distinct source, evidence, deliverable, build, audit, and validation references. Client statements remain statements rather than verified facts; recommendations remain distinct from Human Principal decisions.

Every material child record carries a private immutable owning `engagement_id` exposed only through the read-only `EngagementOwned` interface. Requirement, evidence, and deliverable admission validates that each referenced source, evidence item, or build belongs to the exact target engagement; a valid identifier from another engagement—including another engagement for the same client—is rejected.

The engagement aggregate and its material collections are private. External callers observe them only through immutable accessors; lifecycle, version, predecessor, ownership, provenance, and collection mutation remains confined to the existing checked aggregate methods. No mutable accessor or parallel unchecked insertion path is exposed.

`WORKING_LANGUAGE` is engagement-scoped and limited initially to `ES` or `EN`. It never replaces `SOURCE_LANGUAGE`. Original source references, fingerprints, source language, and provenance remain immutable when working language changes. Localized output is a derived artifact linked to its original source.

Stage II consumes a completed, candidate-bound Stage I `CERTIFIED_PASS` result before entering `AWAITING_HUMAN_DECISION`. It does not reproduce Stage I audit or validation logic. Stage II candidate-specific independent certification is deferred to its separately authorized closure boundary; the executable handoff and Human Principal ordering are preserved now.

The future Workbench remains a minimalist, clean operator control plane. Client access remains distinct from source access, and the proprietary TITUS LAB core remains server-side by default.
