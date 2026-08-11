# Freshness, Versioning, and Supersession

Knowledge freshness metadata includes `CURRENT`, `REVIEW_REQUIRED`, `STALE`, `SUPERSEDED`, `DEPRECATED` and `UNKNOWN`. These are knowledge-record semantics only. Missing freshness, review date, as-of date or applicable decay rule cannot yield `CURRENT` for a Material factual object.

Freshness rules are type-specific. Law, regulation, tax, technology, software, markets, pricing, standards, competition, institutional roles and vendor capability require review frequencies suited to their rate of change. Historical validity does not imply current applicability.

Every methodology identifies methodology ID/version, valid-from, superseded-by, change classification, compatibility status, re-performance requirement and affected result classes. Compatibility results are `BACKWARD_COMPATIBLE`, `REVIEW_REQUIRED`, `REPERFORMANCE_REQUIRED`, `INCOMPATIBLE` or `UNKNOWN`. Unknown fails closed.

A newer method never silently reinterprets an older result. Historical work remains bound to the method actually used. Re-performance creates new lineage and preserves the original.

