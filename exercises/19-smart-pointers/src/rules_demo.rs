//! Exercise 1 — recursive `Rule` trees with `Box`.

use anyhow::Result;

use crate::rule::{Rule, sample_policy};

/// Walk a sample policy and evaluate several subjects.
pub fn run(verbose: bool) -> Result<()> {
    println!("🌳 Rules — Box-backed recursive policy tree");
    println!();

    let policy = sample_policy();
    let subjects = ["read", "list", "write", "delete"];

    println!("  policy nodes: {}", policy.node_count());
    for subject in subjects {
        let allowed = policy.allows(subject);
        println!("  {subject}: {}", if allowed { "ALLOW" } else { "DENY" });
    }

    if verbose {
        println!();
        println!("  Tree shape: AndThen( Or(Allow read, Allow list), Deny delete )");
        let leaf = Rule::Allow("audit".into());
        println!("  single Allow node count: {}", leaf.node_count());
    }

    Ok(())
}
