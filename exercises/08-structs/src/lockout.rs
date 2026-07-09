//! Account lockout policy demonstration.

use anyhow::Result;

use crate::account::{Role, UserAccount};

/// Run the lockout-policy exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("🛡️  Lockout Policy — Failed Login Thresholds");
    println!();
    println!(
        "Default threshold: {} failed attempts",
        UserAccount::LOCK_THRESHOLD
    );

    let mut viewer = UserAccount::new(
        "scanner-bot",
        "bot@scanner.local",
        "probe-token",
        Role::Viewer,
    );

    println!();
    println!(
        "Simulating credential stuffing against '{}':",
        viewer.username
    );
    for i in 1..=UserAccount::LOCK_THRESHOLD + 1 {
        let accepted = viewer.verify_credential("wrong-password");
        if !accepted {
            viewer.record_failed_login();
        }
        println!(
            "  probe {i}: accepted={accepted}, fails={}, locked={}",
            viewer.failed_logins, viewer.locked
        );
        if viewer.locked {
            break;
        }
    }

    if verbose {
        println!();
        println!("Post-lockout state:");
        println!("  is_admin: {}", viewer.is_admin());
        println!("  can_authenticate: {}", viewer.can_authenticate());
        let audit =
            UserAccount::new("audit", "audit@secops.local", "n/a", Role::Admin).into_audit_line();
        println!("  sample audit: {audit}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lockout_run_succeeds() {
        assert!(run(false).is_ok());
    }
}
