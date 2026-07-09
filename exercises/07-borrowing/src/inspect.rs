//! Inspect exercise — immutable borrows of passwords.

use anyhow::Result;

use crate::password::{count_digits, first_char, mask_keep_last, meets_policy};

/// Run the inspect exercise with demo output.
pub fn run(verbose: bool) -> Result<()> {
    let password = "Tr0ub4dor&3";
    println!("🔍 Inspect — Read Passwords Without Ownership");
    println!();
    println!("Password (borrowed): \"{password}\"");

    if let Some(first) = first_char(password) {
        println!("- first character: {first:?}");
    }

    println!("- digit count:     {}", count_digits(password));
    println!("- meets policy:    {}", meets_policy(password, 8));
    println!("- masked display:  {}", mask_keep_last(password, 2));

    if verbose {
        println!();
        println!("   Multiple `&str` borrows can coexist while the owner lives.");
        println!("   Masking allocates a new `String`; the original is untouched.");
    }

    Ok(())
}
