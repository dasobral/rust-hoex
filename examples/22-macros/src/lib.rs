//! Declarative macros (`macro_rules!`) — no proc-macro crates.
//!
//! # What you will see
//!
//! - Designators: `$ident`, `$expr`, `$ty`, …
//! - Repetition: `$( ... )*`, `$( ... ),+`, separators
//! - Multiple matcher arms (overload by pattern)
//! - A brief note on **hygiene**: macro-introduced names do not collide with
//!   caller locals (unless you intentionally use `$crate` / pass idents in)
//!
//! Theme: tiny helpers — `say!` logging, `testvec!` fixtures, `maplit!`
//! `HashMap` construction — the kind of macros you write before reaching for
//! procedural macros.

use std::collections::HashMap;

/// Print a tagged log line. Accepts zero or more expressions after the tag.
///
/// ```ignore
/// say!("boot");
/// say!("count", 1 + 2, status);
/// ```
#[macro_export]
macro_rules! say {
    // Arm 1: tag only
    ($tag:expr) => {{
        println!("[{}] ", $tag);
    }};
    // Arm 2: tag + one or more values (repetition with `,`)
    ($tag:expr, $($value:expr),+ $(,)?) => {{
        print!("[{}] ", $tag);
        $(
            print!("{} ", $value);
        )*
        println!();
    }};
}

/// Build a `Vec` of test pairs `(input, expected)` for table-driven tests.
///
/// ```ignore
/// let cases = testvec![
///     ("a", 1),
///     ("b", 2),
/// ];
/// ```
#[macro_export]
macro_rules! testvec {
    ( $( ($input:expr, $expected:expr) ),* $(,)? ) => {{
        vec![
            $(
                ($input, $expected),
            )*
        ]
    }};
}

/// `hashmap!`-style literal for `HashMap`.
///
/// Keys and values are expressions; the macro expands to inserts.
///
/// ```ignore
/// let m = maplit! {
///     "ttl" => 30,
///     "retries" => 3,
/// };
/// ```
#[macro_export]
macro_rules! maplit {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

/// Count how many expressions you passed — demos `$()*` length via a counter.
///
/// Hygiene note: the temporary `count` inside the expansion is distinct from
/// any `count` in the caller — you cannot accidentally assign to the caller's
/// `count` from inside this macro.
#[macro_export]
macro_rules! count_exprs {
    ( $( $e:expr ),* $(,)? ) => {
        // No `mut` counter — avoids `unused_mut` when the repetition is empty.
        0_usize $( + { let _ = &$e; 1_usize } )*
    };
}

/// Score a password-ish string (toy policy used by the demo binary).
#[must_use]
pub fn password_score(s: &str) -> u32 {
    let mut score = 0_u32;
    if s.len() >= 8 {
        score = score.saturating_add(2);
    }
    if s.chars().any(|c| c.is_ascii_uppercase()) {
        score = score.saturating_add(1);
    }
    if s.chars().any(|c| c.is_ascii_digit()) {
        score = score.saturating_add(1);
    }
    score
}

/// Look up a config key from a string map, returning a default.
#[must_use]
pub fn config_or<S: ::std::hash::BuildHasher>(
    map: &HashMap<&str, u32, S>,
    key: &str,
    default: u32,
) -> u32 {
    map.get(key).copied().unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // `#[macro_export]` places macros at the crate root; import for local use.
    #[allow(unused_imports)] // rustc may not count `macro!` invocations as uses
    use crate::{count_exprs, maplit, say, testvec};

    #[test]
    fn testvec_builds_pairs() {
        let cases = testvec![("short", 0_u32), ("Longer1A", 4_u32),];
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].0, "short");
        for (input, expected) in cases {
            assert_eq!(password_score(input), expected);
        }
    }

    #[test]
    fn maplit_inserts_entries() {
        let m: HashMap<&str, u32> = maplit! {
            "ttl" => 30,
            "retries" => 3,
        };
        assert_eq!(m.get("ttl"), Some(&30));
        assert_eq!(config_or(&m, "missing", 9), 9);
    }

    #[test]
    fn count_exprs_counts() {
        assert_eq!(count_exprs!(), 0);
        assert_eq!(count_exprs!(1, 2, 3), 3);
    }

    #[test]
    fn say_compiles_both_arms() {
        // Just ensure expansion type-checks; output goes to stdout.
        say!("test-tag");
        say!("test-tag", 1, 2);
    }
}
