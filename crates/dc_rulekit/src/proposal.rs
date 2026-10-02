use crate::error::{Result, RulekitError};
use crate::model::{ProposalStatus, Rule, RuleProposal};
use crate::store::RuleStore;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Proposal workflow: propose → approve → durable active rule in [`RuleStore`].
#[derive(Debug, Default)]
pub struct ProposalStore {
    proposals: HashMap<String, RuleProposal>,
    path: Option<PathBuf>,
}

impl ProposalStore {
    pub fn in_memory() -> Self {
        Self {
            proposals: HashMap::new(),
            path: None,
        }
    }

    pub fn with_file_path(path: impl Into<PathBuf>) -> Self {
        Self {
            proposals: HashMap::new(),
            path: Some(path.into()),
        }
    }

    pub fn load_from_disk(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut store = Self::with_file_path(path.clone());
        if path.exists() {
            let data = fs::read_to_string(&path).map_err(|e| RulekitError::StoreError {
                message: e.to_string(),
            })?;
            let proposals: Vec<RuleProposal> =
                serde_json::from_str(&data).map_err(|e| RulekitError::StoreError {
                    message: e.to_string(),
                })?;
            for p in proposals {
                p.rule.validate_schema()?;
                store.proposals.insert(p.proposal_id.clone(), p);
            }
        }
        Ok(store)
    }

    fn persist(&self) -> Result<()> {
        let Some(path) = &self.path else {
            return Ok(());
        };
        let proposals: Vec<&RuleProposal> = self.proposals.values().collect();
        let data = serde_json::to_string_pretty(&proposals).map_err(|e| RulekitError::StoreError {
            message: e.to_string(),
        })?;
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| RulekitError::StoreError {
                    message: e.to_string(),
                })?;
            }
        }
        fs::write(path, data).map_err(|e| RulekitError::StoreError {
            message: e.to_string(),
        })?;
        Ok(())
    }

    pub fn propose(&mut self, rule: Rule) -> Result<RuleProposal> {
        rule.validate_schema()?;
        let proposal = RuleProposal::new(rule);
        self.proposals
            .insert(proposal.proposal_id.clone(), proposal.clone());
        self.persist()?;
        Ok(proposal)
    }

    pub fn get(&self, proposal_id: &str) -> Result<&RuleProposal> {
        self.proposals
            .get(proposal_id)
            .ok_or_else(|| RulekitError::ProposalNotFound {
                proposal_id: proposal_id.to_string(),
            })
    }

    pub fn reject(&mut self, proposal_id: &str) -> Result<RuleProposal> {
        let proposal = self.proposals.get_mut(proposal_id).ok_or_else(|| {
            RulekitError::ProposalNotFound {
                proposal_id: proposal_id.to_string(),
            }
        })?;
        if proposal.status != ProposalStatus::Proposed {
            return Err(RulekitError::InvalidProposalState {
                message: format!("cannot reject proposal in state {:?}", proposal.status),
            });
        }
        proposal.status = ProposalStatus::Rejected;
        let out = proposal.clone();
        self.persist()?;
        Ok(out)
    }

    /// Approve a proposal and persist the rule into the active store.
    pub fn approve(
        &mut self,
        proposal_id: &str,
        active_store: &mut RuleStore,
    ) -> Result<Rule> {
        let proposal = self.proposals.get_mut(proposal_id).ok_or_else(|| {
            RulekitError::ProposalNotFound {
                proposal_id: proposal_id.to_string(),
            }
        })?;
        if proposal.status != ProposalStatus::Proposed {
            return Err(RulekitError::InvalidProposalState {
                message: format!("cannot approve proposal in state {:?}", proposal.status),
            });
        }
        proposal.status = ProposalStatus::Approved;
        let rule = proposal.rule.clone();
        active_store.upsert(rule.clone())?;
        self.persist()?;
        Ok(rule)
    }
}
