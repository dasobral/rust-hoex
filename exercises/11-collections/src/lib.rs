//! Collections exercises — intrusion log aggregation with `Vec`, `HashMap`, and `HashSet`.
//!
//! # Modules
//!
//! - [`intrusion`] — `LogEvent`, `IntrusionLog`, and aggregation helpers
//! - [`aggregator`] — Exercise 1: building a log with `Vec`
//! - [`analysis`] — Exercise 2: counts and sets
//! - [`investigation`] — Exercise 3: filtering and ranking

pub mod aggregator;
pub mod analysis;
pub mod intrusion;
pub mod investigation;

pub use intrusion::{IntrusionLog, LogEvent, sample_log};

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
            name: "aggregator",
            description: "Build and inspect a Vec-backed intrusion log",
            concepts: vec![
                "Vec::push and len/is_empty",
                "Borrowing slices with events()",
                "Ownership of LogEvent values",
            ],
        },
        ExerciseInfo {
            name: "analysis",
            description: "Count IPs and deduplicate users/actions",
            concepts: vec![
                "HashMap entry API for ip_counts",
                "HashSet for unique_users and actions",
                "hot_ips as an alias of ip_counts",
            ],
        },
        ExerciseInfo {
            name: "investigation",
            description: "Filter events and identify the top offender",
            concepts: vec![
                "Iterator filter + collect into Vec",
                "HashMap for per-user counts",
                "Deterministic tie-breaking in top_user",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "aggregator" => aggregator::run(verbose),
        "analysis" => analysis::run(verbose),
        "investigation" => investigation::run(verbose),
        _ => anyhow::bail!(
            "Unknown exercise: {name}. Available: aggregator, analysis, investigation"
        ),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["aggregator", "analysis", "investigation"];

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
