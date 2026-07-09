//! Integration tests for `example_smartpointers`.

use std::rc::Rc;

use example_smartpointers::{PolicyEngine, Rule, SharedConfig};

#[test]
fn engine_shares_config_and_evaluates() {
    let cfg = SharedConfig::new("test");
    let engine = PolicyEngine::new(
        Rc::clone(&cfg),
        Rule::Any(vec![
            Box::new(Rule::Allow("get".into())),
            Box::new(Rule::Allow("head".into())),
        ]),
    );
    assert!(engine.check("get"));
    assert!(!engine.check("post"));
    assert_eq!(cfg.hits(), 2);
}
