use crate::context::EvalContext;
use crate::error::{Result, RulekitError};
use serde_json::Value;

/// Evaluates a single condition plugin invocation.
pub trait ConditionEvaluator: Send + Sync {
    fn plugin_id(&self) -> &str;
    fn evaluate(&self, params: &Value, ctx: &EvalContext) -> Result<bool>;
}

/// Executes a single action plugin invocation.
pub trait ActionHandler: Send + Sync {
    fn plugin_id(&self) -> &str;
    /// When `true`, dry-run may invoke the handler (no side effects). When `false`, dry-run skips execution and records the planned action.
    fn is_pure(&self) -> bool;
    fn execute(&self, params: &Value, ctx: &EvalContext) -> Result<Value>;
}

/// Registry of host-supplied condition and action plugins (fail-closed on unknown ids).
#[derive(Default)]
pub struct PluginRegistry {
    conditions: std::collections::HashMap<String, Box<dyn ConditionEvaluator>>,
    actions: std::collections::HashMap<String, Box<dyn ActionHandler>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_condition(&mut self, evaluator: Box<dyn ConditionEvaluator>) {
        let id = evaluator.plugin_id().to_string();
        self.conditions.insert(id, evaluator);
    }

    pub fn register_action(&mut self, handler: Box<dyn ActionHandler>) {
        let id = handler.plugin_id().to_string();
        self.actions.insert(id, handler);
    }

    pub fn get_condition(&self, plugin_id: &str) -> Result<&dyn ConditionEvaluator> {
        self.conditions
            .get(plugin_id)
            .map(|b| b.as_ref())
            .ok_or_else(|| RulekitError::UnknownConditionPlugin {
                plugin_id: plugin_id.to_string(),
            })
    }

    pub fn get_action(&self, plugin_id: &str) -> Result<&dyn ActionHandler> {
        self.actions
            .get(plugin_id)
            .map(|b| b.as_ref())
            .ok_or_else(|| RulekitError::UnknownActionPlugin {
                plugin_id: plugin_id.to_string(),
            })
    }

    pub fn validate_rule_plugins(&self, rule: &crate::model::Rule) -> Result<()> {
        for c in &rule.when {
            self.get_condition(&c.plugin)?;
        }
        for a in &rule.then {
            self.get_action(&a.plugin)?;
        }
        Ok(())
    }
}
