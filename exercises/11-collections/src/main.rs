//! CLI for the 11-collections intrusion log exercises.
//!
//! ```bash
//! cargo run -p exercise_collections
//! cargo run -p exercise_collections -- list
//! cargo run -p exercise_collections -- analysis --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use collections_exercises::{get_exercise_list, run_all, run_exercise};

/// Collections exercises — intrusion log aggregation
#[derive(Parser, Debug)]
#[command(
    name = "exercise_collections",
    about = "Rust collections exercises via SOC intrusion log aggregation",
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
    /// Exercise 1: Vec-backed log construction
    Aggregator,
    /// Exercise 2: HashMap/HashSet aggregation
    Analysis,
    /// Exercise 3: filter and rank suspects
    Investigation,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 11-collections exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Aggregator) => run_exercise("aggregator", verbose)?,
        Some(Commands::Analysis) => run_exercise("analysis", verbose)?,
        Some(Commands::Investigation) => run_exercise("investigation", verbose)?,
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
    println!("  cargo run -p exercise_collections -- list");
    println!("  cargo run -p exercise_collections -- aggregator [--verbose]");
    println!("  cargo run -p exercise_collections -- analysis [--verbose]");
    println!("  cargo run -p exercise_collections -- investigation [--verbose]");
    println!("  cargo run -p exercise_collections -- all [--verbose]");
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
