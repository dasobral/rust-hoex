//! Exercise 3 — filtering events and identifying top offenders.

use anyhow::Result;

use crate::intrusion::sample_log;

/// Run the investigation exercise: filter by action and find top user.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔍 Investigation — filter & rank suspects");
    println!();

    let log = sample_log();

    println!("  failed logins:");
    for event in log.filter_by_action("login_fail") {
        println!("    {} @ {} ({})", event.user, event.ip, event.action);
    }

    if let Some(top) = log.top_user() {
        println!();
        println!("  top user by volume: {top}");
    }

    if verbose {
        println!();
        println!("  Tie-break rule: highest count wins; on equal counts the");
        println!("  lexicographically smallest username is returned.");
    }

    Ok(())
}
