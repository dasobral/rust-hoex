//! CLI for the 13-generics security data exercises.

use anyhow::Result;
use clap::{Parser, Subcommand};
use generics_exercises::{get_exercise_list, run_all, run_exercise};

/// Generics exercises — `SecureContainer`, `Pair`, `find_min`
#[derive(Parser, Debug)]
#[command(
    name = "exercise_generics",
    about = "Rust generics exercises via security-flavored containers and pairs",
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
    /// Exercise 1: `SecureContainer<T>`
    Container,
    /// Exercise 2: `Pair`<A, B>
    Pair,
    /// Exercise 3: `find_min`
    Search,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 13-generics exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Container) => run_exercise("container", verbose)?,
        Some(Commands::Pair) => run_exercise("pair", verbose)?,
        Some(Commands::Search) => run_exercise("search", verbose)?,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses() {
        Cli::command().debug_assert();
    }
}
