//! CLI for the 02-variables physics exercises
//!
//! Run with:
//! ```bash
//! cargo run -p exercise_variables
//! cargo run -p exercise_variables -- list
//! cargo run -p exercise_variables -- quantum --verbose
//! ```
//!
//! Demonstrates `clap` derive macros for argument parsing and
//! wiring a binary crate to a library API.

use anyhow::Result;
use clap::{Parser, Subcommand};
use variables_exercises::{get_exercise_list, run_all, run_exercise};

/// Advanced variable concepts through physics problems
#[derive(Parser, Debug)]
#[command(
    name = "exercise_variables",
    about = "Rust variable exercises via quantum, EM, and temperature physics",
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
    /// List available exercises and the concepts they teach
    List,

    /// Run every exercise in sequence
    All,

    /// Run the quantum energy-levels exercise (signed integers)
    Quantum,

    /// Run the electromagnetic fields exercise (vectors & overflow)
    Electromagnetic,

    /// Run the temperature conversions exercise (type conversions)
    Temperature,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 02-variables exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => {
            list_exercises();
        }
        Some(Commands::Quantum) => {
            run_exercise("quantum", verbose)?;
        }
        Some(Commands::Electromagnetic) => {
            run_exercise("electromagnetic", verbose)?;
        }
        Some(Commands::Temperature) => {
            run_exercise("temperature", verbose)?;
        }
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
    println!("  cargo run -p exercise_variables -- list");
    println!("  cargo run -p exercise_variables -- quantum [--verbose]");
    println!("  cargo run -p exercise_variables -- electromagnetic [--verbose]");
    println!("  cargo run -p exercise_variables -- temperature [--verbose]");
    println!("  cargo run -p exercise_variables -- all [--verbose]");
    println!("  cargo run -p exercise_variables              # same as 'all'");
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parses() {
        // Ensures the derive macros produce a valid clap Command
        Cli::command().debug_assert();
    }

    #[test]
    fn test_list_does_not_panic() {
        list_exercises();
    }
}
