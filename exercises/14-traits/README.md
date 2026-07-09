# 14-traits Exercises

Shared threat-scoring behavior with the `ThreatScorer` trait — auth failures, network scans, malware alerts, and file integrity events.

## Overview

SOC platforms score heterogeneous security events through a common interface. This crate defines `ThreatScorer` with required methods, sensible defaults, and type-specific overrides — all via **static dispatch** (generics, no `dyn` yet).

## Project layout

```bash
exercises/14-traits/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── scoring.rs          # ThreatScorer, types, max_score
│   ├── auth.rs             # Exercise 1
│   ├── network.rs          # Exercise 2
│   └── malware.rs          # Exercise 3
└── tests/integration.rs
```

## Learning objectives

- [x] Define a trait with required + default methods
- [x] Implement `ThreatScorer` for multiple event types
- [x] Override `risk_level` and `summary` where needed
- [x] Use trait bounds (`T: ThreatScorer`) for static dispatch
- [x] Write `max_score` over homogeneous slices

## Running

```bash
cargo run -p exercise_traits
cargo run -p exercise_traits -- network --verbose
cargo test -p exercise_traits
cargo clippy -p exercise_traits --all-targets --all-features -- -D warnings
cargo fmt -p exercise_traits
```

## ThreatScorer trait

```rust
pub trait ThreatScorer {
    fn score(&self) -> u32;              // required
    fn category(&self) -> &'static str; // required

    fn risk_level(&self) -> RiskLevel { /* default bands */ }
    fn summary(&self) -> String { /* default one-liner */ }
    fn is_critical(&self) -> bool { /* score ≥ 85 or Critical */ }
}
```

### Default `risk_level` bands

| Score | Band |
|-------|------|
| 0–24 | Low |
| 25–59 | Medium |
| 60–84 | High |
| 85–100 | Critical |

### `is_critical` default

Returns `true` when `score >= 85` **or** `risk_level() == Critical`.

## Implementors

| Type | Category | Notes |
|------|----------|-------|
| `AuthFailure` | `auth` | Attempts × 15, +25 if unknown host |
| `NetworkScan` | `network` | Overrides `summary()` |
| `MalwareAlert` | `malware` | Overrides `risk_level()` — see below |
| `FileIntegrityEvent` | `integrity` | Severity maps directly to score |

### MalwareAlert `risk_level` override

- **confidence ≥ 50** → at least **High** (even if score band would be Medium)
- **score ≥ 85** → **Critical** (never downgraded by the floor rule)

Example: confidence 55 → score 55 → default would be Medium, override → **High**.  
Confidence 90 → score 90 → **Critical**.

## Helpers

```rust
let line = analyze_event(&scan);       // T: ThreatScorer
let urgent = is_actionable(&alert);    // High or Critical
let peak = max_score(&events);         // highest score in slice
```

## Exercises

1. **auth** — `AuthFailure`, default methods, `is_critical`
2. **network** — `NetworkScan`, overridden `summary`, `max_score`
3. **malware** — `MalwareAlert` + `FileIntegrityEvent`, confidence floor

## Derive vs manual

| Mechanism | Use for |
|-----------|---------|
| `#[derive(Debug, Clone, …)]` | Boilerplate traits |
| `impl ThreatScorer for …` | Domain scoring rules |

## Testing

Unit tests in `scoring.rs` and exercise modules; integration tests cover cross-type behavior.

## Related material

- Example: `examples/14-traits`
- [The Rust Book — Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

## Stretch goals

1. Add `fn summarize_all<T: ThreatScorer>(events: &[T]) -> Vec<String>`
2. Introduce `Box<dyn ThreatScorer>` for heterogeneous event batches
3. Implement `Display` for each event type alongside the trait
