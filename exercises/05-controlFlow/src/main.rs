//! CLI for the 05-controlFlow threat score exercises.
//!
//! ```bash
//! cargo run -p exercise_controlflow
//! cargo run -p exercise_controlflow -- list
//! cargo run -p exercise_controlflow -- scoring --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use controlflow_exercises::{get_exercise_list, run_all, run_exercise};

/// Control flow exercises via threat scores and log classification
#[derive(Parser, Debug)]
#[command(
    name = "exercise_controlflow",
    about = "Rust control flow exercises — scoring, severity, and batch classification",
    version
)]
struct Cli {
    /// Print detailed educational explanations
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List available exercises and concepts
    List,

    /// Run every exercise in sequence
    All,

    /// Keyword scoring and severity mapping
    Scoring,

    /// Batch log classification with loops
    Batch,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 05-controlFlow exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Scoring) => run_exercise("scoring", verbose)?,
        Some(Commands::Batch) => run_exercise("batch", verbose)?,
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
    println!("  cargo run -p exercise_controlflow -- list");
    println!("  cargo run -p exercise_controlflow -- scoring [--verbose]");
    println!("  cargo run -p exercise_controlflow -- batch [--verbose]");
    println!("  cargo run -p exercise_controlflow -- all [--verbose]");
    println!("  cargo run -p exercise_controlflow              # same as 'all'");
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parses() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_list_does_not_panic() {
        list_exercises();
    }
}
