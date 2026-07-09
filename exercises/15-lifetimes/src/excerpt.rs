//! Struct that holds a borrowed excerpt from a log line or alert.

/// A struct that **holds** a reference — the lifetime must appear on the type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportantExcerpt<'a> {
    /// Borrowed highlight text (e.g. last word of an alert).
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    /// Build an excerpt wrapping `text` directly.
    #[must_use]
    pub const fn new(text: &'a str) -> Self {
        Self { part: text }
    }

    /// Return the last whitespace-separated word of `text`.
    #[must_use]
    pub fn last_word(text: &'a str) -> Self {
        let trimmed = text.trim_end();
        let start = trimmed
            .char_indices()
            .rev()
            .find(|(_, ch)| ch.is_whitespace())
            .map_or(0, |(idx, ch)| idx + ch.len_utf8());
        Self {
            part: &trimmed[start..],
        }
    }

    /// Level label for display (owned `String` — no extra lifetime needed).
    #[must_use]
    pub fn level(&self) -> String {
        format!("excerpt[len={}]", self.part.len())
    }
}

/// Run the excerpt demo.
pub fn run(verbose: bool) -> crate::Result<()> {
    println!("ImportantExcerpt<'a> demo\n");

    let line = "ALERT user=alice action=login_fail host=web-01";
    let excerpt = ImportantExcerpt::last_word(line);
    println!("  line: {line}");
    println!("  last word: {}", excerpt.part);
    println!("  level: {}", excerpt.level());

    let direct = ImportantExcerpt::new("CRITICAL");
    if verbose {
        println!("  direct excerpt: {}", direct.part);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_word_extracts_final_token() {
        let text = "deny all traffic now";
        let excerpt = ImportantExcerpt::last_word(text);
        assert_eq!(excerpt.part, "now");
    }

    #[test]
    fn new_wraps_text() {
        let excerpt = ImportantExcerpt::new("HIGH");
        assert_eq!(excerpt.level(), "excerpt[len=4]");
    }

    #[test]
    fn demo_runs() {
        assert!(run(false).is_ok());
    }
}
