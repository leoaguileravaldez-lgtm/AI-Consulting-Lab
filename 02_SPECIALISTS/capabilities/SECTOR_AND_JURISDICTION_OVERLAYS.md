# Sector and Jurisdiction Overlays

## Purpose

Overlays constrain and qualify specialist work for a sector, jurisdiction, regulated profession, population, or institutional context. They are not standalone practices, workflow roles, actors, validators, or authorities.

## Canonical Form

An overlay record identifies:

```text
overlay_id
overlay_version
sector_or_jurisdiction_scope
applicable_practice_ids
capability_ids
qualification_record_refs
actor_eligibility_constraints
authoritative_sources_and_freshness
mandatory_methods_or_standards
prohibited_uses
required_human_professional_review
risk_domains
data_and_residency_constraints
conflicts_and_restrictions
effective_from
expires_at
owner
approval_reference
audit_linkage
```

## Rules

- An overlay narrows eligibility, evidence, methods, or review; it cannot broaden scope, tier, data access, or authority.
- Applicability is determined from engagement facts, not a specialist preference.
- Multiple overlays combine using the more restrictive applicable requirement.
- Unknown jurisdiction or sector classification uses the higher plausible risk and escalates.
- Overlay qualification must be current and independently verifiable.
- A generic practice qualification cannot substitute for licensed or jurisdiction-specific human review.
- Overlay reuse across engagements contains no client content and does not bypass engagement-specific authorization.
- Changes are versioned; affected active work is reassessed.

## Common Overlay Domains

Potential overlays include financial services, healthcare, energy, infrastructure, defense, education, government, nonprofit/development, consumer, industrial, technology, real estate, climate/environment, procurement, tax/accounting, cybersecurity/privacy, employment, safety/engineering, and country/subnational jurisdiction.

This list does not activate capabilities or assert qualifications.

## Stop Conditions

Stop when jurisdiction, sector, applicable authority, qualification, licensing need, residency/data requirement, or conflict cannot be established. The Human Principal or applicable qualified human resolves authority and professional-review requirements; specialists cannot self-approve an overlay.
