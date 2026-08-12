# Client Isolation and Handoffs

Every client-bound work item, dependency, queue entry, attempt, lease, transition, exception, handoff, and audit record carries Layer 10 `client_instance_id`, Layer 09 `client_id`, legal entity where applicable, Layer 03 engagement ID, client security domain, engagement subdomain, purpose, and classification.

Cross-client execution reuse, cross-engagement substitution, shared mutable state, shared completion flags, shared retry/recurrence counters, and boundary-less queues are prohibited. A missing, mixed, copied, foreign, stale, or ambiguous identifier causes denial, quarantine, containment, audit, and canonical incident escalation.

Handoffs record sender/recipient roles, exact work item/input/output versions, boundary, purpose, classification, state, dependencies, blockers, limitations, acceptance criteria, time, authority, and audit. Receipt does not imply acceptance, readiness, correctness, or approval. Recipient validates the handoff independently.

Parallelism and handoffs cannot bypass Layer 10 isolation. Client material, prompts, context, embeddings, caches, summaries, or outputs never cross boundaries through workflow metadata.

