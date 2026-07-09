//! Lifetimes exercises — tying returned references to their inputs.
//!
//! # Why returning `&str` to a local fails
//!
//! A function like `fn broken() -> &str { let s = String::from("x"); &s }` cannot
//! compile: `s` is dropped at the end of the function, so the returned reference
//! would dangle. Lifetimes name the relationship between inputs and outputs so
//! the borrow checker rejects this at compile time. When you need to outlive the
//! function, return an **owned** `String` instead — see [`owned_summary`].

pub mod excerpt;
pub mod shortest;
pub mod summary;

pub use excerpt::ImportantExcerpt;
pub use shortest::shortest;
pub use summary::owned_summary;

pub type Result<T> = anyhow::Result<T>;

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
            name: "shortest",
            description: "Compare two log lines with shared lifetime `'a`",
            concepts: vec![
                "Explicit lifetime on multiple inputs and output",
                "Both borrows must outlive the result",
            ],
        },
        ExerciseInfo {
            name: "excerpt",
            description: "Store borrowed text in ImportantExcerpt<'a>",
            concepts: vec![
                "Lifetime on struct fields",
                "last_word borrows from input text",
            ],
        },
        ExerciseInfo {
            name: "summary",
            description: "Build owned summaries to avoid dangling references",
            concepts: vec![
                "Return String instead of &str when data is assembled locally",
                "Why &str to local String fails to compile",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "shortest" => shortest::run(verbose),
        "excerpt" => excerpt::run(verbose),
        "summary" => summary::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: shortest, excerpt, summary"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["shortest", "excerpt", "summary"];

    for (i, name) in exercises.iter().enumerate() {
        println!("⏳ Exercise {} of {}: {name}", i + 1, exercises.len());
        println!("{}", "=".repeat(50));
        run_exercise(name, verbose)?;
        if i < exercises.len() - 1 {
            println!("\n{}\n", "─".repeat(50));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_picks_shorter() {
        assert_eq!(shortest("disk pressure alert", "ok"), "ok");
    }

    #[test]
    fn owned_summary_joins() {
        let parts = ["CRITICAL", "disk", "full"];
        assert_eq!(owned_summary(&parts), "CRITICAL / disk / full");
    }
}
