//! Iterator adapter exercises — firewall log line processing.
//!
//! # Modules
//!
//! - [`log_line`] — `LogLine` and core iterator helpers
//! - [`adapters`] — Exercise 1: `filter` + `map` + `sum`
//! - [`filters`] — Exercise 2: `collect` and `count`
//! - [`pipeline`] — Exercise 3: `inspect` side effects

pub mod adapters;
pub mod filters;
pub mod log_line;
pub mod pipeline;

pub use log_line::{
    LogLine, allowed_bytes, count_allows, denied_ips, inspect_action_counts, sample_logs,
};

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
            name: "adapters",
            description: "Sum allowed bytes with filter → map → sum",
            concepts: vec![
                "Iterator adapters are lazy until consumed",
                "filter keeps matching LogLine references",
                "map transforms to u64, sum consumes the chain",
            ],
        },
        ExerciseInfo {
            name: "filters",
            description: "Collect denied IPs and count ALLOW lines",
            concepts: vec![
                "filter + map + collect into Vec<String>",
                "filter().count() without allocating",
                "Borrowing &[LogLine] across adapter chains",
            ],
        },
        ExerciseInfo {
            name: "pipeline",
            description: "Tally actions with inspect before for_each",
            concepts: vec![
                "inspect runs per item during consumption",
                "Side effects without changing iterator items",
                "Chaining enumerate with inspect for tracing",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "adapters" => adapters::run(verbose),
        "filters" => filters::run(verbose),
        "pipeline" => pipeline::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: adapters, filters, pipeline"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["adapters", "filters", "pipeline"];

    for (i, name) in exercises.iter().enumerate() {
        println!("🛡️  Exercise {} of {}: {name}", i + 1, exercises.len());
        println!("{}", "=".repeat(50));
        run_exercise(name, verbose)?;
        if i < exercises.len() - 1 {
            println!("\n{}\n", "─".repeat(50));
        }
    }

    Ok(())
}
