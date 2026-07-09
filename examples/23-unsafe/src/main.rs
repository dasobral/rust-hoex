//! example: 23-unsafe
//!
//! Tiny, documented `unsafe` blocks behind safe wrappers.
//!
//! **Advanced / rarely needed.** Prefer safe Rust until a profiler or API
//! boundary forces otherwise.
//!
//! ```bash
//! cargo run
//! ```

use example_unsafe::{demo_values, read_at, sum_i32};

fn main() {
    println!("=== 23-unsafe: raw pointers + safe wrappers ===\n");
    println!("WARNING: unsafe is advanced — don't use it until you must.\n");

    let data = [4_i32, 8, 15, 16, 23, 42];

    match read_at(&data, 2) {
        Some(v) => println!("read_at(2) = {v}  (safe API, unsafe inside)"),
        None => println!("read_at(2) out of bounds"),
    }

    match read_at(&data, 100) {
        Some(v) => println!("unexpected value {v}"),
        None => println!("read_at(100) = None  (bounds check rejected)"),
    }

    let (first_safe, first_raw, total) = demo_values(&data);
    println!("first via slice: {first_safe:?}");
    println!("first via raw:   {first_raw:?}");
    println!("sum_i32:         {total} (iterator sum would be {})", {
        let mut s = 0_i32;
        for n in data {
            s = s.saturating_add(n);
        }
        s
    });
    println!("direct sum_i32:  {}", sum_i32(&data));

    println!("\nPattern: prove invariants → tiny unsafe → safe public API.");
}
