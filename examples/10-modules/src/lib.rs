//! Password / token strength analysis organized as a small multi-file crate.
//!
//! # Module tree
//!
//! ```text
//! example_modules (crate root = lib.rs)
//! └── analyzer/           // pub module
//!     ├── mod.rs          // submodule declarations + shared types
//!     ├── score.rs        // pub(crate) helpers — crate-visible only
//!     └── report.rs       // pub(crate) formatting
//! ```
//!
//! This file is the **crate root**. It declares child modules with `mod` and
//! re-exports the everyday public API so callers can write
//! `use example_modules::analyze` without digging into the tree.

pub mod analyzer;

// Re-export the public surface. Callers should not need `analyzer::` paths
// for everyday use — that is the point of a tidy crate root.
pub use analyzer::{Analysis, RiskLevel, analyze};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_empty_is_critical() {
        let result = analyze("");
        assert_eq!(result.risk, RiskLevel::Critical);
        assert_eq!(result.score, 0);
    }

    #[test]
    fn analyze_strong_passphrase_is_low_risk() {
        let result = analyze("correct-horse-battery-staple-42!");
        assert!(result.score >= 70);
        assert!(matches!(result.risk, RiskLevel::Low | RiskLevel::Medium));
    }
}
