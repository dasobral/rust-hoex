//! example: 06-ownership
//!
//! Ownership rules through secure password / sensitive-data handling.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key concepts:
//! - Ownership and move semantics for `String`
//! - `Clone` when a second owned value is needed
//! - `Copy` types (`i32`) vs non-`Copy` types (`String`)
//! - Scope and `Drop`
//! - `String` vs `&str` (setup for borrowing)

use example_ownership::{
    clone_password, consume_password, double_score, owned_password, password_byte_len,
    take_ownership,
};

fn main() {
    println!("\nOwnership: Secure Password Handling");
    println!("====================================\n");

    // === String vs &str (brief setup for borrowing) ===
    //
    // `&str` is a borrowed string slice (view into UTF-8 bytes).
    // `String` is an owned, growable heap buffer.
    let literal: &str = "N0tARealP@ss!";
    let secret = owned_password(literal);
    println!("Owned password created from &str: \"{secret}\"");
    println!(
        "Byte length (borrowed view): {}",
        password_byte_len(&secret)
    );

    // === Move semantics ===
    //
    // Assigning a `String` *moves* ownership. The old binding is invalidated
    // so two owners cannot free the same heap buffer.
    let moved = take_ownership(secret);
    // `secret` is no longer valid here — uncommenting the next line fails:
    // println!("{secret}");
    println!("After move, new owner holds: \"{moved}\"");

    // === Clone when you need a second owned copy ===
    let backup = clone_password(&moved);
    println!("Clone kept a second owned copy: \"{backup}\"");
    println!("Original still usable after clone: \"{moved}\"");

    // === Consume (take ownership and drop) ===
    //
    // Security pattern: hand the password to a function that uses it once,
    // then drops it so the caller cannot accidentally reuse or log it.
    let consumed_len = consume_password(moved);
    println!("Consumed password ({consumed_len} bytes) — binding gone after drop");
    // `moved` cannot be used anymore. `backup` is still ours:
    println!("Backup clone still available: \"{backup}\"");

    // Drop the backup explicitly to show scope/drop control.
    drop(backup);
    println!("Backup dropped — heap buffer released\n");

    // === Copy types: i32 is duplicated, not moved ===
    let attempts: i32 = 3;
    let doubled = double_score(attempts);
    println!("Copy type demo (i32):");
    println!("- original attempts: {attempts}");
    println!("- doubled score:     {doubled}");
    println!("Both bindings remain valid because i32 implements Copy.\n");

    // === Scope and Drop ===
    {
        let scoped = owned_password("temp-secret");
        println!("Inside inner scope: \"{scoped}\"");
        // `scoped` is dropped at the end of this block.
    }
    println!("Inner-scope password dropped when the block ended.");

    // Recreate a password for a final consume demo with a fresh binding.
    let final_pw = owned_password("last-chance");
    let _ = consume_password(final_pw);
    println!("Final password consumed safely.\n");
}
