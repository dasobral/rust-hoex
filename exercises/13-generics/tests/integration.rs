//! Integration tests for `exercise_generics`.

use generics_exercises::{
    Pair, SecureContainer, find_min, get_exercise_list, run_all, run_exercise,
};

#[test]
fn exercise_list_has_three_entries() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 3);
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("container", false).is_ok());
    assert!(run_exercise("pair", false).is_ok());
    assert!(run_exercise("search", false).is_ok());
}

#[test]
fn run_all_succeeds() {
    assert!(run_all(false).is_ok());
}

#[test]
fn secure_container_map_and_clear() {
    let c = SecureContainer::new("nonce", String::from("abc"));
    let len = c.map(|s| s.len()).into_inner();
    assert_eq!(len, 3);

    let mut buf = SecureContainer::new("buf", vec![1_u8, 2]);
    buf.clear();
    assert!(buf.get().is_empty());
}

#[test]
fn pair_eq_parts() {
    let a = Pair::new("10.0.0.1", 9_u8);
    let b = Pair::new("10.0.0.1", 9_u8);
    assert!(a.eq_parts(&b));
}

#[test]
fn find_min_across_types() {
    let nums = [5_i32, 2, 8];
    assert_eq!(find_min(&nums), Some(&2));
}
