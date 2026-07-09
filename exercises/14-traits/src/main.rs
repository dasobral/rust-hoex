//! CLI for the 14-traits `ThreatScorer` exercises.
//!
//! ```bash
//! cargo run -p exercise_traits
//! cargo run -p exercise_traits -- list
//! cargo run -p exercise_traits -- malware --verbose
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use traits_exercises::{get_exercise_list, run_all, run_exercise};

/// Traits exercises — `ThreatScorer` for security events
#[derive(Parser, Debug)]
#[command(
    name = "exercise_traits",
    about = "Rust traits exercises via SOC threat scoring",
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
    Auth,
    Network,
    Malware,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 14-traits exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Auth) => run_exercise("auth", verbose)?,
        Some(Commands::Network) => run_exercise("network", verbose)?,
        Some(Commands::Malware) => run_exercise("malware", verbose)?,
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
    println!("  cargo run -p exercise_traits -- list");
    println!("  cargo run -p exercise_traits -- auth [--verbose]");
    println!("  cargo run -p exercise_traits -- network [--verbose]");
    println!("  cargo run -p exercise_traits -- malware [--verbose]");
    println!("  cargo run -p exercise_traits -- all [--verbose]");
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
