use crate::error::{Result, RulekitError};
use serde::{Deserialize, Serialize};

/// Leaf condition: host plugin invocation (json-rules-engine–style tree leaf).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    pub id: String,
    /// Plugin id (alias `fact` accepted on ingest for LLM-friendly JSON).
    #[serde(alias = "fact")]
    pub plugin: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

/// Nested condition tree (`all` / `any` / `not`), aligned with json-rules-engine conventions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionNode {
    All {
        all: Vec<ConditionNode>,
    },
    Any {
        any: Vec<ConditionNode>,
    },
    Not {
        not: Box<ConditionNode>,
    },
    Leaf(Condition),
}

impl Default for ConditionNode {
    fn default() -> Self {
        Self::All { all: Vec::new() }
    }
}

impl ConditionNode {
    pub fn all(nodes: Vec<ConditionNode>) -> Self {
        Self::All { all: nodes }
    }

    pub fn any(nodes: Vec<ConditionNode>) -> Self {
        Self::Any { any: nodes }
    }

    pub fn not(inner: ConditionNode) -> Self {
        Self::Not {
            not: Box::new(inner),
        }
    }

    pub fn leaf(condition: Condition) -> Self {
        Self::Leaf(condition)
    }

    /// Flat list of plugin leaves (pre-order).
    pub fn leaves(&self) -> Vec<&Condition> {
        let mut out = Vec::new();
        self.collect_leaves(&mut out);
        out
    }

    fn collect_leaves<'a>(&'a self, out: &mut Vec<&'a Condition>) {
        match self {
            Self::All { all } => {
                for child in all {
                    child.collect_leaves(out);
                }
            }
            Self::Any { any } => {
                for child in any {
                    child.collect_leaves(out);
                }
            }
            Self::Not { not } => not.collect_leaves(out),
            Self::Leaf(c) => out.push(c),
        }
    }

    pub fn validate_shape(&self) -> Result<()> {
        match self {
            Self::All { all } | Self::Any { any: all } => {
                for child in all {
                    child.validate_shape()?;
                }
                Ok(())
            }
            Self::Not { not } => not.validate_shape(),
            Self::Leaf(c) => {
                if c.id.is_empty() || c.plugin.is_empty() {
                    return Err(RulekitError::EvaluationError {
                        message: "condition leaf requires non-empty id and plugin".into(),
                    });
                }
                Ok(())
            }
        }
    }
}

/// Event-style action (`type` + `params`), familiar from json-rules-engine `event` objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleEvent {
    pub id: String,
    #[serde(rename = "type", alias = "plugin")]
    pub event_type: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

impl RuleEvent {
    pub fn plugin_id(&self) -> &str {
        &self.event_type
    }
}

/// v0.1 flat `when` array → v0.2 `conditions.all` wrapper.
pub fn conditions_from_v1_when(when: Vec<Condition>) -> ConditionNode {
    ConditionNode::All {
        all: when.into_iter().map(ConditionNode::Leaf).collect(),
    }
}
