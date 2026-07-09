//! CLI for the 16-testing password policy exercises.

use anyhow::Result;
use clap::{Parser, Subcommand};
use testing_exercises::{get_exercise_list, run_all, run_exercise};

/// Testing exercises — password policy with unit and integration tests
#[derive(Parser, Debug)]
#[command(
    name = "exercise_testing",
    about = "Rust testing exercises via password policy rules",
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
    /// Exercise 1: policy check demo
    Policy,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 16-testing exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
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
    println!("Tip: cargo test -p exercise_testing -- --ignored");
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses() {
        Cli::command().debug_assert();
    }
}
