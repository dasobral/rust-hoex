//! CLI for the 06-ownership credential exercises.
//!
//! ```bash
//! cargo run -p exercise_ownership
//! cargo run -p exercise_ownership -- list
//! cargo run -p exercise_ownership -- move-vs-copy --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use ownership_exercises::{get_exercise_list, run_all, run_exercise};

/// Ownership exercises via secure credential handling
#[derive(Parser, Debug)]
#[command(
    name = "exercise_ownership",
    about = "Rust ownership exercises — moves, Clone, Copy, and zeroize",
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

    /// Move vs Copy — credential handoff and threat scores
    MoveVsCopy,

    /// Zeroize — scrub secrets before drop
    Zeroize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 06-ownership exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::MoveVsCopy) => run_exercise("move_vs_copy", verbose)?,
        Some(Commands::Zeroize) => run_exercise("zeroize", verbose)?,
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
    println!("  cargo run -p exercise_ownership -- list");
    println!("  cargo run -p exercise_ownership -- move-vs-copy [--verbose]");
    println!("  cargo run -p exercise_ownership -- zeroize [--verbose]");
    println!("  cargo run -p exercise_ownership -- all [--verbose]");
    println!("  cargo run -p exercise_ownership              # same as 'all'");
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
