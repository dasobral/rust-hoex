#!/bin/bash

# rust-hoex module creator
# Creates the basic structure for examples, exercises, or projects

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to show usage
show_usage() {
    echo -e "${BLUE}🦀 Rust Module Creator${NC}"
    echo
    echo "Usage: $0 <type> <name> [description]"
    echo
    echo "Types:"
    echo "  example    - Create a new example in examples/"
    echo "  exercise   - Create a new exercise in exercises/"
    echo "  project    - Create a new project in projects/"
    echo
    echo "Examples:"
    echo "  $0 example 11-collections \"Working with Vec and HashMap\""
    echo "  $0 project web-api \"Simple REST API with axum\""
    echo "  $0 exercise fizzbuzz \"Classic FizzBuzz implementation\""
    echo
    echo "Note: The script must be run from the repository root directory."
}

# Function to validate inputs
validate_inputs() {
    if [ $# -lt 2 ]; then
        echo -e "${RED}❌ Error: Missing required arguments${NC}"
        show_usage
        exit 1
    fi

    local type="$1"
    case "$type" in
        example|exercise|project)
            ;;
        *)
            echo -e "${RED}❌ Error: Invalid type '$type'${NC}"
            echo "Valid types: example, exercise, project"
            exit 1
            ;;
    esac

    # Check if we're in the repository root
    if [ ! -f "Cargo.toml" ] && [ ! -f "cargo.toml" ] || [ ! -d "examples" ] || [ ! -d "utils" ]; then
        echo -e "${RED}❌ Error: This script must be run from the repository root directory${NC}"
        echo "Current directory: $(pwd)"
        echo "Looking for: Cargo.toml (or cargo.toml), examples/, utils/"
        exit 1
    fi
}

# Function to convert name to valid Rust package name
sanitize_package_name() {
    local name="$1"
    # Remove leading digits and convert to valid package name
    # Replace hyphens with underscores, remove invalid characters
    echo "$name" | sed 's/^[0-9]*-*//' | sed 's/-/_/g' | sed 's/[^a-zA-Z0-9_]//g' | tr '[:upper:]' '[:lower:]'
}

# Function to create Cargo.toml
create_cargo_toml() {
    local dir_name="$1"
    local type="$2"
    local description="$3"
    
    # Generate a valid package name
    local package_name=$(sanitize_package_name "$dir_name")
    # If package name is empty after sanitization, use a default
    if [ -z "$package_name" ]; then
        package_name="${type}_example"
    fi
    
    cat > Cargo.toml << EOF
[package]
name = "$package_name"
version = "0.1.0"
edition = "2021"
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "$description"

[dependencies]
# Add dependencies as needed
# Common ones available in workspace:
# serde = { workspace = true }
# tokio = { workspace = true }
# clap = { workspace = true }
# anyhow = { workspace = true }

[[bin]]
name = "$package_name"
path = "src/main.rs"
EOF
}

# Function to create main.rs template
create_main_rs() {
    local dir_name="$1"
    local type="$2"
    local description="$3"
    
    cat > src/main.rs << EOF
// $type: $dir_name
// $description
//
// To run this program:
// 1. Navigate to this directory: cd ${type}s/$dir_name
// 2. Run the program: cargo run
//
// Key concepts demonstrated:
// - [Add concepts here]

fn main() {
    println!("Hello from $dir_name!");
    
    // TODO: Implement your code here
    todo!("Implement the main functionality");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // TODO: Add tests
        assert_eq!(2 + 2, 4);
    }
}
EOF
}

# Function to create README.md
create_readme() {
    local name="$1"
    local type="$2"
    local description="$3"
    local type_capitalized="$(tr '[:lower:]' '[:upper:]' <<< ${type:0:1})${type:1}"
    
    cat > README.md << EOF
# $name

$description

## Overview

This $type demonstrates:
- [Concept 1]
- [Concept 2]
- [Concept 3]

## Learning Objectives

After completing this $type, you should understand:
- [ ] [Objective 1]
- [ ] [Objective 2]
- [ ] [Objective 3]

## Running the Code

\`\`\`bash
# Run the program
cargo run

# Run tests
cargo test

# Check code with clippy
cargo clippy

# Format code
cargo fmt
\`\`\`

## Key Concepts

### [Concept 1]
[Explanation of the first key concept]

### [Concept 2]
[Explanation of the second key concept]

## Exercises

1. [Exercise 1 description]
2. [Exercise 2 description]
3. [Exercise 3 description]

## Further Reading

- [The Rust Book - Relevant Chapter](https://doc.rust-lang.org/book/)
- [Rust by Example - Relevant Section](https://doc.rust-lang.org/rust-by-example/)

## Related Examples

- [Link to related example 1]
- [Link to related example 2]
EOF
}

# Function to create integration test template
create_integration_test() {
    local dir_name="$1"
    
    mkdir -p tests
    cat > tests/integration.rs << EOF
// Integration tests for $dir_name

#[test]
fn test_integration() {
    // TODO: Add integration tests
    assert_eq!(2 + 2, 4);
}
EOF
}

# Main function
main() {
    local type="$1"
    local name="$2"
    local description="${3:-A Rust $type for learning}"
    
    validate_inputs "$@"
    
    local target_dir="${type}s/$name"
    
    # Check if directory already exists
    if [ -d "$target_dir" ]; then
        echo -e "${YELLOW}⚠️  Directory '$target_dir' already exists${NC}"
        read -p "Do you want to overwrite it? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${BLUE}Operation cancelled${NC}"
            exit 0
        fi
        echo -e "${YELLOW}📁 Removing existing directory...${NC}"
        rm -rf "$target_dir"
    fi
    
    echo -e "${BLUE}🦀 Creating $type: $name${NC}"
    echo -e "${YELLOW}📁 Creating directory structure...${NC}"
    
    # Create directory structure
    mkdir -p "$target_dir/src"
    cd "$target_dir"
    
    # Create files
    echo -e "${YELLOW}📄 Creating Cargo.toml...${NC}"
    create_cargo_toml "$name" "$type" "$description"
    
    echo -e "${YELLOW}📄 Creating src/main.rs...${NC}"
    create_main_rs "$name" "$type" "$description"
    
    echo -e "${YELLOW}📄 Creating README.md...${NC}"
    create_readme "$name" "$type" "$description"
    
    echo -e "${YELLOW}📄 Creating integration tests...${NC}"
    create_integration_test "$name"
    
    # Go back to repository root
    cd - > /dev/null
    
    # Test that the new module compiles
    echo -e "${YELLOW}🔍 Checking that the new module compiles...${NC}"
    if (cd "$target_dir" && cargo check --quiet); then
        echo -e "${GREEN}✅ Module compiles successfully${NC}"
    else
        echo -e "${YELLOW}⚠️  Module created but has compilation issues${NC}"
        echo "You can check errors later with: cd $target_dir && cargo check"
    fi
    
    echo -e "${GREEN}🎉 Successfully created $type: $name${NC}"
    echo
    echo "Next steps:"
    echo "1. Navigate to the directory: cd $target_dir"
    echo "2. Edit src/main.rs to implement your code"
    echo "3. Update README.md with specific details"
    echo "4. Run the code: cargo run"
    echo "5. Add tests and run: cargo test"
    echo
    echo -e "${BLUE}Happy coding! 🦀${NC}"
}

# Check if script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi