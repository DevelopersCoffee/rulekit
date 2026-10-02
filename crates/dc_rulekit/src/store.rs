use crate::error::{Result, RulekitError};
use crate::model::Rule;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// In-memory rule store with optional JSON file persistence.
#[derive(Debug, Default)]
pub struct RuleStore {
    rules: HashMap<String, Rule>,
    path: Option<PathBuf>,
}

impl RuleStore {
    pub fn in_memory() -> Self {
        Self {
            rules: HashMap::new(),
            path: None,
        }
    }

    pub fn with_file_path(path: impl Into<PathBuf>) -> Self {
        Self {
            rules: HashMap::new(),
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
            let rules: Vec<Rule> = serde_json::from_str(&data).map_err(|e| RulekitError::StoreError {
                message: e.to_string(),
            })?;
            for rule in rules {
                rule.validate_schema()?;
                store.rules.insert(rule.id.clone(), rule);
            }
        }
        Ok(store)
    }

    pub fn persist(&self) -> Result<()> {
        let Some(path) = &self.path else {
            return Ok(());
        };
        let rules: Vec<&Rule> = self.rules.values().collect();
        let data = serde_json::to_string_pretty(&rules).map_err(|e| RulekitError::StoreError {
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

    pub fn upsert(&mut self, rule: Rule) -> Result<()> {
        rule.validate_schema()?;
        self.rules.insert(rule.id.clone(), rule);
        self.persist()
    }

    pub fn get(&self, rule_id: &str) -> Result<&Rule> {
        self.rules
            .get(rule_id)
            .ok_or_else(|| RulekitError::RuleNotFound {
                rule_id: rule_id.to_string(),
            })
    }

    pub fn remove(&mut self, rule_id: &str) -> Result<Rule> {
        let rule = self.rules.remove(rule_id).ok_or_else(|| RulekitError::RuleNotFound {
            rule_id: rule_id.to_string(),
        })?;
        self.persist()?;
        Ok(rule)
    }

    pub fn list(&self) -> Vec<&Rule> {
        let mut v: Vec<_> = self.rules.values().collect();
        v.sort_by(|a, b| a.id.cmp(&b.id));
        v
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
