//! Declarative macros (`macro_rules!`) for logging, test fixtures, and max.
//!
//! Macros: `say!`, `say_err!`, `testvec!`, `max_of!`.

/// Print a tagged line to stdout.
#[macro_export]
macro_rules! say {
    ($tag:expr) => {{
        println!("[{}]", $tag);
    }};
    ($tag:expr, $($value:expr),+ $(,)?) => {{
        print!("[{}] ", $tag);
        $(
            print!("{} ", $value);
        )*
        println!();
    }};
}

/// Print a tagged line to stderr.
#[macro_export]
macro_rules! say_err {
    ($tag:expr) => {{
        eprintln!("[{}]", $tag);
    }};
    ($tag:expr, $($value:expr),+ $(,)?) => {{
        eprint!("[{}] ", $tag);
        $(
            eprint!("{} ", $value);
        )*
        eprintln!();
    }};
}

/// Build a `Vec` from zero, one, or many expressions.
#[macro_export]
macro_rules! testvec {
    () => {{
        Vec::new()
    }};
    ($single:expr) => {{
        vec![$single]
    }};
    ($($item:expr),+ $(,)?) => {{
        vec![$($item),+]
    }};
}

/// Return the maximum of two or more expressions via nested `.max()`.
#[macro_export]
macro_rules! max_of {
    ($a:expr, $b:expr) => {{
        $a.max($b)
    }};
    ($a:expr, $($rest:expr),+ $(,)?) => {{
        $a.max(max_of!($($rest),+))
    }};
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

/// Run macro demos: logging, `testvec` table, and `max_of`.
pub fn run_macro_demo(verbose: bool) -> anyhow::Result<()> {
    say!("boot", "macros_exercises");
    if verbose {
        say!("verbose", "running table-driven password checks");
    }

    let cases = testvec![("abc", 0_u32), ("password", 2_u32), ("Password1", 4_u32),];

    for (input, expected) in &cases {
        let got = password_score(input);
        say!("score", input, got, "expected", expected);
        if got != *expected {
            say_err!("mismatch", input, got, expected);
        }
    }

    let empty: Vec<u32> = testvec![];
    let single = testvec![42_u32];
    let multi = testvec![1_u32, 5, 3, 9, 2];
    say!(
        "testvec",
        "empty",
        empty.len(),
        "single",
        single[0],
        "multi",
        multi.len()
    );

    let peak = max_of!(3_i32, 9, 4, 7);
    say!("max_of", peak);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::{max_of, say, say_err, testvec};

    #[test]
    fn testvec_empty() {
        let v: Vec<i32> = testvec![];
        assert!(v.is_empty());
    }

    #[test]
    fn testvec_single() {
        let v = testvec![7_u32];
        assert_eq!(v, vec![7]);
    }

    #[test]
    fn testvec_multi() {
        let v = testvec![1_u32, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn max_of_two_and_many() {
        assert_eq!(max_of!(1_i32, 2), 2);
        assert_eq!(max_of!(1_i32, 5, 3, 9, 2), 9);
    }

    #[test]
    fn say_macros_expand() {
        say!("tag");
        say!("tag", 1, 2);
        say_err!("warn");
        say_err!("warn", "detail");
    }

    #[test]
    fn password_score_table() {
        let cases = testvec![("short", 0_u32), ("Longer1A", 4_u32)];
        for (input, expected) in cases {
            assert_eq!(password_score(input), expected);
        }
    }
}
