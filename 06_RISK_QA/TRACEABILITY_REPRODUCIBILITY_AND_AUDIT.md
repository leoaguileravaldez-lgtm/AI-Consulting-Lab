# Traceability, Reproducibility, and Audit

## Exact assessment binding

Every QA assessment binds immutable references to:

- engagement and task;
- authorized scope;
- assessed object, version, and hash where available;
- evidence state/version from `04`;
- challenge and dissent state/version from `05`;
- control manifest and operating-evidence versions;
- canonical blocker, exception, approval, and workflow state/version;
- professional-review state/version where applicable;
- assessor identity, role assignment, session/model, independence record;
- methodology and schema version;
- assessment and as-of timestamps;
- dependency manifest and audit references.

Missing, stale, foreign-engagement, internally inconsistent, or unverifiable bindings prevent clean reliance.

## Reperformance readiness

The record preserves the test question, criteria, inputs, exclusions, sampling, transformations, procedures, calculations, results, limitations, and evidence references sufficient for an eligible reviewer to reperform the test. Reperformance does not authorize a state change or validate source evidence outside `04`.

## Immutable history

Corrections and reassessments append versions; they never overwrite findings, adverse outcomes, dissent, failed controls, exceptions, or prior QA results. Audit append, sequence, integrity, correction, and retention remain canonical to `01`. `06` supplies attributable records and audit references but cannot create a parallel audit authority.

