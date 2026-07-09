//! CLI for the 19-smart-pointers policy exercises.
//!
//! ```bash
//! cargo run -p exercise_smartpointers
//! cargo run -p exercise_smartpointers -- list
//! cargo run -p exercise_smartpointers -- sharing --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use smartpointers_exercises::{get_exercise_list, run_all, run_exercise};

/// Smart pointer exercises — recursive security policy rules
#[derive(Parser, Debug)]
#[command(
    name = "exercise_smartpointers",
    about = "Rust smart pointer exercises via SOC policy evaluation",
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
    /// Exercise 1: Box-backed rule trees
    Rules,
    /// Exercise 2: `Rc` + `RefCell` shared config
    Sharing,
    /// Exercise 3: clone engines sharing config
    Clones,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 19-smart-pointers exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Rules) => run_exercise("rules", verbose)?,
        Some(Commands::Sharing) => run_exercise("sharing", verbose)?,
        Some(Commands::Clones) => run_exercise("clones", verbose)?,
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
    println!("  cargo run -p exercise_smartpointers -- list");
    println!("  cargo run -p exercise_smartpointers -- rules [--verbose]");
    println!("  cargo run -p exercise_smartpointers -- sharing [--verbose]");
    println!("  cargo run -p exercise_smartpointers -- clones [--verbose]");
    println!("  cargo run -p exercise_smartpointers -- all [--verbose]");
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
