//! example: 12-error-handling
//!
//! Config path validation and credentials with `Option`, `Result`, and `?`.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key ideas:
//! - `Option` = value may be absent
//! - `Result` = operation may fail
//! - `?` propagates `Err` (or `None` in Option contexts)
//! - custom errors with `thiserror`
//! - `map` / `and_then` for chaining

use example_errorhandling::{
    ConfigError, load_settings, optional_credential, parse_optional_token, secure_bootstrap,
    validate_config_path,
};

fn main() {
    println!("=== 12-error-handling: config & credential validation ===\n");

    demo_option();
    demo_result_paths();
    demo_question_mark();
    demo_optional_credential();
}

fn demo_option() {
    println!("-- Option (Some / None) --");
    match parse_optional_token("") {
        None => println!("  no API token configured"),
        Some(t) => println!("  unexpected token: {t}"),
    }
    if let Some(token) = parse_optional_token("  sess-9f3a  ") {
        println!("  token present: {token}");
    }
    println!();
}

fn demo_result_paths() {
    println!("-- Result (Ok / Err) --");
    let samples = [
        "/etc/app/config.toml",
        "",
        "relative.yaml",
        "/etc/../passwd.toml",
        "/etc/app/config.json",
    ];

    for raw in samples {
        match validate_config_path(raw) {
            Ok(path) => println!("  OK  {path}"),
            Err(e) => println!("  ERR {e}"),
        }
    }
    println!();
}

fn demo_question_mark() {
    println!("-- ? operator via load_settings / secure_bootstrap --");

    match load_settings("/opt/sensor/config.yml", "hunter2-secret") {
        Ok((path, token)) => {
            println!("  loaded {path}");
            match token {
                Some(masked) => println!("  token (masked): {masked}"),
                None => println!("  token: (none)"),
            }
        }
        Err(e) => println!("  load failed: {e}"),
    }

    if let Err(e) = secure_bootstrap("/bad/../x.toml", "short") {
        // Display uses thiserror's #[error("...")] messages.
        print_config_error(&e);
    }
    println!();
}

fn demo_optional_credential() {
    println!("-- optional_credential (Option → Result) --");
    for sample in [None, Some("tiny"), Some("long-enough-secret")] {
        match optional_credential(sample) {
            Ok(None) => println!("  auth disabled"),
            Ok(Some(c)) => println!("  credential accepted (len={})", c.len()),
            Err(e) => println!("  credential rejected: {e}"),
        }
    }
}

fn print_config_error(err: &ConfigError) {
    println!("  bootstrap error: {err}");
}
