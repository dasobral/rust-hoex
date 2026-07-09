//! Closure exercises — threat event triage and filtering.
//!
//! # Modules
//!
//! - [`threat`] — `ThreatEvent` and closure helpers
//! - [`sorting`] — Exercise 1: `sort_by_key`
//! - [`partition`] — Exercise 2: `into_iter().partition`
//! - [`watchlist`] — Exercise 3: capturing `HashSet` in closures

pub mod partition;
pub mod sorting;
pub mod threat;
pub mod watchlist;

pub use threat::{
    ThreatEvent, count_matching, count_watchlisted, partition_critical, sample_events,
    sort_by_source,
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
            name: "sorting",
            description: "Order events by source with sort_by_key",
            concepts: vec![
                "sort_by_key extracts a comparable key per item",
                "Mutating slice sort reorders in place",
                "Closures as short inline comparators",
            ],
        },
        ExerciseInfo {
            name: "partition",
            description: "Split critical vs normal with partition",
            concepts: vec![
                "into_iter consumes the Vec",
                "partition preserves relative order within buckets",
                "Predicate closure decides bucket membership",
            ],
        },
        ExerciseInfo {
            name: "watchlist",
            description: "Filter events using a captured HashSet",
            concepts: vec![
                "Closures capture environment by reference",
                "count_matching accepts impl Fn predicate",
                "HashSet membership checks in O(1) average time",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "sorting" => sorting::run(verbose),
        "partition" => partition::run(verbose),
        "watchlist" => watchlist::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: sorting, partition, watchlist"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["sorting", "partition", "watchlist"];

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
