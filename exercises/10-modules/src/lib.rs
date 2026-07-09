//! Modules exercises — multi-file password analysis with visibility control.
//!
//! # Module tree
//!
//! ```text
//! modules_exercises (lib.rs)
//! └── analyzer/
//!     ├── mod.rs       — Analysis, analyze
//!     ├── score.rs     — pub(crate) compute_score
//!     ├── risk.rs      — RiskLevel
//!     └── validate.rs  — pub(crate) is_too_common
//! ```

pub mod analyzer;

pub use analyzer::{Analysis, RiskLevel, analyze};

pub type Result<T> = anyhow::Result<T>;

/// Return the risk label for an analysis — re-export style public helper.
#[must_use]
pub const fn risk_label(a: &Analysis) -> &'static str {
    a.risk.as_str()
}

/// Exercise metadata for the CLI.
#[derive(Debug, Clone)]
pub struct ExerciseInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub concepts: Vec<&'static str>,
}

/// List available exercises in this crate.
#[must_use]
pub fn get_exercise_list() -> Vec<ExerciseInfo> {
    vec![
        ExerciseInfo {
            name: "score",
            description: "Score sample passwords and inspect risk labels",
            concepts: vec![
                "pub vs pub(crate) visibility",
                "Re-exported analyze from crate root",
                "Score capping for common passwords",
            ],
        },
        ExerciseInfo {
            name: "validate",
            description: "Check secrets against the common-password denylist",
            concepts: vec![
                "Case-insensitive denylist matching",
                "validate module as pub(crate) internals",
                "Public API via analyze and risk_label",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "score" => score::run(verbose),
        "validate" => validate::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: score, validate"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["score", "validate"];

    for (i, name) in exercises.iter().enumerate() {
        println!("🔐 Exercise {} of {}: {name}", i + 1, exercises.len());
        println!("{}", "=".repeat(50));
        run_exercise(name, verbose)?;
        if i < exercises.len() - 1 {
            println!("\n{}\n", "─".repeat(50));
        }
    }

    Ok(())
}

pub mod score;
pub mod validate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risk_label_matches_analysis() {
        let a = analyze("password");
        assert_eq!(risk_label(&a), "critical");
    }
}
