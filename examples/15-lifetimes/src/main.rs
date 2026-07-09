//! example: 15-lifetimes
//!
//! Lifetime annotations on functions that return references, and on structs
//! that store them.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```

use example_lifetimes::{ImportantExcerpt, find_in_haystack, longer_excerpt, longest};

fn main() {
    println!("=== 15-lifetimes: references that outlive the call ===\n");

    demo_longest();
    demo_haystack();
    demo_excerpt();
}

fn demo_longest() {
    let alert = String::from("disk pressure");
    let notice = "ok";
    // Both borrows must be valid for as long as `winner` is used.
    let winner = longest(alert.as_str(), notice);
    println!("longest message: {winner}");
}

fn demo_haystack() {
    let log_line = "src=10.0.0.5 dst=10.0.0.9 action=ALLOW";
    match find_in_haystack(log_line, "ALLOW") {
        Some(hit) => println!("found action token: {hit}"),
        None => println!("action token missing"),
    }
}

fn demo_excerpt() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = ImportantExcerpt::from_first_word(&novel);
    let second = ImportantExcerpt { part: "Ishmael" };
    let best = longer_excerpt(first, second);
    println!(
        "best excerpt: {} ({})",
        best.announce_and_return_part("highlight"),
        best.level()
    );
}
