//! example: 19-smart-pointers
//!
//! Recursive security-rule trees (`Box`) and shared config (`Rc` + `RefCell`).
//!
//! ```bash
//! cargo run
//! ```

use std::rc::Rc;

use example_smartpointers::{PolicyEngine, Rule, SharedConfig};

fn main() {
    println!("=== 19-smart-pointers: Box / Rc / RefCell ===\n");

    // Box: recursive AST — children live on the heap.
    let policy = Rule::All(vec![
        Box::new(Rule::Any(vec![
            Box::new(Rule::Allow("read".into())),
            Box::new(Rule::Allow("list".into())),
        ])),
        Box::new(Rule::Not(Box::new(Rule::Allow("delete".into())))),
    ]);
    println!("rule tree has {} nodes", policy.node_count());
    println!("  allows read?   {}", policy.allows("read"));
    println!("  allows delete? {}", policy.allows("delete"));

    // Rc: many owners of one config; RefCell: mutate hit counter via &self.
    let config = SharedConfig::new("prod");
    let gateway = PolicyEngine::new(Rc::clone(&config), policy);
    let auditor = PolicyEngine::new(Rc::clone(&config), Rule::Allow("audit".into()));

    println!("\nshared config owners: {}", gateway.config_owners());
    println!("gateway check(read):  {}", gateway.check("read"));
    println!("auditor check(audit): {}", auditor.check("audit"));
    println!("total config hits:    {}", config.hits());
    println!("\n(Use Arc + Mutex when you need this pattern across threads.)");
}
