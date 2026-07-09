//! Owned summaries — return `String` when assembling new text.

/// Join alert parts into an owned summary.
///
/// Returning `String` avoids dangling references: the result owns its buffer
/// and is valid after the function returns. Contrast with returning `&str` to a
/// local `String` — that cannot compile because the local value is dropped.
#[must_use]
pub fn owned_summary(parts: &[&str]) -> String {
    parts.join(" / ")
}

/// Run the summary demo.
pub fn run(verbose: bool) -> crate::Result<()> {
    println!("owned_summary demo\n");

    let parts = ["CRITICAL", "disk", "full", "/var"];
    let summary = owned_summary(&parts);
    println!("  parts: {parts:?}");
    println!("  summary: {summary}");

    if verbose {
        println!("  owned String survives after the function returns");
        println!("  fn broken() -> &str {{ let s = String::from(\"x\"); &s }} // won't compile");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_parts_yields_empty_string() {
        assert_eq!(owned_summary(&[]), "");
    }

    #[test]
    fn joins_with_separator() {
        let parts = ["login_fail", "alice"];
        assert_eq!(owned_summary(&parts), "login_fail / alice");
    }

    #[test]
    fn demo_runs() {
        assert!(run(false).is_ok());
    }
}
