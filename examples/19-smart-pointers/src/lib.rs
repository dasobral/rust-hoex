//! Smart pointers: `Box`, `Rc`, and `RefCell`.
//!
//! # When to use which
//!
//! | Type       | Owns? | Shared? | Mutability              | Threads? |
//! |------------|-------|---------|-------------------------|----------|
//! | `Box<T>`   | yes   | no      | exclusive (`&mut`)      | `Send` if `T: Send` |
//! | `Rc<T>`    | shared| yes     | immutable by default    | **no** (use `Arc`) |
//! | `RefCell<T>` | wraps | —     | runtime borrow checks   | **no** (use `Mutex`) |
//!
//! - **`Box`**: heap allocation, recursive types (`enum` with children), trait objects.
//! - **`Rc`**: multiple owners of the same value on **one** thread.
//! - **`RefCell`**: interior mutability — mutate through a shared `&` when the
//!   borrow checker cannot prove exclusivity at compile time.
//! - **`Arc`**: thread-safe `Rc` (atomic refcount). Pair with `Mutex`/`RwLock`
//!   instead of `RefCell` across threads.
//!
//! Theme: a recursive **security-rule tree** (`Box`) plus a **shared config**
//! (`Rc`) with an access **counter** (`RefCell`).

use std::cell::RefCell;
use std::rc::Rc;

/// A leaf or branch in a security-policy AST.
///
/// Recursive variants need `Box` — without indirection the type would have
/// infinite size (`Allow` contains another `Rule`, which contains…).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    /// Permit a named action (e.g. `"read"`).
    Allow(String),
    /// Deny a named action.
    Deny(String),
    /// All children must pass.
    All(Vec<Box<Self>>),
    /// At least one child must pass.
    Any(Vec<Box<Self>>),
    /// Invert the child result.
    Not(Box<Self>),
}

impl Rule {
    /// Evaluate whether `action` is permitted by this rule tree.
    #[must_use]
    pub fn allows(&self, action: &str) -> bool {
        match self {
            Self::Allow(name) => name == action,
            Self::Deny(name) => name != action,
            Self::All(kids) => kids.iter().all(|r| r.allows(action)),
            Self::Any(kids) => kids.iter().any(|r| r.allows(action)),
            Self::Not(inner) => !inner.allows(action),
        }
    }

    /// Count nodes in the tree (demonstrates walking through `Box`).
    #[must_use]
    pub fn node_count(&self) -> usize {
        match self {
            Self::Allow(_) | Self::Deny(_) => 1,
            Self::All(kids) | Self::Any(kids) => {
                1 + kids.iter().map(|r| r.node_count()).sum::<usize>()
            }
            Self::Not(inner) => 1 + inner.node_count(),
        }
    }
}

/// Shared application config. Cheap to clone via `Rc` (refcount bump only).
#[derive(Debug)]
pub struct SharedConfig {
    /// Human-readable environment name.
    pub env: String,
    /// How many times the config was consulted (interior mutability).
    hits: RefCell<u64>,
}

impl SharedConfig {
    /// Build a new shared config wrapped in `Rc`.
    #[must_use]
    pub fn new(env: impl Into<String>) -> Rc<Self> {
        Rc::new(Self {
            env: env.into(),
            hits: RefCell::new(0),
        })
    }

    /// Record one access and return the new total.
    ///
    /// `RefCell` lets us mutate `hits` through `&self` (shared borrow of `Rc`).
    /// A panic occurs only if we already hold a conflicting borrow — keep
    /// borrow scopes short.
    pub fn record_hit(&self) -> u64 {
        let mut hits = self.hits.borrow_mut();
        *hits = hits.saturating_add(1);
        *hits
    }

    /// Current hit count (immutable borrow of the cell).
    #[must_use]
    pub fn hits(&self) -> u64 {
        *self.hits.borrow()
    }
}

/// A policy engine that shares one config among many rule evaluators.
#[derive(Debug, Clone)]
pub struct PolicyEngine {
    config: Rc<SharedConfig>,
    root: Rule,
}

impl PolicyEngine {
    /// Create an engine. Cloning the engine shares the same `Rc` config.
    #[must_use]
    pub const fn new(config: Rc<SharedConfig>, root: Rule) -> Self {
        Self { config, root }
    }

    /// Evaluate `action`, bumping the shared hit counter.
    #[must_use]
    pub fn check(&self, action: &str) -> bool {
        let _ = self.config.record_hit();
        self.root.allows(action)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Rule {
        // Allow read|list, and reject delete (Not(Allow("delete"))).
        Rule::All(vec![
            Box::new(Rule::Any(vec![
                Box::new(Rule::Allow("read".into())),
                Box::new(Rule::Allow("list".into())),
            ])),
            Box::new(Rule::Not(Box::new(Rule::Allow("delete".into())))),
        ])
    }

    #[test]
    fn box_enables_recursive_rule_tree() {
        let tree = sample_tree();
        assert!(tree.allows("read"));
        assert!(!tree.allows("write"));
        assert!(tree.node_count() >= 5);
    }

    #[test]
    fn rc_shares_config_across_engines() {
        let cfg = SharedConfig::new("staging");
        let a = PolicyEngine::new(Rc::clone(&cfg), Rule::Allow("read".into()));
        let b = PolicyEngine::new(Rc::clone(&cfg), Rule::Deny("write".into()));
        // cfg + a + b => 3 owners
        assert_eq!(a.config_owners(), 3);
        assert_eq!(b.config().env, "staging");
        drop(cfg);
        assert_eq!(a.config_owners(), 2);
    }

    #[test]
    fn refcell_counts_hits_through_shared_refs() {
        let cfg = SharedConfig::new("prod");
        let engine = PolicyEngine::new(Rc::clone(&cfg), Rule::Allow("ping".into()));
        assert!(engine.check("ping"));
        assert!(engine.check("ping"));
        assert_eq!(cfg.hits(), 2);
    }
}
