//! CLI for the 09-enums security modeling exercises.
//!
//! ```bash
//! cargo run -p exercise_enums
//! cargo run -p exercise_enums -- list
//! cargo run -p exercise_enums -- auth-status --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use enums_exercises::{get_exercise_list, run_all, run_exercise};

/// Enum exercises — auth, network events, HTTP status, tokens
#[derive(Parser, Debug)]
#[command(
    name = "exercise_enums",
    about = "Rust enum exercises via auth status and network incident modeling",
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
    /// List available exercises and concepts
    List,

    /// Run every exercise in sequence
    All,

    /// Authentication outcome enum variants
    AuthStatus,

    /// Network events, HTTP codes, and token fallbacks
    Network,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 09-enums exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::AuthStatus) => run_exercise("auth-status", verbose)?,
        Some(Commands::Network) => run_exercise("network", verbose)?,
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
    println!("  cargo run -p exercise_enums -- list");
    println!("  cargo run -p exercise_enums -- auth-status [--verbose]");
    println!("  cargo run -p exercise_enums -- network [--verbose]");
    println!("  cargo run -p exercise_enums -- all [--verbose]");
    println!("  cargo run -p exercise_enums              # same as 'all'");
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
    fn list_does_not_panic() {
        list_exercises();
    }
}
