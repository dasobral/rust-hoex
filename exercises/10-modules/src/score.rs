//! Exercise 1: demonstrate scoring via the public `analyze` API.

use crate::{Result, analyze, risk_label};

const SAMPLES: &[&str] = &[
    "",
    "password",
    "123456",
    "S3cret!",
    "correct-horse-battery-staple-99!",
];

/// Run the score demo.
pub fn run(verbose: bool) -> Result<()> {
    println!("Password scoring demo (public API only)\n");

    for secret in SAMPLES {
        let label = if secret.is_empty() { "(empty)" } else { secret };
        let analysis = analyze(secret);
        println!("  input: {label}");
        println!("  {}", analysis.summary());
        println!("  risk_label: {}", risk_label(&analysis));
        if verbose {
            println!(
                "  note: score capped at 10 when common={}",
                analysis.too_common
            );
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_demo_runs() {
        assert!(run(false).is_ok());
    }
}
