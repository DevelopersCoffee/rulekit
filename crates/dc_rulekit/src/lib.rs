//! Business-agnostic on-device rules engine.
//!
//! Host applications register [`ConditionEvaluator`] and [`ActionHandler`] plugins by string id.
//! The core validates, stores, and evaluates rules without domain-specific effects.

mod audit;
mod conditions;
mod context;
mod engine;
mod error;
mod model;
mod plugin;
mod proposal;
mod schema_validate;
mod store;

pub use audit::{AuditHook, NoopAuditHook};
pub use context::EvalContext;
pub use engine::{Engine, EvaluateOptions};
pub use error::{Result, RulekitError};
pub use conditions::{Condition, ConditionNode, RuleEvent};
pub use model::{
    Action, ActionOutcome, AuditReceipt, ConditionOutcome, ProposalStatus, Rule, RuleProposal,
    RuleSource, Trigger, CURRENT_SCHEMA_VERSION, READABLE_SCHEMA_VERSIONS,
};
pub use plugin::{ActionHandler, ConditionEvaluator, PluginRegistry};
pub use proposal::ProposalStore;
pub use store::RuleStore;
