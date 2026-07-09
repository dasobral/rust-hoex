//! Hello World exercises — cybersecurity-flavored greetings and banner strings.
//!
//! Introduces `format!`, string slices, and basic text helpers used in
//! security tooling output and operator consoles.

pub type Result<T> = anyhow::Result<T>;

/// Build a friendly greeting for `name`.
///
/// # Examples
///
/// ```
/// use helloworld_exercises::greet;
/// assert_eq!(greet("Analyst"), "Hello, Analyst!");
/// ```
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Build an ASCII security operations banner for the named tool.
#[must_use]
pub fn security_banner(tool: &str) -> String {
    format!(
        "╔══════════════════════════════════╗\n\
         ║  Security Operations Console   ║\n\
         ╠══════════════════════════════════╣\n\
         ║  Tool: {tool:<28} ║\n\
         ╚══════════════════════════════════╝"
    )
}

/// Join string slices with newline separators.
#[must_use]
pub fn join_lines(parts: &[&str]) -> String {
    parts.join("\n")
}

/// Replace every character in `secret` with `*` for safe display.
#[must_use]
pub fn mask_secret(secret: &str) -> String {
    "*".repeat(secret.chars().count())
}

/// Exercise metadata for the CLI.
#[derive(Debug, Clone)]
pub struct ExerciseInfo {
    /// Static exercise identifier.
    pub name: &'static str,
    /// Short description shown in `list`.
    pub description: &'static str,
    /// Concepts covered by the exercise.
    pub concepts: Vec<&'static str>,
}

/// Return metadata for every exercise in this crate.
#[must_use]
pub fn get_exercise_list() -> Vec<ExerciseInfo> {
    vec![
        ExerciseInfo {
            name: "greet",
            description: "Operator greetings, multi-line status, and secret masking",
            concepts: vec![
                "`format!` and string interpolation",
                "Slice joining with newlines",
                "Redacting credentials for logs",
            ],
        },
        ExerciseInfo {
            name: "banner",
            description: "Security tool banners with fixed-width formatting",
            concepts: vec![
                "Multi-line raw strings",
                "Left-aligned padding with width specifiers",
                "Console branding for CLI tools",
            ],
        },
    ]
}

/// Run the greet exercise — greetings, status lines, and masked secrets.
pub fn run_greet_exercise(name: &str, verbose: bool) {
    println!("{}", greet(name));

    let status = join_lines(&[
        "Session: authenticated",
        "Role: security-analyst",
        "Clearance: operator",
    ]);
    println!("\n{status}");

    let api_key = "sk-live-9f3a2b1c";
    let masked = mask_secret(api_key);
    println!("\nAPI key (redacted): {masked}");

    if verbose {
        println!("\n--- Concepts ---");
        println!("• `greet` uses `format!` to interpolate a name into a template.");
        println!("• `join_lines` builds multi-line status blocks from string slices.");
        println!("• `mask_secret` keeps length while hiding sensitive values in logs.");
    }
}

/// Run the banner exercise — branded tool headers.
pub fn run_banner_exercise(tool: &str, verbose: bool) {
    println!("{}", security_banner(tool));

    if verbose {
        println!("\n--- Concepts ---");
        println!("• `security_banner` uses a multi-line `format!` template.");
        println!("• Left-align padding keeps tool names readable inside the banner frame.");
        println!("• Banners give operators immediate context about which tool is running.");
    }
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, operator: &str, tool: &str, verbose: bool) -> Result<()> {
    match name {
        "greet" => {
            run_greet_exercise(operator, verbose);
            Ok(())
        }
        "banner" => {
            run_banner_exercise(tool, verbose);
            Ok(())
        }
        _ => anyhow::bail!("Unknown exercise: {name}. Available: greet, banner"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(operator: &str, tool: &str, verbose: bool) -> Result<()> {
    let exercises = ["greet", "banner"];

    for (i, exercise_name) in exercises.iter().enumerate() {
        println!(
            "🛡️  Exercise {} of {}: {}",
            i + 1,
            exercises.len(),
            exercise_name
        );
        println!("{}", "=".repeat(50));

        run_exercise(exercise_name, operator, tool, verbose)?;

        if i < exercises.len() - 1 {
            println!("\n{}\n", "─".repeat(50));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_includes_name() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn greet_handles_empty_name() {
        assert_eq!(greet(""), "Hello, !");
    }

    #[test]
    fn security_banner_contains_tool() {
        let banner = security_banner("nmap");
        assert!(banner.contains("nmap"));
        assert!(banner.contains("Security Operations Console"));
    }

    #[test]
    fn join_lines_joins_with_newlines() {
        assert_eq!(join_lines(&["a", "b", "c"]), "a\nb\nc");
    }

    #[test]
    fn join_lines_empty_slice() {
        assert_eq!(join_lines(&[]), "");
    }

    #[test]
    fn mask_secret_replaces_all_chars() {
        assert_eq!(mask_secret("abc123"), "******");
    }

    #[test]
    fn mask_secret_empty_string() {
        assert_eq!(mask_secret(""), "");
    }

    #[test]
    fn mask_secret_preserves_unicode_length() {
        assert_eq!(mask_secret("🔑🔑"), "**");
    }

    #[test]
    fn exercise_list_has_two_entries() {
        assert_eq!(get_exercise_list().len(), 2);
    }
}
