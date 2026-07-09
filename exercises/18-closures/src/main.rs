//! CLI for the 18-closures threat event exercises.
//!
//! ```bash
//! cargo run -p exercise_closures
//! cargo run -p exercise_closures -- list
//! cargo run -p exercise_closures -- watchlist --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use closures_exercises::{get_exercise_list, run_all, run_exercise};

/// Closure exercises — threat event triage
#[derive(Parser, Debug)]
#[command(
    name = "exercise_closures",
    about = "Rust closure exercises via SOC threat event triage",
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
    /// Exercise 1: `sort_by_key` on source
    Sorting,
    /// Exercise 2: `partition` critical events
    Partition,
    /// Exercise 3: watchlist closure capture
    Watchlist,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 18-closures exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Sorting) => run_exercise("sorting", verbose)?,
        Some(Commands::Partition) => run_exercise("partition", verbose)?,
        Some(Commands::Watchlist) => run_exercise("watchlist", verbose)?,
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
    println!("  cargo run -p exercise_closures -- list");
    println!("  cargo run -p exercise_closures -- sorting [--verbose]");
    println!("  cargo run -p exercise_closures -- partition [--verbose]");
    println!("  cargo run -p exercise_closures -- watchlist [--verbose]");
    println!("  cargo run -p exercise_closures -- all [--verbose]");
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
