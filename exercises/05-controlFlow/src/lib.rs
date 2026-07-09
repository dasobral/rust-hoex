//! Control flow exercises — threat scores and log classification.
//!
//! Demonstrates `match`, `if` expressions, loops, `break`/`continue`, and
//! `while let` in a security log analysis context.

mod batch;
pub mod classifier;
mod scoring;

pub use classifier::{
    Severity, accumulate_until_budget, cap_score, classify_batch, classify_score,
    requires_escalation, score_log_line, severity_label, walk_nonempty_lines,
};

pub type Result<T> = anyhow::Result<T>;

/// Exercise metadata for the CLI.
#[derive(Debug, Clone)]
pub struct ExerciseInfo {
    /// Static exercise identifier.
    pub name: &'static str,
    /// Short description shown in `list`.
    pub description: &'static str,
    /// Concepts covered by the exercise.
    pub concepts: Vec<&'static str>,
}

/// Return metadata for every exercise in this crate.
#[must_use]
pub fn get_exercise_list() -> Vec<ExerciseInfo> {
    vec![
        ExerciseInfo {
            name: "scoring",
            description: "Keyword scoring and severity mapping with match",
            concepts: vec![
                "`match` on integer ranges",
                "`if` expressions for capping",
                "Enum severity tiers",
                "Keyword heuristics",
            ],
        },
        ExerciseInfo {
            name: "batch",
            description: "Batch classification with loops and event limits",
            concepts: vec![
                "`for` loops with `break`",
                "Skipping blanks with `continue`",
                "`while let` iterator walks",
                "Budget-limited accumulation",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "scoring" => {
            scoring::run(verbose);
            Ok(())
        }
        "batch" => {
            batch::run(verbose);
            Ok(())
        }
        _ => anyhow::bail!("Unknown exercise: {name}. Available: scoring, batch"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["scoring", "batch"];

    for (i, exercise_name) in exercises.iter().enumerate() {
        println!(
            "🚨 Exercise {} of {}: {}",
            i + 1,
            exercises.len(),
            exercise_name
        );
        println!("{}", "=".repeat(50));

        run_exercise(exercise_name, verbose)?;

        if i < exercises.len() - 1 {
            println!("\n{}\n", "─".repeat(50));
        }
    }

    Ok(())
}
