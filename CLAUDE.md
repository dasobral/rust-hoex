# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **rust-hoex** (Rust Hands-On EXamples), a structured learning repository for Rust programming. It's organized as a Cargo workspace with examples, exercises, and projects designed to teach Rust concepts progressively from beginner to advanced levels.

## Key Commands

### Development Commands

- `cargo test --workspace` - Run all tests across the workspace
- `cargo build --workspace` - Build all workspace members
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` - Lint with warnings as errors
- `cargo fmt --all` - Format all code in the workspace
- `cargo check --workspace` - Quick compile check for all members
- `./utils/check.sh` - Run comprehensive quality checks (format, lint, test, doc, audit)

### Module Creation

- `./utils/creator.sh example <name> [description]` - Create new example
- `./utils/creator.sh project <name> [description]` - Create new project  
- `./utils/creator.sh exercise <name> [description]` - Create new exercise

### Aliases (if loaded via `source utils/aliases.sh`)

- `rust-check` - Run quality checks
- `rust-test` - Test workspace
- `rust-build` - Build workspace
- `rust-fmt` - Format code
- `rust-clippy` - Lint workspace
- `rust-example <name> [desc]` - Create example
- `rust-project <name> [desc]` - Create project
- `rust-exercise <name> [desc]` - Create exercise

## Architecture

### Workspace Structure

The repository uses Cargo workspace with three main member categories:

- `examples/*/` - Learning examples (currently active)
- `projects/*/` - Larger tutorial projects (commented out until created)
- `exercises/*/` - Coding challenges (commented out until created)

### Code Standards

- **Safety-first approach**: `unsafe_code = "forbid"` in workspace lints
- **Strict linting**: Clippy pedantic and nursery lints enabled with `unwrap_used = "deny"`
- **Consistent formatting**: Standard rustfmt across all modules
- **Comprehensive testing**: Both unit tests (in src/) and integration tests (in tests/)

### Module Template Structure

Each module follows this pattern:

```bash
module-name/
├── Cargo.toml          # Inherits workspace config
├── src/main.rs         # Main source with unit tests
├── README.md           # Module documentation
└── tests/integration.rs # Integration tests
```

### Dependencies

Common workspace dependencies include:

- `serde` - Serialization
- `tokio` - Async runtime  
- `clap` - CLI parsing
- `anyhow`/`thiserror` - Error handling
- `reqwest` - HTTP client
- `criterion` - Benchmarking

## Development Workflow

1. **Quality checks first**: Always run `./utils/check.sh` or `rust-check` before committing
2. **Test-driven development**: Each module includes both unit and integration tests
3. **Documentation**: Every module has comprehensive README.md with learning objectives
4. **Progressive complexity**: Examples build from basic concepts to advanced topics

## Important Notes

- The project is structured for learning, not production use
- Each example is self-contained with clear educational objectives
- The `creator.sh` script generates consistent module templates
- All modules must compile and pass quality checks
- Documentation emphasizes learning over API reference
