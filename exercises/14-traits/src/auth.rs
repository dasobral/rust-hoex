//! Exercise 1 — `AuthFailure` and trait default methods.

use anyhow::Result;

use crate::scoring::{AuthFailure, ThreatScorer, analyze_event, is_actionable};

/// Run the auth exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔐 Auth — ThreatScorer for failed logins");
    println!();

    let events = [
        AuthFailure {
            user: String::from("alice"),
            attempts: 2,
            from_known_host: true,
        },
        AuthFailure {
            user: String::from("bob"),
            attempts: 6,
            from_known_host: false,
        },
    ];

    for event in &events {
        println!("  {}", analyze_event(event));
        println!(
            "    actionable={} critical={}",
            is_actionable(event),
            event.is_critical()
        );
    }

    if verbose {
        println!();
        println!("  Default risk_level bands: 0-24 low, 25-59 medium,");
        println!("  60-84 high, 85+ critical. is_critical: score≥85 or Critical.");
    }

    Ok(())
}
