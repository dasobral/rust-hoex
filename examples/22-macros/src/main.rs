//! example: 22-macros
//!
//! Declarative macros: `say!`, `testvec!`, `maplit!`, `count_exprs!`.
//!
//! ```bash
//! cargo run
//! ```

use std::collections::HashMap;

use example_macros::{config_or, count_exprs, maplit, password_score, say, testvec};

fn main() {
    println!("=== 22-macros: macro_rules! helpers ===\n");

    say!("boot", "example_macros");

    let cases = testvec![("abc", 0_u32), ("password", 2_u32), ("Password1", 4_u32),];
    say!("cases", count_exprs!("a", "b", "c"));

    for (input, expected) in &cases {
        let got = password_score(input);
        say!("score", input, got, "expected", expected);
        if got != *expected {
            eprintln!("mismatch for {input}: got {got}, expected {expected}");
        }
    }

    let cfg: HashMap<&str, u32> = maplit! {
        "ttl" => 60,
        "retries" => 3,
        "workers" => 4,
    };
    say!("ttl", config_or(&cfg, "ttl", 0));
    say!("missing", config_or(&cfg, "nope", 99));

    println!("\n(Declarative macros expand at compile time; prefer functions when enough.)");
}
