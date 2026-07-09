//! Smart pointer exercises — recursive policy rules with `Box`, `Rc`, and `RefCell`.
//!
//! # Modules
//!
//! - [`rule`] — recursive `Rule` enum with `Box`
//! - [`config`] — `SharedConfig` with `Rc` and `RefCell`
//! - [`engine`] — `PolicyEngine` tying rules to shared config
//! - [`rules_demo`] — Exercise 1: evaluate rule trees
//! - [`sharing`] — Exercise 2: shared hits via interior mutability
//! - [`clones`] — Exercise 3: cheap engine clones

pub mod clones;
pub mod config;
pub mod engine;
pub mod rule;
pub mod rules_demo;
pub mod sharing;

pub use config::SharedConfig;
pub use engine::PolicyEngine;
pub use rule::{Rule, sample_policy};

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
            name: "rules",
            description: "Evaluate recursive Allow/Deny/AndThen/Or trees",
            concepts: vec![
                "Box breaks infinite enum size for recursive types",
                "AndThen requires both branches to allow",
                "Or permits when either branch allows",
            ],
        },
        ExerciseInfo {
            name: "sharing",
            description: "Share policy config and hit counter with Rc/RefCell",
            concepts: vec![
                "Rc enables multiple owners on one thread",
                "RefCell provides interior mutability through &self",
                "Rc::clone bumps strong count, not deep copy",
            ],
        },
        ExerciseInfo {
            name: "clones",
            description: "Clone PolicyEngine without duplicating config",
            concepts: vec![
                "PolicyEngine::clone shares the Rc<SharedConfig>",
                "Rule trees remain owned once per engine",
                "Hit counter accumulates across all engines",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "rules" => rules_demo::run(verbose),
        "sharing" => sharing::run(verbose),
        "clones" => clones::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: rules, sharing, clones"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["rules", "sharing", "clones"];

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
