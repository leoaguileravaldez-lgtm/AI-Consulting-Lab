# Sensitivity and Decision Reversal

## Distinctions

- `MODEL_SENSITIVITY`: numerical output movement from input change.
- `DECISION_SENSITIVITY`: whether credible input change alters the preferred decision.
- `CLASSIFICATION_SENSITIVITY`: whether change alters materiality or required control.
- `DECISION_SWITCH_THRESHOLD`: value at which option ranking changes.
- `ROBUSTNESS_RANGE`: credible range over which the recommendation remains preferred.

Low model sensitivity does not prove decision robustness; a small movement near a decision boundary may reverse the choice. High model sensitivity may still leave option ranking unchanged.

## Procedure

Identify break-even values, high-elasticity variables, nonlinearities, interactions, correlated inputs, downside ranges and reversal conditions. Test ranges supported by `04`-validated evidence or label them as scenarios.

For each Material variable record base value/version, plausible range, provenance, method owner, output response, decision effect, confidence, limitation and audit link.

`05` owns adversarial framing and interpretation. Governed calculations belong to `02` where invoked; formal validation remains canonical. A sensitivity finding is not risk acceptance, confidence authority or approval.

If small plausible changes reverse the recommendation, publish `RECOMMENDATION_FRAGILE` or another applicable analytical outcome and preserve the exact threshold. `01` decides workflow effect.
