//! Exercise 2 — `Rc<SharedConfig>` and `RefCell` hit counting.

use std::rc::Rc;

use anyhow::Result;

use crate::config::SharedConfig;
use crate::engine::PolicyEngine;
use crate::rule::Rule;

/// Demonstrate shared config and interior mutability for hit counts.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔗 Sharing — Rc config + RefCell hits");
    println!();

    let cfg = SharedConfig::new("soc-edge-policy");
    let engine_a = PolicyEngine::new(Rc::clone(&cfg), Rule::Allow("read".into()));
    let engine_b = PolicyEngine::new(Rc::clone(&cfg), Rule::Deny("delete".into()));

    let _ = engine_a.check("read");
    let _ = engine_b.check("write");
    let _ = engine_a.check("list");

    println!("  policy name: {}", cfg.name());
    println!(
        "  config owners (cfg + 2 engines): {}",
        engine_a.config_owners()
    );
    println!("  shared hits after checks: {}", cfg.hits());

    if verbose {
        println!();
        let counter = cfg.hit_counter();
        println!("  hit counter strong count: {}", Rc::strong_count(&counter));
        println!("  engine_a allows read: {}", engine_a.check("read"));
        println!("  engine_b allows delete: {}", engine_b.check("delete"));
        println!("  hits after verbose checks: {}", cfg.hits());
    }

    Ok(())
}
