# 14-traits

Shared threat-scoring behavior with a `ThreatScorer` trait implemented by auth,
network, and malware event types. Prefers **static dispatch** for beginners.

## What this teaches

- Defining a trait with required methods (`score`, `category`)
- Default methods (`risk_level`, `summary`) that implementors can override
- `impl ThreatScorer for AuthFailure` (and friends)
- Trait bounds on functions: `fn analyze_event<T: ThreatScorer>(…)`
- `derive` for boilerplate (`Debug`, `Clone`) vs manual domain `impl`s
- Why homogeneous slices + generics beat `dyn Trait` at this stage

## How to run

```bash
cd examples/14-traits
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_traits
cargo test -p example_traits
```

## Key concepts

### Trait definition

```rust
pub trait ThreatScorer {
    fn score(&self) -> u32;
    fn category(&self) -> &'static str;

    fn risk_level(&self) -> RiskLevel { /* default */ }
    fn summary(&self) -> String { /* default */ }
}
```

Required methods must be implemented. Default methods are inherited unless you
override them (`NetworkScan` overrides `summary`).

### Static dispatch

```rust
pub fn analyze_event<T: ThreatScorer>(event: &T) -> String {
    event.summary()
}
```

Each call site with a concrete type gets a specialized copy (like generics).
No runtime vtable. Mixing *different* types in one `Vec` would need
`Vec<Box<dyn ThreatScorer>>` — powerful, but save that for later.

### Derive vs manual

| Mechanism | Good for |
|-----------|----------|
| `#[derive(Debug, Clone, …)]` | Mechanical, compiler-known traits |
| Manual `impl ThreatScorer` | Domain rules (scoring formulas) |

### Trait bounds

`T: ThreatScorer` means “any type that implements this trait.” You can also
write `impl ThreatScorer` in argument position — same idea, slightly different
syntax.

## Exercises

1. Add `fn is_critical(&self) -> bool` as a default method on `ThreatScorer`.
2. Implement `ThreatScorer` for a new `FileIntegrityEvent { path, severity }`.
3. Write `fn max_score<T: ThreatScorer>(events: &[T]) -> u32`.
4. Override `risk_level` on `MalwareAlert` so confidence ≥ 50 is always `High`.

## Further reading

- [The Rust Book — Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [The Rust Book — Trait Objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) (optional next step)
- [Rust by Example — Traits](https://doc.rust-lang.org/rust-by-example/trait.html)
