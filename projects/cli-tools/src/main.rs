//! Capstone CLI: **seccheck** — password entropy and strength auditing.
//!
//! ```bash
//! cargo run -p project_cli_tools -- entropy 's3cret!'
//! cargo run -p project_cli_tools -- analyze 'Tr0ub4dor&3'
//! printf 'password\nGoodPass1!\n' | cargo run -p project_cli_tools -- batch
//! ```

use std::io::{self, BufRead, Write};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use project_cli_tools::{
    AnalysisReport, CharClass, EntropyEstimate, analyze_password, estimate_entropy,
};

/// Password entropy and strength checker (rust-hoex capstone).
#[derive(Parser, Debug)]
#[command(
    name = "seccheck",
    version,
    about = "Estimate password entropy and analyze strength",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print Shannon-style entropy in bits for a password
    Entropy {
        /// Password to measure (prefer quoting in the shell)
        password: String,
    },
    /// Full strength analysis: entropy, classes, findings, rating
    Analyze {
        /// Password to analyze
        password: String,
    },
    /// Read one password per line from stdin and analyze each
    Batch {
        /// Skip blank lines instead of reporting them
        #[arg(long)]
        skip_empty: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Entropy { password } => {
            require_nonempty(&password)?;
            print_entropy(&estimate_entropy(&password))?;
        }
        Commands::Analyze { password } => {
            require_nonempty(&password)?;
            print_analysis(&analyze_password(&password))?;
        }
        Commands::Batch { skip_empty } => {
            run_batch(skip_empty)?;
        }
    }
    Ok(())
}

fn require_nonempty(password: &str) -> Result<()> {
    if password.is_empty() {
        bail!("password must not be empty; pass a non-empty argument or use `batch`");
    }
    Ok(())
}

fn print_entropy(est: &EntropyEstimate) -> Result<()> {
    let mut out = io::stdout().lock();
    writeln!(out, "length:         {}", est.length)?;
    writeln!(out, "alphabet size:  {}", est.alphabet_size)?;
    writeln!(out, "classes:        {}", format_classes(&est.classes))?;
    writeln!(out, "entropy:        {:.1} bits", est.bits)?;
    Ok(())
}

fn print_analysis(report: &AnalysisReport) -> Result<()> {
    let mut out = io::stdout().lock();
    writeln!(out, "strength:       {}", report.strength.as_str())?;
    writeln!(out, "entropy:        {:.1} bits", report.entropy.bits)?;
    writeln!(out, "length:         {}", report.entropy.length)?;
    writeln!(
        out,
        "classes:        {}",
        format_classes(&report.entropy.classes)
    )?;
    writeln!(
        out,
        "class counts:   {}",
        format_counts(&report.class_counts)
    )?;
    if report.findings.is_empty() {
        writeln!(out, "findings:       (none)")?;
    } else {
        writeln!(out, "findings:")?;
        for finding in &report.findings {
            writeln!(out, "  - {finding}")?;
        }
    }
    Ok(())
}

fn format_classes(classes: &std::collections::HashSet<CharClass>) -> String {
    let mut labels: Vec<&str> = classes
        .iter()
        .map(|c| match c {
            CharClass::Lower => "lower",
            CharClass::Upper => "upper",
            CharClass::Digit => "digit",
            CharClass::Symbol => "symbol",
        })
        .collect();
    labels.sort_unstable();
    if labels.is_empty() {
        "(none)".to_owned()
    } else {
        labels.join(", ")
    }
}

fn format_counts(counts: &std::collections::HashMap<&'static str, usize>) -> String {
    let mut pairs: Vec<(&str, usize)> = counts.iter().map(|(&k, &v)| (k, v)).collect();
    pairs.sort_by_key(|(k, _)| *k);
    if pairs.is_empty() {
        "(none)".to_owned()
    } else {
        pairs
            .into_iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn run_batch(skip_empty: bool) -> Result<()> {
    let stdin = io::stdin();
    let mut out = io::stdout().lock();
    let mut processed = 0_usize;
    let mut weak_or_worse = 0_usize;

    for (idx, line) in stdin.lock().lines().enumerate() {
        let line_no = idx + 1;
        let password = line.with_context(|| format!("failed to read stdin line {line_no}"))?;

        if password.is_empty() {
            if skip_empty {
                continue;
            }
            writeln!(out, "[{line_no}] (empty) → very weak")?;
            weak_or_worse += 1;
            processed += 1;
            continue;
        }

        let report = analyze_password(&password);
        if report.strength <= project_cli_tools::Strength::Weak {
            weak_or_worse += 1;
        }

        let preview = redact_preview(&password);
        writeln!(
            out,
            "[{line_no}] {preview} → {} ({:.1} bits)",
            report.strength.as_str(),
            report.entropy.bits
        )?;
        processed += 1;
    }

    writeln!(
        out,
        "---\nprocessed: {processed}  weak-or-worse: {weak_or_worse}"
    )?;
    Ok(())
}

/// Show first/last char with middle redacted so batch output is safer to share.
fn redact_preview(password: &str) -> String {
    let chars: Vec<char> = password.chars().collect();
    match chars.len() {
        0 => String::new(),
        1 => "*".to_owned(),
        2 => format!("{}*", chars[0]),
        n => {
            let stars = "*".repeat(n.saturating_sub(2).min(8));
            format!("{}{stars}{}", chars[0], chars[n - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_short_and_long() {
        assert_eq!(redact_preview("a"), "*");
        assert_eq!(redact_preview("ab"), "a*");
        assert_eq!(redact_preview("abcdef"), "a****f");
    }

    #[test]
    fn require_nonempty_rejects_blank() {
        assert!(require_nonempty("").is_err());
        assert!(require_nonempty("x").is_ok());
    }
}
