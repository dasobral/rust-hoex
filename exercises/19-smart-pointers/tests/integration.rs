//! Integration tests for `exercise_smartpointers`.

use std::rc::Rc;

use smartpointers_exercises::{
    PolicyEngine, Rule, SharedConfig, get_exercise_list, run_all, run_exercise, sample_policy,
};

#[test]
fn exercise_list_has_three_entries() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 3);
    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"rules"));
    assert!(names.contains(&"sharing"));
    assert!(names.contains(&"clones"));
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("rules", false).is_ok());
    assert!(run_exercise("sharing", false).is_ok());
    assert!(run_exercise("clones", false).is_ok());
}

#[test]
fn run_unknown_exercise_errors() {
    let err = run_exercise("firewall", false);
    assert!(err.is_err());
    if let Err(e) = err {
        assert!(format!("{e}").contains("Unknown exercise"));
    }
}

#[test]
fn run_all_succeeds() {
    assert!(run_all(false).is_ok());
}

#[test]
fn policy_engine_end_to_end() {
    let policy = sample_policy();
    assert!(policy.allows("read"));
    assert!(!policy.allows("delete"));

    let cfg = SharedConfig::new("integration-test");
    let engine = PolicyEngine::new(Rc::clone(&cfg), policy);
    assert!(engine.check("list"));
    assert_eq!(cfg.hits(), 1);
    assert_eq!(engine.config_owners(), 2);
}

#[test]
fn rule_or_and_then_composition() {
    let rule = Rule::AndThen(
        Box::new(Rule::Or(
            Box::new(Rule::Allow("audit".into())),
            Box::new(Rule::Allow("scan".into())),
        )),
        Box::new(Rule::Deny("exfil".into())),
    );
    assert!(rule.allows("audit"));
    assert!(!rule.allows("exfil"));
}
