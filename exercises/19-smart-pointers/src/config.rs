//! Shared configuration with `Rc` and interior mutability via `RefCell`.

use std::cell::RefCell;
use std::rc::Rc;

/// Policy metadata shared across engines via `Rc`.
#[derive(Debug)]
pub struct SharedConfig {
    /// Human-readable policy name (cheaply cloned through `Rc`).
    name: Rc<String>,
    /// How many times any engine consulted this config.
    hits: Rc<RefCell<u64>>,
}

impl SharedConfig {
    /// Build a new shared config wrapped in `Rc`.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Rc<Self> {
        Rc::new(Self {
            name: Rc::new(name.into()),
            hits: Rc::new(RefCell::new(0)),
        })
    }

    /// Borrow the shared policy name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Record one access and return the new total.
    pub fn record_hit(&self) -> u64 {
        let mut hits = self.hits.borrow_mut();
        *hits = hits.saturating_add(1);
        *hits
    }

    /// Current hit count.
    #[must_use]
    pub fn hits(&self) -> u64 {
        *self.hits.borrow()
    }

    /// Shared hit counter handle (same cell as `record_hit`).
    #[must_use]
    pub fn hit_counter(&self) -> Rc<RefCell<u64>> {
        Rc::clone(&self.hits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_shares_name_and_hits() {
        let cfg = SharedConfig::new("soc-default");
        let clone = Rc::clone(&cfg);
        assert_eq!(cfg.name(), "soc-default");
        assert_eq!(Rc::strong_count(&cfg), 2);
        drop(clone);
        assert_eq!(Rc::strong_count(&cfg), 1);
    }

    #[test]
    fn refcell_tracks_hits() {
        let cfg = SharedConfig::new("prod");
        assert_eq!(cfg.record_hit(), 1);
        assert_eq!(cfg.record_hit(), 2);
        assert_eq!(cfg.hits(), 2);
    }

    #[test]
    fn hit_counter_is_shared() {
        let cfg = SharedConfig::new("staging");
        let counter = cfg.hit_counter();
        *counter.borrow_mut() = 5;
        assert_eq!(cfg.hits(), 5);
    }
}
