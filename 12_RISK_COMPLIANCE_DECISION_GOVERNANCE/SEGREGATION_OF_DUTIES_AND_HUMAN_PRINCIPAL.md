# Segregation of Duties and Human Principal Authority

## Segregation of duties

SOD constraints specify incompatible roles/actions, materiality, scope, minimum independent reviewers, permitted human delegation, recusal, substitute authority, and validation.

The mandatory Material baseline, versioned as `L12-MATERIAL-SOD-BASELINE-v1`, always contains all eight rule IDs and prohibits the same effective actor from serving as:

1. `MSOD-01`: risk assessor and risk-acceptance authority;
2. `MSOD-02`: waiver requester and waiver-approval authority;
3. `MSOD-03`: exception requester and exception-decision authority;
4. `MSOD-04`: decision proposer and sole decision approver;
5. `MSOD-05`: compliance assessor and sole compliance-override authority;
6. `MSOD-06`: conflict subject and sole conflict-resolution authority;
7. `MSOD-07`: evidence producer and sole decision authority when the decision materially depends on that evidence;
8. `MSOD-08`: both sides of any incompatibility through human/account aliases, model or agent identities, delegated roles, or authority chains.

These rules are non-optional and apply before configurable extensions. Validation requires exact set equality with all eight rule IDs and records an applicability/result for each; `NOT_APPLICABLE` requires exact reason and authority. An empty, missing, disabled, incomplete, or less restrictive configurable SOD set cannot remove them. Unknown applicability or unknown effective identity on Material work is `SOD_BLOCKED`.

Self-approval, reciprocal approval, circular approval chains, concealed common control, role aliasing, shared identity, unverified delegation, and approval splitting fail closed. Technical capability, seniority, client status, urgency, or staffing shortage cannot cure a mandatory SOD violation.

## Effective actor

`effective_actor_id` identifies the ultimate human or canonical authority controlling an act after resolving account aliases, role aliases, agent/model delegation, service identity, acting-on-behalf-of relationships, and delegation chains. Resolution records every presented identity, canonical identity, controlling authority, delegation edge, validity window, conflicts, and verification source. Delegation loops, unresolved common control, multiple identities converging on one controller, or unknown effective identity fail closed. Models and agents cannot become independent human approvers merely through distinct names or roles.

## Human Principal

The Human Principal retains all authority reserved by Layers 00–11, including non-delegable approvals, permitted exceptions, material residual-risk acceptance, and strategic/policy decisions. Layer 12 requires exact contemporaneous decision references with identity, category, scope, conditions, version, time, and validity.

No model, agent, workflow, CRM state, scheduler, retry, timeout, silence, consensus, vote, confidence score, commercial priority, client instruction, role label, or technical access may appoint, authenticate, impersonate, infer, fabricate, replay, batch, delegate, or proxy Human Principal authorization.

Human Principal approval cannot make unsupported evidence true, make analysis professionally valid, pass QA, erase dissent, cure unlawful conduct, authorize a non-waivable exception, downgrade confidentiality implicitly, permit unauthorized cross-client movement, rewrite audit history, or itself execute an action.
