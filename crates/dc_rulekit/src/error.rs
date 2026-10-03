use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RulekitError {
    #[error("unknown condition plugin: {plugin_id}")]
    UnknownConditionPlugin { plugin_id: String },

    #[error("unknown action plugin: {plugin_id}")]
    UnknownActionPlugin { plugin_id: String },

    #[error("rule is disabled: {rule_id}")]
    DisabledRule { rule_id: String },

    #[error("condition failed: rule={rule_id}, condition={condition_id}, plugin={plugin_id}")]
    ConditionFailed {
        rule_id: String,
        condition_id: String,
        plugin_id: String,
    },

    #[error("action denied: rule={rule_id}, action={action_id}, plugin={plugin_id}, reason={reason}")]
    ActionDenied {
        rule_id: String,
        action_id: String,
        plugin_id: String,
        reason: String,
    },

    #[error("schema version mismatch: expected={expected}, found={found}")]
    SchemaVersionMismatch { expected: u32, found: u32 },

    #[error("rule not found: {rule_id}")]
    RuleNotFound { rule_id: String },

    #[error("proposal not found: {proposal_id}")]
    ProposalNotFound { proposal_id: String },

    #[error("invalid proposal state: {message}")]
    InvalidProposalState { message: String },

    #[error("store error: {message}")]
    StoreError { message: String },

    #[error("evaluation error: {message}")]
    EvaluationError { message: String },

    #[error("invalid plugin params: plugin={plugin_id}, {message}")]
    InvalidPluginParams {
        plugin_id: String,
        message: String,
    },
}

pub type Result<T> = std::result::Result<T, RulekitError>;
