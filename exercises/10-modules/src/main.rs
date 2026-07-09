//! CLI for the 10-modules password analysis exercises.
//!
//! ```bash
//! cargo run -p exercise_modules
//! cargo run -p exercise_modules -- list
//! cargo run -p exercise_modules -- score --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use modules_exercises::{get_exercise_list, run_all, run_exercise};

/// Modules exercises — multi-file password analysis
#[derive(Parser, Debug)]
#[command(
    name = "exercise_modules",
    about = "Rust modules exercises via password strength analysis",
    version
)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List exercises and concepts
    List,
    /// Run all exercises
    All,
    /// Exercise 1: score sample passwords
    Score,
    /// Exercise 2: validate against denylist
    Validate,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 10-modules exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Score) => run_exercise("score", verbose)?,
        Some(Commands::Validate) => run_exercise("validate", verbose)?,
    }

    Ok(())
}

fn list_exercises() {
    println!("Available exercises:\n");
    for info in get_exercise_list() {
        println!("  • {} — {}", info.name, info.description);
        println!("    Concepts:");
        for concept in &info.concepts {
            println!("      - {concept}");
        }
        println!();
    }
    println!("Usage:");
    println!("  cargo run -p exercise_modules -- list");
    println!("  cargo run -p exercise_modules -- score [--verbose]");
    println!("  cargo run -p exercise_modules -- validate [--verbose]");
    println!("  cargo run -p exercise_modules -- all [--verbose]");
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses() {
        Cli::command().debug_assert();
    }

    #[test]
    fn list_runs() {
        list_exercises();
    }
}
