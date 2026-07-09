//! Integration tests for `example_macros`.

use std::collections::HashMap;

use example_macros::{config_or, count_exprs, maplit, password_score, testvec};

#[test]
fn macros_and_helpers_work_together() {
    let cases = testvec![("Aa1aaaaa", 4_u32), ("tiny", 0_u32)];
    for (input, expected) in cases {
        assert_eq!(password_score(input), expected);
    }
    let m: HashMap<&str, u32> = maplit! { "x" => 1 };
    assert_eq!(config_or(&m, "x", 0), 1);
    assert_eq!(count_exprs!(10, 20), 2);
}
