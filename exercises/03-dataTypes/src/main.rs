//! CLI for the 03-dataTypes packet header exercises.
//!
//! ```bash
//! cargo run -p exercise_datatypes
//! cargo run -p exercise_datatypes -- list
//! cargo run -p exercise_datatypes -- ports --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use datatypes_exercises::{get_exercise_list, run_all, run_exercise};

/// Data type exercises via packet header scalars and compounds
#[derive(Parser, Debug)]
#[command(
    name = "exercise_datatypes",
    about = "Rust data type exercises — ports, TOS masks, and IPv4 headers",
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

    /// TCP port scalars and endpoint structs
    Ports,

    /// TOS bit masks, protocol labels, and header arrays
    TosProtocol,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 03-dataTypes exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Ports) => run_exercise("ports", verbose)?,
        Some(Commands::TosProtocol) => run_exercise("tos_protocol", verbose)?,
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
    println!("  cargo run -p exercise_datatypes -- list");
    println!("  cargo run -p exercise_datatypes -- ports [--verbose]");
    println!("  cargo run -p exercise_datatypes -- tos-protocol [--verbose]");
    println!("  cargo run -p exercise_datatypes -- all [--verbose]");
    println!("  cargo run -p exercise_datatypes              # same as 'all'");
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
