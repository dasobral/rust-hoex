//! Integration tests for `example_iterators`.

use example_iterators::{denied_ips, parse_logs, total_bytes};

#[test]
fn public_parse_and_aggregate() {
    let logs = parse_logs(&[
        "src=1.1.1.1 action=DENY bytes=10",
        "src=2.2.2.2 action=ALLOW bytes=5",
    ]);
    assert_eq!(logs.len(), 2);
    assert_eq!(denied_ips(&logs), vec!["1.1.1.1"]);
    assert_eq!(total_bytes(&logs), 15);
}
