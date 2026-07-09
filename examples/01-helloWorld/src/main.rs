//! example: 01-helloWorld
//!
//! Your first Rust program: printing to the console with macros.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key ideas:
//! - `fn main()` is the program entry point
//! - `println!` / `format!` / `dbg!` are macros (note the `!`)
//! - Library code in `lib.rs` can be reused by the binary and by tests

use example_helloworld::greet;

fn main() {
    // `println!` writes a line to stdout. The `!` means this is a macro,
    // not a regular function — it expands at compile time.
    println!("Hello from 01-helloWorld!");

    // Call our library helper. Separating logic into `lib.rs` lets tests
    // (and other crates) reuse the same code without running `main`.
    let message = greet("Rustacean");
    println!("{message}");

    // `format!` builds a `String` without printing it — useful when you
    // need the text for later (logging, return values, etc.).
    let topic = "macros";
    let detail = format!("Learning Rust {topic} today.");
    println!("{detail}");

    // Debug printing: `{:?}` uses the Debug trait; `dbg!` also shows
    // file/line and returns the value so you can inspect mid-expression.
    let numbers = [1, 2, 3];
    println!("numbers (Debug): {numbers:?}");
    let _sum = dbg!(1 + 2 + 3);
}
