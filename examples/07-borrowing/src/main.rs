//! example: 07-borrowing
//!
//! References and borrowing — analyze passwords without taking ownership.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key concepts:
//! - Shared references (`&T`) and mutable references (`&mut T`)
//! - Borrow checker: many `&T` XOR one `&mut T`
//! - Slices: `&str` and `&[T]`
//! - Dangling references are prevented at compile time

use example_borrowing::{
    average_scores, count_ascii_letters, mask_password, meets_min_length, password_prefix,
    password_summary, update_strength_score,
};

fn main() {
    println!("\nBorrowing: Password Analysis Without Ownership");
    println!("==============================================\n");

    // We own the password once. Analysis borrows it — no moves required.
    let password = String::from("Tr0ub4dor&3");
    println!("Password (owned): \"{password}\"");

    // === Multiple immutable borrows are allowed simultaneously ===
    //
    // The borrow checker permits any number of shared (`&`) references at once,
    // as long as no mutable borrow is active.
    let len_borrow = password.len();
    let letters = count_ascii_letters(&password);
    let long_enough = meets_min_length(&password, 8);
    let (summary_len, summary_letters, summary_ok) = password_summary(&password);

    println!("Immutable borrows (many at once):");
    println!("- len via method:          {len_borrow}");
    println!("- ASCII letters:           {letters}");
    println!("- meets min length (8):    {long_enough}");
    println!("- summary: len={summary_len}, letters={summary_letters}, ok={summary_ok}");

    // Masking also borrows immutably and returns a new owned String.
    let masked = mask_password(&password, 2);
    println!("- masked for display:      {masked}");

    // === Mutable borrow exclusivity ===
    //
    // While `score` is mutably borrowed, you cannot also immutably borrow it
    // (or create a second `&mut`). One `&mut T` XOR many `&T`.
    let mut score: i32 = 0;
    println!("\nMutable borrow (exclusive):");
    println!("- score before: {score}");

    // Exclusive mutable borrow of `score`; shared borrow of `password` is fine
    // because they are different values.
    update_strength_score(&mut score, &password);
    println!("- score after update: {score}");

    // After the mutable borrow ends, shared borrows of `score` are OK again:
    let score_ref = &score;
    println!("- score via shared ref: {score_ref}");

    // The following would NOT compile — mutable and immutable at the same time:
    // let r1 = &score;
    // let r2 = &mut score;
    // println!("{r1} {r2}");

    // === Slices: &str and &[T] ===
    if let Some(prefix) = password_prefix(&password, 4) {
        println!("\nSlices:");
        println!("- &str prefix (4 bytes): \"{prefix}\"");
    }

    let history = [10_i32, 20, 25, 30];
    // `&history` coerces to `&[i32]` — a shared slice of the array.
    match average_scores(&history) {
        Some(avg) => println!("- average of score history {:?}: {avg}", &history[..]),
        None => println!("- score history was empty"),
    }

    // Password still owned here — borrowing never took it away.
    println!("\nOriginal password still owned: \"{password}\"");
    println!("Borrowing lets you inspect without consuming.\n");
}
