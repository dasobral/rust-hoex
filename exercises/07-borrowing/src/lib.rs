//! Borrowing exercises — inspect and update passwords without taking ownership.
//!
//! Demonstrates shared references (`&T`), mutable references (`&mut T`), and
//! slices in a cybersecurity password-audit context.

pub mod inspect;
pub mod password;
pub mod policy;

pub use password::{
    average_scores, count_digits, first_char, mask_keep_last, meets_policy, update_strength,
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
            name: "inspect",
            description: "Read password traits via immutable borrows",
            concepts: vec![
                "Shared references (`&str`)",
                "Multiple simultaneous immutable borrows",
                "Safe display via masking",
                "Character iteration without ownership",
            ],
        },
        ExerciseInfo {
            name: "policy",
            description: "Score passwords and aggregate threat history",
            concepts: vec![
                "Mutable references (`&mut i32`)",
                "Exclusive borrow rules",
                "Slice borrows (`&[i32]`)",
                "Policy checks without moving data",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "inspect" => inspect::run(verbose),
        "policy" => policy::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: inspect, policy"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["inspect", "policy"];

    for (i, exercise_name) in exercises.iter().enumerate() {
        println!(
            "🔍 Exercise {} of {}: {}",
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
