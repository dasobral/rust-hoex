# Rust Hands-On EXamples (rust-hoex)

A personal, non-official and beginner-friendly repository for learning Rust

## First steps

If you have never used Rust before, you will probably need to install it in your system

```bash
# Install rustup (Rust installer and version management tool)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment (or restart your terminal)
source ~/.cargo/env

# Verify installation
rustc --version       # This is the compiler
cargo --version       # This is the build system and package manager (like pip + make)

# Install useful components
rustup component add clippy    # Linter for catching common mistakes (normally automatically installed)
rustup component add rustfmt   # Code formatter (normally automatically installed)
rustup component add rust-src  # Source code for better IDE support

# Optional but recommended: install rust-analyzer for IDE support
# If using VS Code, install the "rust-analyzer" extension
```

## Repository Structure

```bash
rust-hoex/
├── README.md                 # Main project documentation
├── LICENSE                   # MIT License
├── .gitignore                # Rust-specific gitignore
├── Cargo.toml                # Workspace configuration
├── setup.sh                  # Environment setup script
├── docs/                     # Learning documentation
│   ├── 00-basics.md          # Installation and setup guide
│   ├── 01-fundamentals.md    # Basic syntax and concepts
│   └── ...
├── examples/                 # Standalone example programs
│   ├── 01-helloWorld/
│   ├── 02-variables/
│   ├── 03-functions/
│   └── ...
├── projects/                # Larger tutorial projects
│   ├── cli-utils/           # Command-line applications
│   ├── web-server/          # Basic web server
│   ├── systems/             # Systems programming examples
│   └── algorithms/          # Data structures and algorithms
├── exercises/               # Coding challenges and exercises
│   ├── rustlings/           # Rustlings exercises (if used)
│   ├── advent-of-code/      # Advent of Code solutions
│   └── custom/              # Custom practice problems
├── benchmarks/              # Performance testing examples
├── tests/                   # Integration tests
└── utils/                   # Development utilities and scripts
    ├── aliases.sh           # Aliases for creation commands
    ├── creator.sh           # Automatic creation of examples, exercises and projects
    └── check.sh             # Code quality check script
```

## Learning Path Structure

### Phase 1: Fundamentals (examples/)

- **01-helloWorld**: First Rust program
- **02-variables**: Variables, mutability, shadowing
- **03-dataTypes**: Scalar and compound types
- **04-functions**: Function syntax, parameters, return values
- **05-controlFlow**: if/else, loops, match
- **06-ownership**: Core Rust concept - ownership rules
- **07-borrowing**: References and borrowing
- **08-structs**: Custom data types
- **09-enums**: Enums and pattern matching
- **10-modules**: Code organization

### Phase 2: Intermediate Concepts

- **11-collections**: Vec, HashMap, etc.
- **12-error-handling**: Result and Option types
- **13-generics**: Generic functions and structs
- **14-traits**: Defining shared behavior
- **15-lifetimes**: Advanced memory management
- **16-testing**: Unit and integration tests
- **17-iterators**: Functional programming concepts
- **18-closures**: Anonymous functions

### Phase 3: Advanced Topics

- **19-smart-pointers**: Box, Rc, RefCell
- **20-concurrency**: Threads and message passing
- **21-async**: Async/await programming
- **22-macros**: Metaprogramming
- **23-unsafe**: When and how to use unsafe code

### Phase 4: Real-World Projects (projects/)

- **cli-tools**: Command-line applications with `clap`
- **web-server**: HTTP servers with `axum` or `warp`
- **systems**: File I/O, network programming
- **algorithms**: Performance-critical code
- **embedded**: Basic embedded programming concepts

## Workspace Configuration

The root `Cargo.toml` defines a workspace to manage all sub-projects:

```toml
[workspace]
resolver = "2"
members = [
    "examples/*/",
    # Uncomment these as you create projects/exercises:
    # "projects/*/",
    # "exercises/*/",
]

[workspace.dependencies]
# Common dependencies used across examples
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"

[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
enum_glob_use = "deny"
pedantic = "warn"
nursery = "warn"
unwrap_used = "deny"
```

## Each Example Structure

Each example follows a consistent pattern:

```bash
examples/01-helloWorld/
├── Cargo.toml           # Project configuration
├── src/
│   └── main.rs         # Source code
├── README.md           # Example-specific documentation
└── tests/              # Tests (if applicable)
    └── integration.rs
```

## Development Workflow

### Initial Setup

1. **Install Rust** (see First steps above)
2. **Clone repository**: `git clone <repo> && cd rust-hoex`
3. **Run setup**: `./setup.sh`
4. **Load aliases permanently**: Add this line to your `~/.bashrc` or `~/.zshrc`:

   ```bash
   source /path/to/your/rust-hoex/utils/aliases.sh
   ```

Then restart your terminal or run `source ~/.bashrc`

### Creating New Modules

#### Option A: Direct Script Usage

```bash
# Create a new example
./utils/creator.sh example 01-helloWorld "Hello World program"

# Create a new project  
./utils/creator.sh project web-api "Simple REST API with axum"

# Create a new exercise
./utils/creator.sh exercise fizzbuzz "Classic FizzBuzz implementation"
```

#### Option B: Using Aliases (Recommended)

After loading aliases, use these convenient commands:

```bash
# Module creation
rust-example 01-helloWorld "Hello World program"
rust-project web-api "Simple REST API with axum"
rust-exercise fizzbuzz "Classic FizzBuzz implementation"

# Development tasks
rust-check    # Run quality checks (formatting, linting, tests)
rust-test     # Run all tests in workspace
rust-build    # Build entire workspace
rust-fmt      # Format all code
rust-clippy   # Run clippy linter on workspace
```

## Available Aliases

After sourcing `utils/aliases.sh`, you get these commands:

**Module Creation:**

- `rust-create <type> <n> [description]` - Generic module creator
- `rust-example <n> [description]` - Create new example
- `rust-exercise <n> [description]` - Create new exercise  
- `rust-project <n> [description]` - Create new project

**Development Tasks:**

- `rust-check` - Run quality checks (formatting, linting, tests)
- `rust-test` - Run all tests in workspace
- `rust-build` - Build entire workspace
- `rust-fmt` - Format all code
- `rust-clippy` - Run clippy linter on workspace

## Development Workflow

1. **Setup**: Run `./setup.sh` to install dependencies
2. **Development**: Work in individual example/project directories
3. **Testing**: `cargo test` in workspace root runs all tests
4. **Quality**: `./utils/check.sh` runs formatting, linting, and tests
5. **Documentation**: Each example includes comprehensive README

## Getting Started

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Clone and enter repository: `git clone <repo> && cd rust-hoex`
3. Run setup: `./setup.sh`
4. Create your first example: `./utils/creator.sh example 01-helloWorld "Hello World"`
5. Start coding: `cd examples/01-helloWorld && cargo run`

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)

## Contributing

This is a learning repository, but contributions for improvements are welcome!
Please ensure all examples:

- Include comprehensive documentation
- Follow Rust best practices
- Include tests where appropriate
- Are beginner-friendly with clear explanations
