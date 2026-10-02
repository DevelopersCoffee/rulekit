use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Host-provided evaluation context (domain-free bag of facts).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EvalContext {
    #[serde(default)]
    pub app_namespace: String,
    #[serde(default)]
    pub facts: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub event: Option<serde_json::Value>,
}

impl EvalContext {
    pub fn new(app_namespace: impl Into<String>) -> Self {
        Self {
            app_namespace: app_namespace.into(),
            facts: HashMap::new(),
            event: None,
        }
    }

    pub fn with_fact(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.facts.insert(key.into(), value);
        self
    }
}
