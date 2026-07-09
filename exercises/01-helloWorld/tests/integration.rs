//! Integration tests for the `exercise_helloworld` crate.

use helloworld_exercises::{
    get_exercise_list, greet, join_lines, mask_secret, run_all, run_exercise, security_banner,
};

#[test]
fn test_exercise_list_contains_both() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 2);

    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"greet"));
    assert!(names.contains(&"banner"));

    for exercise in &list {
        assert!(!exercise.description.is_empty());
        assert!(!exercise.concepts.is_empty());
    }
}

#[test]
fn test_run_each_exercise() {
    assert!(run_exercise("greet", "Analyst", "SecCheck", false).is_ok());
    assert!(run_exercise("banner", "Analyst", "nmap", false).is_ok());
}

#[test]
fn test_run_unknown_exercise_errors() {
    let err = run_exercise("scan", "Analyst", "SecCheck", false);
    assert!(err.is_err());
    if let Err(e) = err {
        let message = format!("{e}");
        assert!(message.contains("Unknown exercise"));
    }
}

#[test]
fn test_run_all() {
    assert!(run_all("Analyst", "SecCheck", false).is_ok());
}

#[test]
fn test_greet_and_mask_workflow() {
    let greeting = greet("BlueTeam");
    assert_eq!(greeting, "Hello, BlueTeam!");

    let log_block = join_lines(&[
        &greeting,
        "Event: login-success",
        &format!("Token: {}", mask_secret("s3cr3t-t0k3n")),
    ]);
    assert!(log_block.contains("Hello, BlueTeam!"));
    assert!(log_block.contains("***********"));
    assert!(!log_block.contains("s3cr3t-t0k3n"));
}

#[test]
fn test_security_banner_for_tools() {
    for tool in ["nmap", "wireshark", "SecCheck"] {
        let banner = security_banner(tool);
        assert!(banner.contains(tool));
        assert!(banner.contains("Security Operations Console"));
    }
}
