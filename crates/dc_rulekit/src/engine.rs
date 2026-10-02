use crate::audit::AuditHook;
use crate::context::EvalContext;
use crate::error::{Result, RulekitError};
use crate::model::{
    ActionOutcome, AuditReceipt, ConditionOutcome, Rule,
};
use crate::plugin::PluginRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluateOptions {
    pub dry_run: bool,
}

impl Default for EvaluateOptions {
    fn default() -> Self {
        Self { dry_run: false }
    }
}

pub struct Engine<'a> {
    registry: &'a PluginRegistry,
}

impl<'a> Engine<'a> {
    pub fn new(registry: &'a PluginRegistry) -> Self {
        Self { registry }
    }

    /// Validate plugins and evaluate `when` conditions (all must pass), then run `then` actions.
    pub fn evaluate(
        &self,
        rule: &Rule,
        ctx: &EvalContext,
        options: EvaluateOptions,
    ) -> Result<AuditReceipt> {
        rule.validate_schema()?;
        if !rule.enabled {
            return Err(RulekitError::DisabledRule {
                rule_id: rule.id.clone(),
            });
        }
        self.registry.validate_rule_plugins(rule)?;

        let mut condition_results = Vec::with_capacity(rule.when.len());
        let mut all_passed = true;

        for condition in &rule.when {
            let evaluator = self.registry.get_condition(&condition.plugin)?;
            let passed = evaluator.evaluate(&condition.params, ctx)?;
            if !passed {
                all_passed = false;
                condition_results.push(ConditionOutcome {
                    condition_id: condition.id.clone(),
                    plugin: condition.plugin.clone(),
                    passed: false,
                    detail: None,
                });
                // Fail-fast on first failing condition (ordered evaluation).
                break;
            }
            condition_results.push(ConditionOutcome {
                condition_id: condition.id.clone(),
                plugin: condition.plugin.clone(),
                passed: true,
                detail: None,
            });
        }

        let mut action_outcomes = Vec::new();
        if all_passed {
            for action in &rule.then {
                let handler = self.registry.get_action(&action.plugin)?;
                let (executed, skipped_dry_run, result) = if options.dry_run && !handler.is_pure() {
                    (false, true, None)
                } else {
                    match handler.execute(&action.params, ctx) {
                        Ok(value) => (true, false, Some(value)),
                        Err(RulekitError::ActionDenied {
                            reason, ..
                        }) => {
                            return Err(RulekitError::ActionDenied {
                                rule_id: rule.id.clone(),
                                action_id: action.id.clone(),
                                plugin_id: action.plugin.clone(),
                                reason,
                            });
                        }
                        Err(e) => return Err(e),
                    }
                };
                action_outcomes.push(ActionOutcome {
                    action_id: action.id.clone(),
                    plugin: action.plugin.clone(),
                    executed,
                    skipped_dry_run,
                    result,
                });
            }
        }

        Ok(AuditReceipt {
            rule_id: rule.id.clone(),
            matched: all_passed,
            dry_run: options.dry_run,
            condition_results,
            action_outcomes,
            opaque: serde_json::json!({}),
        })
    }

    pub fn evaluate_with_audit<H: AuditHook>(
        &self,
        rule: &Rule,
        ctx: &EvalContext,
        options: EvaluateOptions,
        hook: &H,
        opaque: serde_json::Value,
    ) -> Result<AuditReceipt> {
        let mut receipt = self.evaluate(rule, ctx, options)?;
        receipt.opaque = opaque;
        hook.on_receipt(&receipt);
        Ok(receipt)
    }
}
