use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Current rule document schema version written by this crate.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleSource {
    Static,
    Llm,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Trigger {
    Manual,
    Event {
        #[serde(default)]
        topic: String,
    },
    Schedule {
        /// Cron-like or ISO8601 duration string — interpreted by the host scheduler (not v0.1 runner).
        expression: String,
    },
}

impl Default for Trigger {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    pub id: String,
    pub plugin: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub plugin: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub schema_version: u32,
    pub id: String,
    pub title: String,
    pub source: RuleSource,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub trigger: Trigger,
    #[serde(default)]
    pub when: Vec<Condition>,
    #[serde(default)]
    pub then: Vec<Action>,
}

fn default_enabled() -> bool {
    true
}

impl Rule {
    pub fn new(id: impl Into<String>, title: impl Into<String>, source: RuleSource) -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            id: id.into(),
            title: title.into(),
            source,
            enabled: true,
            trigger: Trigger::default(),
            when: Vec::new(),
            then: Vec::new(),
        }
    }

    pub fn validate_schema(&self) -> crate::error::Result<()> {
        if self.schema_version != CURRENT_SCHEMA_VERSION {
            return Err(crate::error::RulekitError::SchemaVersionMismatch {
                expected: CURRENT_SCHEMA_VERSION,
                found: self.schema_version,
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProposalStatus {
    Proposed,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleProposal {
    pub proposal_id: String,
    pub status: ProposalStatus,
    pub proposed_at: DateTime<Utc>,
    pub rule: Rule,
}

impl RuleProposal {
    pub fn new(rule: Rule) -> Self {
        Self {
            proposal_id: Uuid::new_v4().to_string(),
            status: ProposalStatus::Proposed,
            proposed_at: Utc::now(),
            rule,
        }
    }
}

/// Opaque audit/receipt payload emitted after evaluation (host interprets bytes/JSON).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditReceipt {
    pub rule_id: String,
    pub matched: bool,
    pub dry_run: bool,
    pub condition_results: Vec<ConditionOutcome>,
    pub action_outcomes: Vec<ActionOutcome>,
    #[serde(default)]
    pub opaque: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionOutcome {
    pub condition_id: String,
    pub plugin: String,
    pub passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionOutcome {
    pub action_id: String,
    pub plugin: String,
    pub executed: bool,
    pub skipped_dry_run: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}
