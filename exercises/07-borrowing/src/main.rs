//! CLI for the 07-borrowing password audit exercises.
//!
//! ```bash
//! cargo run -p exercise_borrowing
//! cargo run -p exercise_borrowing -- list
//! cargo run -p exercise_borrowing -- inspect --verbose
//! ```

use anyhow::Result;
use borrowing_exercises::{get_exercise_list, run_all, run_exercise};
use clap::{Parser, Subcommand};

/// Borrowing exercises via password inspection and policy scoring
#[derive(Parser, Debug)]
#[command(
    name = "exercise_borrowing",
    about = "Rust borrowing exercises — &T, &mut T, and slices",
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

    /// Inspect — immutable borrows of password data
    Inspect,

    /// Policy — mutable borrows and slice aggregation
    Policy,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 07-borrowing exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Inspect) => run_exercise("inspect", verbose)?,
        Some(Commands::Policy) => run_exercise("policy", verbose)?,
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
    println!("  cargo run -p exercise_borrowing -- list");
    println!("  cargo run -p exercise_borrowing -- inspect [--verbose]");
    println!("  cargo run -p exercise_borrowing -- policy [--verbose]");
    println!("  cargo run -p exercise_borrowing -- all [--verbose]");
    println!("  cargo run -p exercise_borrowing              # same as 'all'");
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
