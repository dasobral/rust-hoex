//! Testing exercises — password policy library built to be tested.

pub mod policy;
pub mod rules;

pub use policy::{PolicyReport, assert_nonempty, check_or_err, check_policy, scan_candidates};
pub use rules::{MIN_LENGTH, has_digit, has_min_length, has_punctuation, has_uppercase};

pub type Result<T> = anyhow::Result<T>;

/// Sample passwords for CLI and integration demos.
pub const SAMPLE_PASSWORDS: &[&str] = &["", "short1A", "NoPunct1", "Vault!2026"];

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
    vec![ExerciseInfo {
        name: "policy",
        description: "Check sample passwords and print policy reports",
        concepts: vec![
            "Unit tests per rule in rules.rs",
            "Integration tests in tests/",
            "#[ignore] for slow candidate scans",
            "#[should_panic] only inside tests",
        ],
    }]
}

/// Run the policy demo — prints check results for sample passwords.
pub fn run_policy_demo(verbose: bool) -> Result<()> {
    println!("Password policy check demo\n");

    for pw in SAMPLE_PASSWORDS {
        let label = if pw.is_empty() { "(empty)" } else { pw };
        let report = check_policy(pw);
        println!("  candidate: {label}");
        println!("  ok={}", report.ok);
        if report.ok {
            println!("  violations: (none)");
        } else {
            println!("  violations: {}", report.violations.join(", "));
        }
        if verbose {
            match check_or_err(pw) {
                Ok(()) => println!("  check_or_err: Ok"),
                Err(e) => println!("  check_or_err: Err({e})"),
            }
        }
        println!();
    }

    Ok(())
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "policy" => run_policy_demo(verbose),
        _ => anyhow::bail!("Unknown exercise: {name}. Available: policy"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    run_policy_demo(verbose)
}

#[cfg(test)]
mod slow {
    use super::*;

    /// Slow integration-style scan — run with `cargo test -- --ignored`.
    #[test]
    #[ignore = "slow brute-force style candidate scan"]
    fn slow_scan_finds_weak_candidates() {
        let candidates: Vec<&str> = (0..5_000)
            .map(|i| match i {
                0 => "password",
                1 => "12345678",
                2 => "Vault!2026",
                3 => "short1A",
                _ => "candidate",
            })
            .collect();
        let weak = scan_candidates(&candidates, |pw| !check_policy(pw).ok);
        assert!(!weak.is_empty());
    }
}
