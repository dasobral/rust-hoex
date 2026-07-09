//! Exercise — credential parsing with `Option` and transpose.

use anyhow::Result;

use crate::credentials::{
    mask_token, optional_credential, parse_optional_token, require_credential,
};

/// Run the credentials exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔑 Credentials — Option & Result patterns");
    println!();

    match parse_optional_token("") {
        None => println!("  no API token configured"),
        Some(t) => println!("  token: {t}"),
    }

    if let Some(token) = parse_optional_token("  sess-9f3a  ") {
        println!("  trimmed token: {token}");
        println!("  masked: {}", mask_token(&token));
    }

    println!();
    for sample in [None, Some("tiny"), Some("long-enough-secret")] {
        match optional_credential(sample) {
            Ok(None) => println!("  auth disabled"),
            Ok(Some(c)) => println!("  credential accepted (len={})", c.len()),
            Err(e) => println!("  rejected: {e}"),
        }
    }

    if verbose {
        println!();
        println!("  optional_credential uses Option::map + transpose:");
        println!("  None → Ok(None); Some(err) → Err; Some(ok) → Ok(Some(...)).");
    }

    if let Err(e) = require_credential("bad") {
        println!();
        println!("  require_credential error: {e}");
    }

    Ok(())
}
