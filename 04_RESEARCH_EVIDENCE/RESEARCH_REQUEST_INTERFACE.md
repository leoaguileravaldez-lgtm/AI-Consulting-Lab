# Research Request Interface

## Request Contract

`02_SPECIALISTS` and `03_ENGAGEMENTS` express evidence needs through a bounded request submitted to `01_ORCHESTRATOR`. Required fields are:

- engagement, canonical task and workstream IDs;
- requester actor/role and authorization reference;
- question and exact claim to test;
- decision context without a prescribed conclusion;
- materiality and decision-criticality;
- jurisdiction, population, definitions, units and time scope;
- minimum evidence threshold and freshness deadline;
- authorized source/tool/data classifications and access limits;
- known sources, assumptions, conflicts and exclusions;
- requested output, deadline, priority and audit correlation ID.

`04` may clarify an evidentiary ambiguity, propose a narrower testable claim, or report that the request is not researchable. It may not redefine the business decision, widen scope, lower materiality, change jurisdiction, activate a specialist, choose a strategy or promise a favorable answer.

## Routing and Acceptance

Only `01` assigns researchers/validators, creates canonical tasks/handoffs, grants access and determines state. The recipient explicitly accepts engagement, scope, role, qualifications, conflicts, data, tools and stop conditions. Missing/mismatched values fail closed through applicable canonical controls.

A request containing answer-leading instructions, selective-source restrictions, client pressure, unsupported confidence targets or a demand to ignore contrary evidence is flagged as a bias/conflict condition. Research continues only within an authorized corrected scope.

Deadlines do not lower evidence, freshness, independence or corroboration requirements. If the threshold cannot be met, return `INSUFFICIENT_EVIDENCE` or `NOT_VERIFIABLE` and notify `01`; do not fabricate completeness.

## Future Tooling Boundary

The request may name a future web, financial-data, government, academic, client-document, email, cloud-storage, news, market-data or engineering-data interface. The field is descriptive only. It cannot activate a connector, expose credentials, browse, purchase, subscribe, message, email, upload or execute. Every future retrieval needs separate exact authorization and governed connector architecture.
