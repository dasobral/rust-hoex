#!/bin/bash

# rust-hoex alias setup
# Source this file to add convenient aliases for the repository
if [[ "${BASH_SOURCE[0]}" == "${0}" ]] && ( [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]] ); then
    echo "🦀 Rust learning aliases loaded!"
    echo
    echo "Available commands:"
    echo "  rust-create <type> <name> [description]  - Create new module"
    echo "  rust-example <name> [description]        - Create new example"
    echo "  rust-exercise <name> [description]       - Create new exercise"
    echo "  rust-project <name> [description]        - Create new project"
    echo "  rust-check                               - Run quality checks"
    echo "  rust-test                                - Run all tests"
    echo "  rust-build                               - Build workspace"
    echo "  rust-fmt                                 - Format all code"
    echo "  rust-clippy                              - Run clippy on workspace"
    echo
    echo "Examples:"
    echo "  rust-example 11-collections \"Working with Vec and HashMap\""
    echo "  rust-project web-api \"Simple REST API with axum\""
    echo "  rust-exercise fizzbuzz \"Classic FizzBuzz implementation\""
    exit 0
fi

# Get the absolute path to the repository root
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Function to create a new rust module (example, exercise, or project)
rust-create() {
    "$REPO_ROOT/utils/creator.sh" "$@"
}

# Convenient shortcuts for specific types
rust-example() {
    rust-create example "$@"
}

rust-exercise() {
    rust-create exercise "$@"
}

rust-project() {
    rust-create project "$@"
}

# Quality check shortcut
rust-check() {
    (cd "$REPO_ROOT" && ./utils/check.sh)
}

# Workspace operations
rust-test() {
    (cd "$REPO_ROOT" && cargo test --workspace)
}

rust-build() {
    (cd "$REPO_ROOT" && cargo build --workspace)
}

rust-fmt() {
    (cd "$REPO_ROOT" && cargo fmt --all)
}

rust-clippy() {
    (cd "$REPO_ROOT" && cargo clippy --workspace --all-targets --all-features)
}

# Quick navigation (if you want to add these)
# Uncomment the ones you find useful:

# rust-examples() {
#     cd "$REPO_ROOT/examples"
# }

# rust-projects() {
#     cd "$REPO_ROOT/projects" 
# }

# rust-exercises() {
#     cd "$REPO_ROOT/exercises"
# }

# Export functions so they're available in the shell
export -f rust-create rust-example rust-exercise rust-project
export -f rust-check rust-test rust-build rust-fmt rust-clippy
