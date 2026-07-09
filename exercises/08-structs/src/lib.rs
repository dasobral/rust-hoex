//! Struct exercises — user accounts, credentials, and session tokens.
//!
//! Cybersecurity-themed drills for custom types, method receivers,
//! private fields, and struct update patterns.

pub mod account;
pub mod auth_flow;
pub mod lockout;
pub mod session;

pub use account::{Role, UserAccount};
pub use session::Session;

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
            name: "auth-flow",
            description: "Authenticate users and mint session tokens",
            concepts: vec![
                "Struct fields and private credentials",
                "Method receivers (&self, &mut self, self)",
                "Session token matching",
                "Account lockout after failed attempts",
            ],
        },
        ExerciseInfo {
            name: "lockout",
            description: "Simulate credential-stuffing lockout policy",
            concepts: vec![
                "Associated constants (LOCK_THRESHOLD)",
                "Mutable state via &mut self",
                "Role-based privilege checks",
                "Audit line generation",
            ],
        },
    ]
}

/// Run one exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "auth-flow" => auth_flow::run(verbose),
        "lockout" => lockout::run(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: auth-flow, lockout"),
    }
}

/// Run all exercises in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["auth-flow", "lockout"];

    for (i, exercise_name) in exercises.iter().enumerate() {
        println!(
            "🔐 Exercise {} of {}: {}",
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
        let list = get_exercise_list();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn run_unknown_exercise_errors() {
        assert!(run_exercise("missing", false).is_err());
    }
}
