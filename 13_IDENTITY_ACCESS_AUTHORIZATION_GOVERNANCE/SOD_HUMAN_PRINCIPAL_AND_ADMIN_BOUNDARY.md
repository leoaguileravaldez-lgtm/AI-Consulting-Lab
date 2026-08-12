# SOD, Human Principal, and Administrative Boundary

Layer 13 incorporates Layer 12 `L12-MATERIAL-SOD-BASELINE-v1`, including `MSOD-01` through `MSOD-08`, by exact version and cannot weaken, omit, configure empty, or waive any rule.

Additional mandatory Material access rules, versioned as `L13-MATERIAL-ACCESS-SOD-BASELINE-v1`, are:

1. `ASOD-01`: access requester cannot be sole access approver;
2. `ASOD-02`: grantor cannot be the beneficiary of their grant;
3. `ASOD-03`: privilege-elevation requester cannot approve elevation;
4. `ASOD-04`: delegated actor cannot be sole delegation approver;
5. `ASOD-05`: audit subject cannot be sole audit authority/verifier;
6. `ASOD-06`: administrator cannot be sole business-decision approver;
7. `ASOD-07`: machine identity cannot authorize itself;
8. `ASOD-08`: aliases, shared accounts, agents/models, delegation, or common control cannot create nominal separation.

Validation requires exact set equality with all eight Layer 12 and all eight Layer 13 mandatory rule IDs, exact baseline versions, counts of eight each, and an applicability/result for each. Configurable extensions only add restrictions. Unknown effective actor or common control is `SOD_BLOCKED`.

Human Principal authority requires an exact, current, attributable, category/scope/version/time-bound canonical reference. Credentials, authentication, role labels, ownership, administration, system control, timeout, silence, workflow state, client instruction, urgency, or machine identity cannot infer or proxy it. Delegation is permitted only where certified policy explicitly allows the exact bounded action; non-delegable authority remains non-delegable.
