//! Exercise 2: demonstrate denylist validation outcomes.

use crate::analyzer::validate as denylist;
use crate::{Result, analyze};

const SAMPLES: &[&str] = &["password", "PaSsWoRd", "123456", "unique-vault-key"];

/// Run the validate demo.
pub fn run(verbose: bool) -> Result<()> {
    println!("Common-password validation demo\n");

    for secret in SAMPLES {
        let analysis = analyze(secret);
        let verdict = if analysis.too_common {
            "BLOCKED"
        } else {
            "allowed"
        };
        println!("  secret: {secret}");
        println!("  verdict: {verdict}");
        println!("  detail: {}", denylist::validation_message(secret));
        if verbose {
            println!("  score after cap: {}", analysis.score);
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_demo_runs() {
        assert!(run(false).is_ok());
    }
}
