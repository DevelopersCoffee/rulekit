use dc_rulekit::{
    Action, ActionHandler, AuditHook, Condition, ConditionEvaluator, Engine, EvaluateOptions,
    EvalContext, PluginRegistry, ProposalStore, Rule, RuleSource, RuleStore, RulekitError,
};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

struct AlwaysCondition;
impl ConditionEvaluator for AlwaysCondition {
    fn plugin_id(&self) -> &str {
        "demo.when.always"
    }
    fn evaluate(&self, _params: &Value, _ctx: &EvalContext) -> dc_rulekit::Result<bool> {
        Ok(true)
    }
}

struct NeverCondition;
impl ConditionEvaluator for NeverCondition {
    fn plugin_id(&self) -> &str {
        "demo.when.never"
    }
    fn evaluate(&self, _params: &Value, _ctx: &EvalContext) -> dc_rulekit::Result<bool> {
        Ok(false)
    }
}

struct LogAction {
    log: Arc<Mutex<Vec<String>>>,
}
impl ActionHandler for LogAction {
    fn plugin_id(&self) -> &str {
        "demo.then.log"
    }
    fn is_pure(&self) -> bool {
        false
    }
    fn execute(&self, params: &Value, _ctx: &EvalContext) -> dc_rulekit::Result<Value> {
        let msg = params
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("log");
        self.log.lock().unwrap().push(msg.to_string());
        Ok(json!({ "logged": msg }))
    }
}

struct PureEchoAction;
impl ActionHandler for PureEchoAction {
    fn plugin_id(&self) -> &str {
        "demo.then.echo"
    }
    fn is_pure(&self) -> bool {
        true
    }
    fn execute(&self, params: &Value, _ctx: &EvalContext) -> dc_rulekit::Result<Value> {
        Ok(params.clone())
    }
}

struct CapturingAudit {
    receipts: Arc<Mutex<Vec<dc_rulekit::AuditReceipt>>>,
}
impl AuditHook for CapturingAudit {
    fn on_receipt(&self, receipt: &dc_rulekit::AuditReceipt) {
        self.receipts.lock().unwrap().push(receipt.clone());
    }
}

fn registry_with_demo() -> (PluginRegistry, Arc<Mutex<Vec<String>>>) {
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut reg = PluginRegistry::new();
    reg.register_condition(Box::new(AlwaysCondition));
    reg.register_condition(Box::new(NeverCondition));
    reg.register_action(Box::new(LogAction {
        log: Arc::clone(&log),
    }));
    reg.register_action(Box::new(PureEchoAction));
    (reg, log)
}

#[test]
fn evaluate_runs_actions_when_conditions_pass() {
    let (reg, log) = registry_with_demo();
    let engine = Engine::new(&reg);
    let mut rule = Rule::new("demo.app/rule-1", "Test", RuleSource::Static);
    rule.when.push(Condition {
        id: "c1".into(),
        plugin: "demo.when.always".into(),
        params: json!({}),
    });
    rule.then.push(Action {
        id: "a1".into(),
        plugin: "demo.then.log".into(),
        params: json!({ "message": "hello" }),
    });

    let receipt = engine
        .evaluate(&rule, &EvalContext::new("demo.app"), EvaluateOptions::default())
        .unwrap();
    assert!(receipt.matched);
    assert_eq!(log.lock().unwrap().len(), 1);
}

#[test]
fn dry_run_skips_impure_actions() {
    let (reg, log) = registry_with_demo();
    let engine = Engine::new(&reg);
    let mut rule = Rule::new("demo.app/rule-2", "Dry", RuleSource::Static);
    rule.when.push(Condition {
        id: "c1".into(),
        plugin: "demo.when.always".into(),
        params: json!({}),
    });
    rule.then.push(Action {
        id: "a1".into(),
        plugin: "demo.then.log".into(),
        params: json!({}),
    });

    let receipt = engine
        .evaluate(
            &rule,
            &EvalContext::new("demo.app"),
            EvaluateOptions { dry_run: true },
        )
        .unwrap();
    assert!(receipt.matched);
    assert!(receipt.action_outcomes[0].skipped_dry_run);
    assert!(!receipt.action_outcomes[0].executed);
    assert!(log.lock().unwrap().is_empty());
}

