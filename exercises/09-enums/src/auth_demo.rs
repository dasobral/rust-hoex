//! Authentication status walkthrough exercise.

use anyhow::Result;

use crate::auth_status::AuthStatus;

/// Run the auth-status exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔑 Auth Status — Enum Variants for Identity Outcomes");
    println!();

    let outcomes = [
        AuthStatus::Success {
            user: "alice".to_owned(),
        },
        AuthStatus::Failure {
            reason: "invalid MFA code".to_owned(),
        },
        AuthStatus::Pending,
        AuthStatus::Locked {
            until: "2026-07-10T08:00:00Z".to_owned(),
        },
    ];

    for status in &outcomes {
        let icon = match status {
            AuthStatus::Success { .. } => "[ok]",
            AuthStatus::Failure { .. } => "[no]",
            AuthStatus::Pending => "[..]",
            AuthStatus::Locked { .. } => "[lock]",
        };
        println!(
            "{icon} {} (authenticated={})",
            status.summary(),
            status.is_authenticated()
        );
    }

    if verbose {
        println!();
        println!("Exhaustive match ensures every variant is handled at compile time.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_demo_runs() {
        assert!(run(false).is_ok());
    }
}
