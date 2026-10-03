use crate::audit::AuditHook;
use crate::conditions::ConditionNode;
use crate::context::EvalContext;
use crate::error::{Result, RulekitError};
use crate::model::{ActionOutcome, AuditReceipt, ConditionOutcome, Rule};
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

    /// Validate plugins and evaluate `conditions` (json-rules-engine tree), then run `events`.
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

        let mut condition_results = Vec::new();
        let all_passed = self.evaluate_conditions(
            &rule.conditions,
            ctx,
            &mut condition_results,
            true,
        )?;

        let mut action_outcomes = Vec::new();
        if all_passed {
            for action in &rule.events {
                let handler = self.registry.get_action(action.plugin_id())?;
                let (executed, skipped_dry_run, result) = if options.dry_run && !handler.is_pure() {
                    (false, true, None)
                } else {
                    match handler.execute(&action.params, ctx) {
                        Ok(value) => (true, false, Some(value)),
                        Err(RulekitError::ActionDenied { reason, .. }) => {
                            return Err(RulekitError::ActionDenied {
                                rule_id: rule.id.clone(),
                                action_id: action.id.clone(),
                                plugin_id: action.plugin_id().to_string(),
                                reason,
                            });
                        }
                        Err(e) => return Err(e),
                    }
                };
                action_outcomes.push(ActionOutcome {
                    action_id: action.id.clone(),
                    event_type: action.event_type.clone(),
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

    fn evaluate_conditions(
        &self,
        node: &ConditionNode,
        ctx: &EvalContext,
        outcomes: &mut Vec<ConditionOutcome>,
        fail_fast: bool,
    ) -> Result<bool> {
        match node {
            ConditionNode::All { all } => {
                if all.is_empty() {
                    return Ok(true);
                }
                for child in all {
                    let passed = self.evaluate_conditions(child, ctx, outcomes, fail_fast)?;
                    if !passed {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ConditionNode::Any { any } => {
                if any.is_empty() {
                    return Ok(false);
                }
                for child in any {
                    let mut branch_outcomes = Vec::new();
                    let passed =
                        self.evaluate_conditions(child, ctx, &mut branch_outcomes, false)?;
                    if passed {
                        outcomes.extend(branch_outcomes);
                        return Ok(true);
                    }
                    outcomes.extend(branch_outcomes);
                }
                Ok(false)
            }
            ConditionNode::Not { not } => {
                let inner = self.evaluate_conditions(not, ctx, outcomes, false)?;
                Ok(!inner)
            }
            ConditionNode::Leaf(condition) => {
                let evaluator = self.registry.get_condition(&condition.plugin)?;
                let passed = evaluator.evaluate(&condition.params, ctx)?;
                outcomes.push(ConditionOutcome {
                    condition_id: condition.id.clone(),
                    plugin: condition.plugin.clone(),
                    passed,
                    detail: None,
                });
                Ok(passed)
            }
        }
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
