use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use crate::conditions::{conditions_from_v1_when, Condition, ConditionNode, RuleEvent};

/// Current rule document schema version written by this crate.
pub const CURRENT_SCHEMA_VERSION: u32 = 2;

/// Schema versions accepted when reading rule JSON (v1 normalized to v2 in memory).
pub const READABLE_SCHEMA_VERSIONS: &[u32] = &[1, 2];

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
        /// Cron-like or ISO8601 duration string — interpreted by the host scheduler (not core runner).
        expression: String,
    },
}

impl Default for Trigger {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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
    pub conditions: ConditionNode,
    #[serde(default)]
    pub events: Vec<RuleEvent>,
}

fn default_enabled() -> bool {
    true
}

/// Back-compat type alias (v0.1 name).
pub type Action = RuleEvent;

impl Rule {
    pub fn new(id: impl Into<String>, title: impl Into<String>, source: RuleSource) -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            id: id.into(),
            title: title.into(),
            source,
            enabled: true,
            trigger: Trigger::default(),
            conditions: ConditionNode::default(),
            events: Vec::new(),
        }
    }

    pub fn validate_schema(&self) -> crate::error::Result<()> {
        if self.schema_version != CURRENT_SCHEMA_VERSION {
            return Err(crate::error::RulekitError::SchemaVersionMismatch {
                expected: CURRENT_SCHEMA_VERSION,
                found: self.schema_version,
            });
        }
        self.conditions.validate_shape()?;
        for ev in &self.events {
            if ev.id.is_empty() || ev.event_type.is_empty() {
                return Err(crate::error::RulekitError::EvaluationError {
                    message: "event requires non-empty id and type".into(),
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct RuleCompatRaw {
    schema_version: Option<u32>,
    id: String,
    title: String,
    source: RuleSource,
    #[serde(default = "default_enabled")]
    enabled: bool,
    #[serde(default)]
    trigger: Trigger,
    #[serde(default)]
    conditions: Option<ConditionNode>,
    #[serde(default)]
    when: Vec<Condition>,
    #[serde(default)]
    events: Vec<RuleEvent>,
    #[serde(default)]
    then: Vec<RuleEventV1>,
}

#[derive(Debug, Deserialize)]
struct RuleEventV1 {
    id: String,
    #[serde(default)]
    plugin: Option<String>,
    #[serde(rename = "type", default)]
    event_type: Option<String>,
    #[serde(default)]
    params: serde_json::Value,
}

impl RuleEventV1 {
    fn into_v2(self) -> RuleEvent {
        let event_type = self
            .event_type
            .or(self.plugin)
            .unwrap_or_default();
        RuleEvent {
            id: self.id,
            event_type,
            params: self.params,
        }
    }
}

fn normalize_rule_raw(raw: RuleCompatRaw) -> crate::error::Result<Rule> {
    let version = raw.schema_version.unwrap_or(CURRENT_SCHEMA_VERSION);
    if !READABLE_SCHEMA_VERSIONS.contains(&version) {
        return Err(crate::error::RulekitError::SchemaVersionMismatch {
            expected: CURRENT_SCHEMA_VERSION,
            found: version,
        });
    }

    let conditions = if let Some(c) = raw.conditions {
        c
    } else if !raw.when.is_empty() || version == 1 {
        conditions_from_v1_when(raw.when)
    } else {
        ConditionNode::default()
    };

    let events = if !raw.events.is_empty() {
        raw.events
    } else {
        raw.then.into_iter().map(RuleEventV1::into_v2).collect()
    };

    Ok(Rule {
        schema_version: CURRENT_SCHEMA_VERSION,
        id: raw.id,
        title: raw.title,
        source: raw.source,
        enabled: raw.enabled,
        trigger: raw.trigger,
        conditions,
        events,
    })
}

impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RuleCompatRaw::deserialize(deserializer)?;
        normalize_rule_raw(raw).map_err(serde::de::Error::custom)
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
    #[serde(rename = "type")]
    pub event_type: String,
    pub executed: bool,
    pub skipped_dry_run: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}
