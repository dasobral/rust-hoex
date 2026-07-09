//! Integration tests for `example_unsafe`.

use example_unsafe::{read_at, sum_i32};

#[test]
fn safe_api_hides_unsafe() {
    let xs = [1_i32, 2, 3];
    assert_eq!(read_at(&xs, 0), Some(1));
    assert_eq!(read_at(&xs, 3), None);
    assert_eq!(sum_i32(&xs), 6);
}
