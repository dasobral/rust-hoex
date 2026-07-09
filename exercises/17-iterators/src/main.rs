//! CLI for the 17-iterators firewall log exercises.
//!
//! ```bash
//! cargo run -p exercise_iterators
//! cargo run -p exercise_iterators -- list
//! cargo run -p exercise_iterators -- filters --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use iterators_exercises::{get_exercise_list, run_all, run_exercise};

/// Iterator exercises — firewall log processing
#[derive(Parser, Debug)]
#[command(
    name = "exercise_iterators",
    about = "Rust iterator adapter exercises via firewall log analysis",
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
    /// Exercise 1: filter + map + sum
    Adapters,
    /// Exercise 2: collect denied IPs, count allows
    Filters,
    /// Exercise 3: inspect side effects
    Pipeline,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 17-iterators exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Adapters) => run_exercise("adapters", verbose)?,
        Some(Commands::Filters) => run_exercise("filters", verbose)?,
        Some(Commands::Pipeline) => run_exercise("pipeline", verbose)?,
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
    println!("  cargo run -p exercise_iterators -- list");
    println!("  cargo run -p exercise_iterators -- adapters [--verbose]");
    println!("  cargo run -p exercise_iterators -- filters [--verbose]");
    println!("  cargo run -p exercise_iterators -- pipeline [--verbose]");
    println!("  cargo run -p exercise_iterators -- all [--verbose]");
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
