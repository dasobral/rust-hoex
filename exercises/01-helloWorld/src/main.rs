//! CLI for the 01-helloWorld security greeting exercises.
//!
//! ```bash
//! cargo run -p exercise_helloworld
//! cargo run -p exercise_helloworld -- list
//! cargo run -p exercise_helloworld -- greet --name Analyst
//! cargo run -p exercise_helloworld -- banner --tool nmap
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use helloworld_exercises::{get_exercise_list, run_all, run_exercise};

/// Hello World exercises — greetings and security tool banners
#[derive(Parser, Debug)]
#[command(
    name = "exercise_helloworld",
    about = "Rust hello-world exercises — operator greetings and security banners",
    version
)]
struct Cli {
    /// Print detailed educational explanations
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Operator name used by the greet exercise
    #[arg(short, long, global = true, default_value = "Operator")]
    name: String,

    /// Security tool name used by the banner exercise
    #[arg(short, long, global = true, default_value = "SecCheck")]
    tool: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List available exercises and concepts
    List,

    /// Run every exercise in sequence
    All,

    /// Operator greetings, status lines, and secret masking
    Greet,

    /// Security tool banner with fixed-width formatting
    Banner,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;
    let name = cli.name.as_str();
    let tool = cli.tool.as_str();

    match cli.command {
        None | Some(Commands::All) => {
            println!("🦀 rust-hoex · 01-helloWorld exercises\n");
            run_all(name, tool, verbose)?;
            println!("\n🎉 All exercises finished successfully.");
        }
        Some(Commands::List) => list_exercises(),
        Some(Commands::Greet) => run_exercise("greet", name, tool, verbose)?,
        Some(Commands::Banner) => run_exercise("banner", name, tool, verbose)?,
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
    println!("  cargo run -p exercise_helloworld -- list");
    println!("  cargo run -p exercise_helloworld -- greet [--name Analyst] [--verbose]");
    println!("  cargo run -p exercise_helloworld -- banner [--tool nmap] [--verbose]");
    println!("  cargo run -p exercise_helloworld -- all [--verbose]");
    println!("  cargo run -p exercise_helloworld              # same as 'all'");
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
