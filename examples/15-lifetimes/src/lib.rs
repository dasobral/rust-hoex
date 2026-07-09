//! Lifetime annotations: tying returned references to their inputs.
//!
//! # Why lifetimes exist
//!
//! A reference (`&T` / `&mut T`) must not outlive the data it points at.
//! When a function *returns* a reference that came from one of its parameters,
//! the compiler needs to know **which** input the output borrows from.
//! Lifetime parameters (`'a`, `'b`, …) name those relationships so the
//! borrow checker can reject dangling references at compile time.
//!
//! # Lifetime elision (the rules you usually do not write)
//!
//! Rust applies three elision rules so many signatures stay quiet:
//!
//! 1. Each elided input reference gets its own lifetime parameter.
//! 2. If there is exactly one input lifetime, it is assigned to all elided
//!    output lifetimes.
//! 3. If there are multiple input lifetimes but one is `&self` / `&mut self`,
//!    that lifetime is assigned to all elided outputs.
//!
//! When those rules are not enough — two inputs, one output reference — you
//! must write lifetimes explicitly. See [`longest`] and [`find_in_haystack`].
//! [`first_word`] shows the elided form (rule 2).

/// Return the longer of two string slices.
///
/// Both inputs and the output share `'a`: the result may borrow from either
/// argument, so both must live at least as long as the returned `&str`.
///
/// Without `'a`, this would not compile — elision cannot pick which input
/// the output comes from when there are two candidates.
#[must_use]
pub const fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

/// Find the first occurrence of `needle` inside `haystack`.
///
/// The returned slice (if any) is a borrow of `haystack`, so they share `'a`.
/// `needle` only needs to live for the call — it gets a separate (elided)
/// lifetime and is **not** tied to the output. That asymmetry is why `'a`
/// must appear on `haystack` and the return type, but not on `needle`.
#[must_use]
pub fn find_in_haystack<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    haystack
        .find(needle)
        .map(|start| &haystack[start..start + needle.len()])
}

/// First word of a line (whitespace-separated), or the whole line if none.
///
/// One input reference → elision rule 2 applies, so we omit `'a` here.
/// Equivalent explicit form: `fn first_word<'a>(s: &'a str) -> &'a str`.
#[must_use]
pub fn first_word(s: &str) -> &str {
    s.find(char::is_whitespace).map_or(s, |i| &s[..i])
}

/// A struct that **holds** a reference — the lifetime must appear on the type.
///
/// `ImportantExcerpt<'a>` cannot outlive the string it points at. Storing a
/// reference inside a struct always requires an explicit lifetime parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportantExcerpt<'a> {
    /// Borrowed highlight text (e.g. a log line or announcement).
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    /// Build an excerpt from the first word of `text`.
    #[must_use]
    pub fn from_first_word(text: &'a str) -> Self {
        Self {
            part: first_word(text),
        }
    }

    /// Announce the excerpt; return type uses the struct's lifetime via `self`.
    ///
    /// Elision rule 3: `&self` supplies `'a` for the returned `&str`.
    #[must_use]
    pub const fn announce_and_return_part(&self, announcement: &str) -> &str {
        // `announcement` is unused on purpose: in a real program you might log
        // it. We still return a borrow of `self.part`, not of `announcement`.
        let _ = announcement;
        self.part
    }

    /// Level of the excerpt for display (owned `String` — no lifetime needed).
    #[must_use]
    pub fn level(&self) -> String {
        format!("excerpt[{}]", self.part.len())
    }
}

/// Pick the longer excerpt; both must share a lifetime with the result.
#[must_use]
pub const fn longer_excerpt<'a>(
    a: ImportantExcerpt<'a>,
    b: ImportantExcerpt<'a>,
) -> ImportantExcerpt<'a> {
    if a.part.len() >= b.part.len() { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_picks_longer_slice() {
        assert_eq!(longest("short", "much longer"), "much longer");
        assert_eq!(longest("equal1", "equal2"), "equal1");
    }

    #[test]
    fn find_in_haystack_returns_borrow_of_haystack() {
        let hay = "user=alice action=login";
        let found = find_in_haystack(hay, "alice");
        assert_eq!(found, Some("alice"));
        assert!(find_in_haystack(hay, "bob").is_none());
    }

    #[test]
    fn first_word_splits_on_whitespace() {
        assert_eq!(first_word("deny all traffic"), "deny");
        assert_eq!(first_word("single"), "single");
    }

    #[test]
    fn important_excerpt_borrows_source() {
        let text = "CRITICAL: disk full on /var";
        let excerpt = ImportantExcerpt::from_first_word(text);
        assert_eq!(excerpt.part, "CRITICAL:");
        assert_eq!(excerpt.announce_and_return_part("alert"), "CRITICAL:");
        assert!(excerpt.level().contains('9'));
    }

    #[test]
    fn longer_excerpt_compares_parts() {
        let a = ImportantExcerpt { part: "hi" };
        let b = ImportantExcerpt { part: "hello" };
        assert_eq!(longer_excerpt(a, b).part, "hello");
    }
}
