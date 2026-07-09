//! Integration tests for `exercise_unsafe`.

use unsafe_exercises::{max_i32, read_at, run_demo, write_at};

#[test]
fn read_write_round_trip() {
    let mut buf = [0_u8; 4];
    assert_eq!(write_at(&mut buf, 2, 42), Some(()));
    assert_eq!(read_at(&buf, 2), Some(42));
}

#[test]
fn max_i32_integration() {
    assert_eq!(max_i32(&[-3_i32, 0, 12, 5]), Some(12));
}

#[test]
fn demo_runs() {
    assert!(run_demo(false).is_ok());
}
