//! Exercise 3 — clone engines without cloning rule trees.

use std::rc::Rc;

use anyhow::Result;

use crate::config::SharedConfig;
use crate::engine::PolicyEngine;
use crate::rule::sample_policy;

/// Demonstrate cheap engine clones sharing one config and counter.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔄 Clones — shared Rc owners across engines");
    println!();

    let cfg = SharedConfig::new("multi-tenant-soc");
    let base = PolicyEngine::new(Rc::clone(&cfg), sample_policy());
    let shadow = base.clone();

    let _ = base.check("read");
    let _ = shadow.check("delete");

    println!("  base config owners: {}", base.config_owners());
    println!("  shadow shares policy name: {}", shadow.config().name());
    println!("  combined hits: {}", cfg.hits());

    if verbose {
        println!();
        println!("  base root nodes: {}", base.root().node_count());
        println!("  shadow root nodes: {}", shadow.root().node_count());
        println!(
            "  both allow list: base={} shadow={}",
            base.check("list"),
            shadow.check("list")
        );
        println!("  hits after verbose pass: {}", cfg.hits());
    }

    Ok(())
}
