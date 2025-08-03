# Rust Basics: Understanding Your Learning Environment

Welcome to your Rust learning journey! This document explains the basic structure and concepts you'll encounter throughout this repository.

## Table of Contents

- [Project Structure](#project-structure)
- [Rust Program Anatomy](#rust-program-anatomy)
- [Testing in Rust](#testing-in-rust)
- [Basic Rust Concepts](#basic-rust-concepts)
- [Development Workflow](#development-workflow)

## Project Structure

Every example, exercise, and project in this repository follows a consistent structure:

```bash
01-helloWorld/
├── Cargo.toml           # Project configuration and dependencies
├── src/
│   └── main.rs         # Your main source code
├── README.md           # Documentation for this specific module
└── tests/              # Integration tests
    └── integration.rs
```

This structure is standard in the Rust ecosystem and you'll see it everywhere.

### Cargo.toml

This is your project's configuration file. It defines:

- Package metadata (name, version, authors)
- Dependencies (external libraries you want to use)
- Build settings and features

### src/main.rs

This is where your code lives. For simple programs, everything goes here. For larger projects, you might have multiple files in `src/`.

## Rust Program Anatomy

Let's break down the template you'll see in every new module:

```rust
// example: 01-helloWorld
// Hello World
//
// To run this program:
// 1. Navigate to this directory: cd examples/01-helloWorld
// 2. Run the program: cargo run
//
// Key concepts demonstrated:
// - [Add concepts here]

fn main() {
    println!("Hello from 01-helloWorld!");
    
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
```

### Key Elements

**Comments:**

- `//` starts a single-line comment
- `/* */` for multi-line comments (less common)
- Documentation comments start with `///` (we'll cover these later)

**The `main` function:**

- Every Rust program starts execution from `fn main()`
- `fn` declares a function
- `println!` is a macro (note the `!`) that prints to stdout

**The `todo!` macro:**

- A placeholder that compiles but panics when executed
- Useful during development to mark unfinished code
- Replace it with actual implementation as you learn

## Testing in Rust

Rust has excellent built-in testing support. We use two types of tests in this repository:

### 1. Unit Tests (in src/main.rs)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        assert_eq!(2 + 2, 4);
    }
}
```

**What this means:**

- `#[cfg(test)]` - "Only compile this when running tests"
- `mod tests` - Creates a module named "tests"
- `use super::*;` - Import everything from the parent module (your main code)
- `#[test]` - Marks a function as a test
- `assert_eq!` - Checks if two values are equal

**Unit tests are for:**

- Testing individual functions
- Testing internal logic
- Testing private functions (they can access private code)
- Fast feedback during development

### 2. Integration Tests (in tests/integration.rs)

```rust
// Integration tests for 01-helloWorld

#[test]
fn test_integration() {
    // TODO: Add integration tests
    assert_eq!(2 + 2, 4);
}
```

**Integration tests are for:**

- Testing your program's public API
- Testing how different parts work together
- Testing real-world usage scenarios
- Can only access public functions and modules

### Running Tests

```bash
# Run all tests (both unit and integration)
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration

# Run tests with output
cargo test -- --nocapture
```

### Why Both Types?

- **Unit tests**: Catch bugs early, test edge cases, document expected behavior
- **Integration tests**: Ensure your public API works correctly, test real usage

Throughout this repository, you'll see both. Don't worry if they seem similar at first - as examples get more complex, the distinction will become clearer.

## Basic Rust Concepts

### Variables and Mutability

```rust
fn main() {
    let x = 5;           // Immutable by default
    let mut y = 10;      // Mutable with 'mut' keyword
    
    // x = 6;            // ❌ Error! x is immutable
    y = 15;              // ✅ OK, y is mutable
    
    println!("x: {}, y: {}", x, y);
}
```

**Key points:**

- Variables are **immutable by default** (a safety feature)
- Use `mut` to make them mutable
- `let` declares a new variable

### Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}

fn print_message() {
    println!("Hello!"); // No return type = returns ()
}
```

**Key points:**

- `fn` declares functions
- `->` specifies return type
- Last expression (without `;`) is returned
- Use `()` for functions that don't return anything

### Macros vs Functions

```rust
println!("Hello");     // Macro (note the !)
print!("Hello");       // Macro
format!("Hello");      // Macro

some_function();       // Function (no !)
```

**Macros:**

- End with `!`
- Generate code at compile time
- More powerful than functions
- `println!`, `vec!`, `todo!`, `assert_eq!` are common macros

### Ownership (Preview)

Rust's most unique feature - we'll cover this in detail later:

```rust
let s1 = String::from("hello");
let s2 = s1;           // s1 is "moved" to s2
// println!("{}", s1); // ❌ Error! s1 no longer owns the data
println!("{}", s2);    // ✅ OK, s2 owns the data
```

Don't worry about understanding this yet - it's Rust's superpower for memory safety!

## Development Workflow

### Typical Development Cycle

1. **Create a new module:**

   ```bash
   rust-example 02-variables "Learning about variables"
   ```

2. **Navigate to the directory:**

   ```bash
   cd examples/02-variables
   ```

3. **Edit the code:**

   - Open `src/main.rs` in your editor
   - Replace `todo!()` with actual code
   - Add meaningful tests

4. **Run and test:**

   ```bash
   cargo run          # Run the program
   cargo test         # Run tests
   cargo check        # Quick compile check
   cargo clippy       # Linting suggestions
   cargo fmt          # Format code
   ```

5. **Document your learning:**
   - Update the README.md
   - Add comments explaining concepts
   - Note what you learned

### Cargo Commands Reference

```bash
cargo new <name>       # Create new project
cargo run              # Compile and run
cargo build            # Just compile
cargo test             # Run tests
cargo check            # Quick compile check (faster than build)
cargo clippy           # Linting and suggestions
cargo fmt              # Format code
cargo doc --open       # Generate and open documentation
```

## Common Patterns You'll See

### Error Handling (Preview)

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

### Pattern Matching (Preview)

```rust
match some_value {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Something else"),
}
```

### Collections (Preview)

```rust
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
println!("{:?}", vec); // [1, 2]
```

## Learning Tips

1. **Start small:** Don't try to understand everything at once
2. **Experiment:** Change the code, see what happens
3. **Read error messages:** Rust's compiler is very helpful
4. **Use tests:** They help you understand how code should work
5. **Don't fight the borrow checker:** Learn ownership gradually

## Next Steps

Now that you understand the basic structure:

1. Create your first example: `rust-example 01-helloWorld "My first Rust program"`
2. Explore the generated code
3. Replace `todo!()` with some simple code
4. Run tests and see them pass
5. Move on to variables, data types, and functions

Remember: Rust has a learning curve, but it's worth it. The compiler is your friend - it catches bugs before they become problems!

## Resources for This Section

- [The Rust Book - Chapter 1: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [The Rust Book - Chapter 11: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example - Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

---
