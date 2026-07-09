//! CLI for the 12-error-handling config validation exercises.
//!
//! ```bash
//! cargo run -p exercise_errorhandling
//! cargo run -p exercise_errorhandling -- list
//! cargo run -p exercise_errorhandling -- bootstrap --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use errorhandling_exercises::{get_exercise_list, run_all, run_exercise};

/// Error handling exercises — config and credential validation
#[derive(Parser, Debug)]
#[command(
    name = "exercise_errorhandling",
    about = "Rust error handling exercises via config/credential validation",
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
    List,
    All,
    Paths,
    Credentials,
    Bootstrap,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 12-error-handling exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Paths) => run_exercise("paths", verbose)?,
        Some(Commands::Credentials) => run_exercise("credentials", verbose)?,
        Some(Commands::Bootstrap) => run_exercise("bootstrap", verbose)?,
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
    println!("  cargo run -p exercise_errorhandling -- list");
    println!("  cargo run -p exercise_errorhandling -- paths [--verbose]");
    println!("  cargo run -p exercise_errorhandling -- credentials [--verbose]");
    println!("  cargo run -p exercise_errorhandling -- bootstrap [--verbose]");
    println!("  cargo run -p exercise_errorhandling -- all [--verbose]");
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
