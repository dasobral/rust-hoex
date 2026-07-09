//! Policy engine combining recursive rules with shared config.

use std::rc::Rc;

use crate::config::SharedConfig;
use crate::rule::Rule;

/// Evaluates a rule tree while sharing one `SharedConfig` across clones.
#[derive(Debug, Clone)]
pub struct PolicyEngine {
    config: Rc<SharedConfig>,
    root: Rule,
}

impl PolicyEngine {
    /// Create an engine. Cloning shares the same `Rc` config and hit counter.
    #[must_use]
    pub const fn new(config: Rc<SharedConfig>, root: Rule) -> Self {
        Self { config, root }
    }

    /// Evaluate `subject`, bumping the shared hit counter.
    #[must_use]
    pub fn check(&self, subject: &str) -> bool {
        let _ = self.config.record_hit();
        self.root.allows(subject)
    }

    /// Strong-count of the shared config (how many `Rc` owners exist).
    #[must_use]
    pub fn config_owners(&self) -> usize {
        Rc::strong_count(&self.config)
    }

    /// Borrow the shared config.
    #[must_use]
    pub fn config(&self) -> &SharedConfig {
        &self.config
    }

    /// Borrow the root rule tree.
    #[must_use]
    pub const fn root(&self) -> &Rule {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::{Rule, sample_policy};

    #[test]
    fn engine_shares_config_via_rc() {
        let cfg = SharedConfig::new("staging");
        let primary = PolicyEngine::new(Rc::clone(&cfg), sample_policy());
        let replica = PolicyEngine::new(Rc::clone(&cfg), Rule::Allow("ping".into()));
        assert_eq!(primary.config_owners(), 3);
        drop(replica);
        assert_eq!(primary.config_owners(), 2);
    }

    #[test]
    fn check_increments_shared_hits() {
        let cfg = SharedConfig::new("prod");
        let engine = PolicyEngine::new(Rc::clone(&cfg), Rule::Allow("read".into()));
        assert!(engine.check("read"));
        assert!(!engine.check("delete"));
        assert_eq!(cfg.hits(), 2);
    }
}
