# Phase 01 Orchestrator Design Package

## Status

Design only. Subordinate to `00_CORE` version 0.2.0-draft. No implementation or operational authority.

## Objective

Design an Orchestrator that optimizes for defensible decision quality through evidence, independent challenge, validation, risk review, controlled synthesis, and Human Principal authority.

## Reading Order

1. `GOVERNANCE_CONTROL_MAP.md`
2. `ORCHESTRATOR_ARCHITECTURE.md`
3. `AUTHORITY_PERMISSIONS_AND_DUTIES.md`
4. `TASK_INTAKE_CLASSIFICATION_AND_MATERIALITY.md`
5. `SPECIALIST_REGISTRY.md`
6. `STATE_MACHINE.md`
7. `DELEGATION_HANDOFF_AND_ASSURANCE.md`
8. `EVIDENCE_CONFIDENCE_AND_RISK.md`
9. `APPROVAL_STOP_AND_ESCALATION.md`
10. `RECORD_SCHEMAS_AND_AUDIT.md`
11. `DELIVERABLE_LIFECYCLE_AND_RECOVERY.md`
12. `SECURITY_BOUNDARIES_AND_THREAT_MODEL.md`
13. `FUTURE_EXTERNAL_TOOL_BOUNDARY.md`

## Architectural Decision

Material recommendations follow:

```text
PRIMARY ANALYSIS
→ INDEPENDENT CHALLENGE
→ EVIDENCE VALIDATION
→ RISK REVIEW
→ ORCHESTRATOR SYNTHESIS
→ HUMAN PRINCIPAL DECISION
```

The primary specialist cannot independently validate its own Material conclusion. Failed or unknown controls fail closed.

## Proposed Operating Parameters Awaiting Approval

The package labels these proposals wherever used:

- maximum automatic analytical retries: 2;
- maximum recursive delegation depth: 2;
- audit storage normalized to UTC with original timezone retained;
- escalation response-time targets;
- implementation recovery time and recovery point objectives;
- detailed risk appetite below `00_CORE` Material/Critical triggers.

Engagement-wide anti-splitting aggregation, reassessment on Material change, record versioning, content binding where required, stale-write rejection, and explicit reconciliation are mandatory design controls. None of the listed numeric/service parameters is operationally approved by inclusion in this package.
