# TITUS LAB Stage II Mandate Contract

This bounded local slice implements the Layer 15 `MANDATE_CONTRACT` node between the frozen Engagement Domain and the future `DECISION_PROBLEM` node. It defines an authorized problem space; it performs no analysis and creates no answer, recommendation, delivery, external action, or institutional authority.

Each mandate is deterministically identified and immutably bound to one certified engagement. Exactly one current version is retained, prior versions remain in append-only history, and every content or language-reference change creates a successor version. Creation, amendment, and closure require exact current engagement-bound authority and the Human Principal authorizer. Scope is explicitly separated into in-scope, out-of-scope, and unresolved items; no out-of-scope or unresolved request becomes authorized work.

Original client mandate inputs retain exact text, source identity, source language, and provenance. TITUS-normalized `what` and `why` fields are derived mandate content, not replacements for source material. The mandate working-language reference follows the engagement setting while source language and original text remain unchanged.

The only downstream artifact is a read-only, exact-mandate/version `DecisionProblemHandoff`. It contains no Decision Problem and grants no analysis, execution, certification, or decision authority. Stage I and the frozen Engagement Domain remain unchanged and are consumed through their existing boundaries.

Workbench, client access, production persistence, Stage II certification, deployment, and Layer 20 remain absent.

