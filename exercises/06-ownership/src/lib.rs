//! Ownership exercises — secure credential handling.
//!
//! Demonstrates moves, `Clone`, `Copy`, and intentional consumption of secrets
//! in a cybersecurity context.

pub mod move_vs_copy;
pub mod secrets;
pub mod zeroize;

pub use secrets::{
    clone_secret, consume_secret, copy_threat_score, take_then_return, zeroize_and_consume,
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
            name: "move_vs_copy",
            description: "Credential handoff, Clone, and Copy threat scores",
            concepts: vec![
                "Move semantics for `String`",
                "Explicit `Clone` for backup copies",
                "`Copy` types (`i32`) vs non-`Copy` types",
                "Consume-on-use security pattern",
            ],
        },
        ExerciseInfo {
            name: "zeroize",
            description: "Scrub secret bytes before drop",
            concepts: vec![
                "Heap buffer zeroization",
                "Scope and `Drop`",
                "Safe scrubbing via `into_bytes`",
                "Ephemeral credential lifecycle",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "move_vs_copy" => move_vs_copy::run(verbose),
        "zeroize" => zeroize::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: move_vs_copy, zeroize"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["move_vs_copy", "zeroize"];

    for (i, exercise_name) in exercises.iter().enumerate() {
        println!(
            "🔐 Exercise {} of {}: {}",
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
