//! Zeroize exercise — scrub secrets before drop.

use anyhow::Result;

use crate::secrets::{consume_secret, zeroize_and_consume};

/// Run the zeroize exercise with demo output.
pub fn run(verbose: bool) -> Result<()> {
    println!("🧹 Zeroize — Scrub Secrets Before Drop");
    println!();

    let session_token = String::from("sess_9xk2mPqR8vL1");
    let plain_len = consume_secret(session_token);
    println!("1. Plain consume dropped {plain_len} bytes (no scrub)");

    let refresh_token = String::from("rt_4nB8wQjZ6hF0");
    let scrubbed_len = zeroize_and_consume(refresh_token);
    println!("2. Zeroized then dropped {scrubbed_len} bytes");

    {
        let temp = String::from("ephemeral-cred");
        let _ = zeroize_and_consume(temp);
        println!("3. Scoped secret zeroized at end of inner block");
    }
    println!("   Inner-scope credential cannot be reused");

    if verbose {
        println!();
        println!("   Production tip: use the `zeroize` crate for secrets in");
        println!("   structs; here we scrub `Vec<u8>` safely without `unsafe`.");
    }

    Ok(())
}
