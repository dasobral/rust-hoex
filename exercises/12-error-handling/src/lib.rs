//! Error handling exercises — config and credential validation.
//!
//! # Modules
//!
//! - [`config`] — `ConfigError`, `ConfigPath`, path/port validation
//! - [`credentials`] — token and credential helpers
//! - [`bootstrap`] — `secure_bootstrap` pipeline
//! - [`paths`] — Exercise 1: path validation
//! - [`credentials_exercise`] — Exercise 2: Option/Result patterns
//! - [`bootstrap`] — also hosts Exercise 3 run function

pub mod bootstrap;
pub mod config;
pub mod credentials;
pub mod credentials_exercise;
pub mod paths;

pub use bootstrap::{bind_service, load_settings, secure_bootstrap};
pub use config::{ConfigError, ConfigPath, ensure_exists, parse_port, validate_config_path};
pub use credentials::{mask_token, optional_credential, parse_optional_token, require_credential};

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
            name: "paths",
            description: "Validate config paths and parse service ports",
            concepts: vec![
                "Custom ConfigError with thiserror",
                "validate_config_path and ensure_exists allow-list",
                "parse_port with InvalidPort variant",
            ],
        },
        ExerciseInfo {
            name: "credentials",
            description: "Parse optional tokens and required secrets",
            concepts: vec![
                "Option for absent values",
                "require_credential with EmptyCredential",
                "transpose pattern in optional_credential",
            ],
        },
        ExerciseInfo {
            name: "bootstrap",
            description: "Chain validation into secure_bootstrap",
            concepts: vec![
                "? operator for early return",
                "Combining path + credential checks",
                "load_settings with masked optional tokens",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "paths" => paths::run(verbose),
        "credentials" => credentials_exercise::run(verbose),
        "bootstrap" => bootstrap::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: paths, credentials, bootstrap"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["paths", "credentials", "bootstrap"];

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
