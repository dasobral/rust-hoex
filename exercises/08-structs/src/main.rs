//! CLI for the 08-structs security modeling exercises.
//!
//! ```bash
//! cargo run -p exercise_structs
//! cargo run -p exercise_structs -- list
//! cargo run -p exercise_structs -- auth-flow --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use structs_exercises::{get_exercise_list, run_all, run_exercise};

/// Struct exercises — user accounts, credentials, and sessions
#[derive(Parser, Debug)]
#[command(
    name = "exercise_structs",
    about = "Rust struct exercises via user accounts and session tokens",
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

    /// Authenticate users and mint session tokens
    AuthFlow,

    /// Simulate credential-stuffing lockout policy
    Lockout,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 08-structs exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::AuthFlow) => run_exercise("auth-flow", verbose)?,
        Some(Commands::Lockout) => run_exercise("lockout", verbose)?,
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
    println!("  cargo run -p exercise_structs -- list");
    println!("  cargo run -p exercise_structs -- auth-flow [--verbose]");
    println!("  cargo run -p exercise_structs -- lockout [--verbose]");
    println!("  cargo run -p exercise_structs -- all [--verbose]");
    println!("  cargo run -p exercise_structs              # same as 'all'");
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
