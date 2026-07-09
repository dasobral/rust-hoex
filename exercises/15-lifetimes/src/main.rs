//! CLI for the 15-lifetimes log excerpt exercises.

use anyhow::Result;
use clap::{Parser, Subcommand};
use lifetimes_exercises::{get_exercise_list, run_all, run_exercise};

/// Lifetimes exercises — borrowed excerpts and owned summaries
#[derive(Parser, Debug)]
#[command(
    name = "exercise_lifetimes",
    about = "Rust lifetimes exercises via security log excerpts",
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
    /// Exercise 1: shortest<'a>
    Shortest,
    /// Exercise 2: `ImportantExcerpt`<'a>
    Excerpt,
    /// Exercise 3: `owned_summary`
    Summary,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 15-lifetimes exercises\n");
            run_all(verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Shortest) => run_exercise("shortest", verbose)?,
        Some(Commands::Excerpt) => run_exercise("excerpt", verbose)?,
        Some(Commands::Summary) => run_exercise("summary", verbose)?,
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
