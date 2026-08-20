# Phase 1 Recovery/Reconciliation

This workspace implements only `BC_RECOVERY_RECONCILIATION`. Dedicated SQLite child Process A is interrupted at a deterministic uncommitted-transaction barrier; fresh recovery Process B reconstructs only durable internal evidence and preserves UNKNOWN as reconciliation-required without creating retry authority.

Evidence is local implementation evidence only. It does not determine external effects, authorize retry, absorb Layer 19 authority, or claim power-loss, production, distributed, audit, deployment, or independent-certification guarantees.

