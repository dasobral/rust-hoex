//! Traits exercises — `ThreatScorer` trait practice for security events.
//!
//! # Modules
//!
//! - [`scoring`] — `ThreatScorer`, event types, and `max_score`
//! - [`auth`] — Exercise 1: authentication failures
//! - [`network`] — Exercise 2: network scans
//! - [`malware`] — Exercise 3: malware and file integrity

pub mod auth;
pub mod malware;
pub mod network;
pub mod scoring;

pub use scoring::{
    AuthFailure, FileIntegrityEvent, MalwareAlert, NetworkScan, RiskLevel, ThreatScorer,
    analyze_event, is_actionable, max_score,
};

pub type Result<T> = anyhow::Result<T>;

/// Exercise metadata for the CLI.
#[derive(Debug, Clone)]
pub struct ExerciseInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub concepts: Vec<&'static str>,
}

/// List available exercises.
#[must_use]
pub fn get_exercise_list() -> Vec<ExerciseInfo> {
    vec![
        ExerciseInfo {
            name: "auth",
            description: "Score failed authentication attempts",
            concepts: vec![
                "ThreatScorer required methods",
                "Default risk_level and is_critical",
                "Static dispatch with trait bounds",
            ],
        },
        ExerciseInfo {
            name: "network",
            description: "Network scan scoring with summary override",
            concepts: vec![
                "Overriding default trait methods",
                "max_score over homogeneous slices",
                "is_actionable helper",
            ],
        },
        ExerciseInfo {
            name: "malware",
            description: "Malware alerts and file integrity events",
            concepts: vec![
                "MalwareAlert risk_level confidence floor",
                "FileIntegrityEvent implementor",
                "Critical vs High band documentation",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "auth" => auth::run(verbose),
        "network" => network::run(verbose),
        "malware" => malware::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: auth, network, malware"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["auth", "network", "malware"];

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
