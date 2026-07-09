//! CLI for the 04-functions checksum exercises.
//!
//! ```bash
//! cargo run -p exercise_functions
//! cargo run -p exercise_functions -- list
//! cargo run -p exercise_functions -- checksum --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use functions_exercises::{get_exercise_list, run_all, run_exercise};

/// Function exercises via network checksum helpers
#[derive(Parser, Debug)]
#[command(
    name = "exercise_functions",
    about = "Rust function exercises — checksums, sealing, and verification",
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

    /// Checksum hex formatting and word folding
    Checksum,

    /// Seal and verify length-prefixed packets
    Seal,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 04-functions exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Checksum) => run_exercise("checksum", verbose)?,
        Some(Commands::Seal) => run_exercise("seal", verbose)?,
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
    println!("  cargo run -p exercise_functions -- list");
    println!("  cargo run -p exercise_functions -- checksum [--verbose]");
    println!("  cargo run -p exercise_functions -- seal [--verbose]");
    println!("  cargo run -p exercise_functions -- all [--verbose]");
    println!("  cargo run -p exercise_functions              # same as 'all'");
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
