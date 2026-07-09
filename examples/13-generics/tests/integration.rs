//! Integration tests for `example_generics`.

use example_generics::{Pair, SecureContainer, filter_owned, find_max, format_reading};

#[test]
fn container_and_pair_public_api() {
    let c = SecureContainer::new("id", 7u16);
    assert_eq!(c.audit_line(), "[id] 7");
    assert_eq!(c.clone_inner(), 7);

    let p = Pair::new("a", "b");
    assert_eq!(p.swap().left, "b");
}

#[test]
fn generic_helpers() {
    assert_eq!(find_max(&[1, 3, 2]), Some(&3));
    assert_eq!(format_reading("x", 1), "x=1");
    let v = filter_owned(vec![1, 2, 3, 4], |n| n % 2 == 0);
    assert_eq!(v, vec![2, 4]);
}
