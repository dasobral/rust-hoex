//! Integration tests for `exercise_macros`.

use macros_exercises::{max_of, password_score, run_macro_demo, testvec};

#[test]
fn demo_runs() {
    assert!(run_macro_demo(false).is_ok());
}

#[test]
fn max_of_via_crate_root() {
    assert_eq!(max_of!(10_i32, 3, 7), 10);
}

#[test]
fn testvec_fixture_drives_scoring() {
    let cases = testvec![("Password1", 4_u32), ("x", 0_u32)];
    for (input, expected) in cases {
        assert_eq!(password_score(input), expected);
    }
}
