//! Enum exercises — auth status, network events, HTTP codes, and tokens.

pub mod auth_demo;
pub mod auth_status;
pub mod http_status;
pub mod maybe_token;
pub mod network;
pub mod network_demo;

pub use auth_status::AuthStatus;
pub use http_status::HttpStatus;
pub use maybe_token::MaybeToken;
pub use network::{NetworkEvent, sum_packet_bytes};

pub type Result<T> = anyhow::Result<T>;

/// Metadata for a single hands-on exercise.
#[derive(Debug, Clone)]
pub struct ExerciseInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub concepts: Vec<&'static str>,
}

/// List every exercise in this crate.
#[must_use]
pub fn get_exercise_list() -> Vec<ExerciseInfo> {
    vec![
        ExerciseInfo {
            name: "auth-status",
            description: "Authentication outcomes with struct-like enum variants",
            concepts: vec![
                "Unit and struct-like enum variants",
                "Exhaustive match on AuthStatus",
                "is_authenticated and summary helpers",
                "Locked{until} for timed lockouts",
            ],
        },
        ExerciseInfo {
            name: "network",
            description: "Network events, byte totals, HTTP codes, and tokens",
            concepts: vec![
                "Tuple variants (PacketReceived, PacketSent)",
                "sum_packet_bytes over event slices",
                "HttpStatus::from_code mapping",
                "MaybeToken::unwrap_or fallback",
            ],
        },
    ]
}

/// Run one exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "auth-status" => auth_demo::run(verbose),
        "network" => network_demo::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: auth-status, network"),
    }
}

/// Run all exercises in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["auth-status", "network"];

    for (i, exercise_name) in exercises.iter().enumerate() {
        println!(
            "📡 Exercise {} of {}: {}",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_list_has_two_entries() {
        assert_eq!(get_exercise_list().len(), 2);
    }

    #[test]
    fn unknown_exercise_errors() {
        assert!(run_exercise("nope", false).is_err());
    }
}
