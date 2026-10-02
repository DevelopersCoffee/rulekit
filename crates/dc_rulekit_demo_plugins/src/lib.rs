//! Toy plugins for dc_rulekit quickstarts (not for production domain logic).

use dc_rulekit::{ActionHandler, ConditionEvaluator, EvalContext, Result};
use serde_json::{json, Value};

pub const PLUGIN_ALWAYS: &str = "demo.when.always";
pub const PLUGIN_THRESHOLD: &str = "demo.when.threshold";
pub const PLUGIN_COUNTER_GTE: &str = "demo.when.counter_gte";
pub const PLUGIN_LOG: &str = "demo.then.log";
pub const PLUGIN_INCREMENT: &str = "demo.then.increment_counter";

/// Always passes.
pub struct WhenAlways;

impl ConditionEvaluator for WhenAlways {
    fn plugin_id(&self) -> &str {
        PLUGIN_ALWAYS
    }
    fn evaluate(&self, _params: &Value, _ctx: &EvalContext) -> Result<bool> {
        Ok(true)
    }
}

/// Passes when `ctx.facts[key] >= params.min` (numeric).
pub struct WhenThreshold;

impl ConditionEvaluator for WhenThreshold {
    fn plugin_id(&self) -> &str {
        PLUGIN_THRESHOLD
    }
    fn evaluate(&self, params: &Value, ctx: &EvalContext) -> Result<bool> {
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .unwrap_or("value");
        let min = params.get("min").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let actual = ctx
            .facts
            .get(key)
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        Ok(actual >= min)
    }
}

/// Passes when counter fact >= params.threshold (alias for threshold with `counter` key default).
pub struct WhenCounterGte;

impl ConditionEvaluator for WhenCounterGte {
    fn plugin_id(&self) -> &str {
        PLUGIN_COUNTER_GTE
    }
    fn evaluate(&self, params: &Value, ctx: &EvalContext) -> Result<bool> {
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .unwrap_or("counter");
        let threshold = params
            .get("threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);
        let actual = ctx
            .facts
            .get(key)
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        Ok(actual >= threshold)
    }
}

/// Logs `message` to stderr (side effect); returns `{ "logged": message }`.
pub struct ThenLog;

impl ActionHandler for ThenLog {
    fn plugin_id(&self) -> &str {
        PLUGIN_LOG
    }
    fn is_pure(&self) -> bool {
        false
    }
    fn execute(&self, params: &Value, _ctx: &EvalContext) -> Result<Value> {
        let message = params
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("dc_rulekit demo log");
        eprintln!("[dc_rulekit-demo] {}", message);
        Ok(json!({ "logged": message }))
    }
}

/// Pure planner: returns increment instruction (host may apply); does not mutate ctx.
pub struct ThenIncrementCounter;

impl ActionHandler for ThenIncrementCounter {
    fn plugin_id(&self) -> &str {
        PLUGIN_INCREMENT
    }
    fn is_pure(&self) -> bool {
        true
    }
    fn execute(&self, params: &Value, _ctx: &EvalContext) -> Result<Value> {
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .unwrap_or("counter");
        let by = params.get("by").and_then(|v| v.as_f64()).unwrap_or(1.0);
        Ok(json!({ "increment": { "key": key, "by": by } }))
    }
}

/// Register all demo plugins on a registry.
pub fn register_all(registry: &mut dc_rulekit::PluginRegistry) {
    registry.register_condition(Box::new(WhenAlways));
    registry.register_condition(Box::new(WhenThreshold));
    registry.register_condition(Box::new(WhenCounterGte));
    registry.register_action(Box::new(ThenLog));
    registry.register_action(Box::new(ThenIncrementCounter));
}

/// End-to-end quickstart: propose static rule → approve → evaluate → audit.
pub fn run_quickstart_example() -> Result<Value> {
    use dc_rulekit::{
        Action, AuditHook, Condition, Engine, EvaluateOptions, ProposalStore, Rule, RuleSource,
        RuleStore,
    };
    use std::sync::{Arc, Mutex};

    struct CaptureHook(Arc<Mutex<Vec<dc_rulekit::AuditReceipt>>>);
    impl AuditHook for CaptureHook {
        fn on_receipt(&self, receipt: &dc_rulekit::AuditReceipt) {
            self.0.lock().unwrap().push(receipt.clone());
        }
    }

    let mut registry = dc_rulekit::PluginRegistry::new();
    register_all(&mut registry);
    let engine = Engine::new(&registry);

    let mut rule = Rule::new("demo.app/quickstart", "Quickstart", RuleSource::Static);
    rule.when.push(Condition {
        id: "always".into(),
        plugin: PLUGIN_ALWAYS.into(),
        params: json!({}),
    });
    rule.then.push(Action {
        id: "log".into(),
        plugin: PLUGIN_LOG.into(),
        params: json!({ "message": "approved rule fired" }),
    });

    let mut proposals = ProposalStore::in_memory();
    let mut active = RuleStore::in_memory();
    let proposal = proposals.propose(rule)?;
    let active_rule = proposals.approve(&proposal.proposal_id, &mut active)?;

    let hook_storage = Arc::new(Mutex::new(Vec::new()));
    let hook = CaptureHook(Arc::clone(&hook_storage));
    let opaque = json!({ "example": "quickstart", "version": 1 });

    let receipt = engine.evaluate_with_audit(
        &active_rule,
        &EvalContext::new("demo.app"),
        EvaluateOptions::default(),
        &hook,
        opaque,
    )?;

    let hooks = hook_storage.lock().unwrap();
    Ok(json!({
        "proposal_id": proposal.proposal_id,
        "rule_id": active_rule.id,
        "matched": receipt.matched,
        "audit_count": hooks.len(),
        "opaque": hooks.first().map(|r| r.opaque.clone()),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quickstart_runs() {
        let out = run_quickstart_example().unwrap();
        assert_eq!(out.get("matched").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(out.get("audit_count").and_then(|v| v.as_u64()), Some(1));
    }

    #[test]
    fn threshold_condition() {
        let eval = WhenThreshold;
        let ctx = EvalContext::new("demo.app").with_fact("value", json!(5));
        assert!(eval.evaluate(&json!({ "key": "value", "min": 3 }), &ctx).unwrap());
        assert!(!eval.evaluate(&json!({ "key": "value", "min": 10 }), &ctx).unwrap());
    }
}
