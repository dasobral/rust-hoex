//! Generics exercises — type parameters and trait bounds for security tooling.

pub mod container;
pub mod pair;
pub mod search;

pub use container::SecureContainer;
pub use pair::Pair;
pub use search::find_min;

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
            name: "container",
            description: "Wrap secrets and scores in SecureContainer<T>",
            concepts: vec![
                "Generic struct SecureContainer<T>",
                "Trait-bound impl blocks (Default, map)",
                "Monomorphization at compile time",
            ],
        },
        ExerciseInfo {
            name: "pair",
            description: "Compare Pair<A, B> values with eq_parts",
            concepts: vec![
                "Multiple type parameters",
                "Eq bounds on impl blocks",
                "Generic methods on Pair",
            ],
        },
        ExerciseInfo {
            name: "search",
            description: "Find minimum threat scores with find_min",
            concepts: vec![
                "Generic functions with PartialOrd",
                "Returning Option<&T> from a slice",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "container" => container::run(verbose),
        "pair" => pair::run(verbose),
        "search" => search::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: container, pair, search"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["container", "pair", "search"];

    for (i, name) in exercises.iter().enumerate() {
        println!("📦 Exercise {} of {}: {name}", i + 1, exercises.len());
        println!("{}", "=".repeat(50));
        run_exercise(name, verbose)?;
        if i < exercises.len() - 1 {
            println!("\n{}\n", "─".repeat(50));
        }
    }

    Ok(())
}