#[test]
fn dry_run_runs_pure_actions() {
    let (reg, _log) = registry_with_demo();
    let engine = Engine::new(&reg);
    let mut rule = Rule::new("demo.app/rule-3", "Pure dry", RuleSource::Static);
    rule.when.push(Condition {
        id: "c1".into(),
        plugin: "demo.when.always".into(),
        params: json!({}),
    });
    rule.then.push(Action {
        id: "a1".into(),
        plugin: "demo.then.echo".into(),
        params: json!({ "x": 1 }),
    });

    let receipt = engine
        .evaluate(
            &rule,
            &EvalContext::new("demo.app"),
            EvaluateOptions { dry_run: true },
        )
        .unwrap();
    assert!(receipt.action_outcomes[0].executed);
}

#[test]
fn unknown_plugin_fail_closed() {
    let (reg, _) = registry_with_demo();
    let engine = Engine::new(&reg);
    let mut rule = Rule::new("demo.app/rule-4", "Bad", RuleSource::Static);
    rule.when.push(Condition {
        id: "c1".into(),
        plugin: "unknown.plugin".into(),
        params: json!({}),
    });

    let err = engine
        .evaluate(&rule, &EvalContext::new("demo.app"), EvaluateOptions::default())
        .unwrap_err();
    assert!(matches!(
        err,
        RulekitError::UnknownConditionPlugin { .. }
    ));
}

#[test]
fn proposal_approve_and_store_roundtrip() {
    let mut proposals = ProposalStore::in_memory();
    let mut active = RuleStore::in_memory();
    let mut rule = Rule::new("demo.app/rule-prop", "Prop", RuleSource::Static);
    rule.when.push(Condition {
        id: "c1".into(),
        plugin: "demo.when.always".into(),
        params: json!({}),
    });

    let proposal = proposals.propose(rule).unwrap();
    assert_eq!(proposal.status, dc_rulekit::ProposalStatus::Proposed);

    let active_rule = proposals.approve(&proposal.proposal_id, &mut active).unwrap();
    assert_eq!(active_rule.id, "demo.app/rule-prop");
    assert!(active.get("demo.app/rule-prop").is_ok());
}

#[test]
fn store_file_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("rules.json");
    let mut rule = Rule::new("demo.app/file", "File", RuleSource::Static);
    rule.then.push(Action {
        id: "a1".into(),
        plugin: "demo.then.log".into(),
        params: json!({}),
    });

    {
        let mut store = RuleStore::with_file_path(&path);
        store.upsert(rule.clone()).unwrap();
    }
    let store = RuleStore::load_from_disk(&path).unwrap();
    assert_eq!(store.get("demo.app/file").unwrap().title, "File");
}

#[test]
fn audit_hook_receives_opaque_payload() {
    let (reg, _) = registry_with_demo();
    let engine = Engine::new(&reg);
    let mut rule = Rule::new("demo.app/audit", "Audit", RuleSource::Static);
    rule.when.push(Condition {
        id: "c1".into(),
        plugin: "demo.when.always".into(),
        params: json!({}),
    });
    rule.then.push(Action {
        id: "a1".into(),
        plugin: "demo.then.echo".into(),
        params: json!({}),
    });

    let audit = CapturingAudit {
        receipts: Arc::new(Mutex::new(Vec::new())),
    };
    let opaque = json!({ "host": "payload", "trace": "abc" });
    engine
        .evaluate_with_audit(
            &rule,
            &EvalContext::new("demo.app"),
            EvaluateOptions::default(),
            &audit,
            opaque.clone(),
        )
        .unwrap();

    let captured = audit.receipts.lock().unwrap();
    assert_eq!(captured.len(), 1);
    assert_eq!(captured[0].opaque, opaque);
}

#[test]
fn disabled_rule_errors() {
    let (reg, _) = registry_with_demo();
    let engine = Engine::new(&reg);
    let mut rule = Rule::new("demo.app/dis", "Dis", RuleSource::Static);
    rule.enabled = false;

    let err = engine
        .evaluate(&rule, &EvalContext::new("demo.app"), EvaluateOptions::default())
        .unwrap_err();
    assert!(matches!(err, RulekitError::DisabledRule { .. }));
}
