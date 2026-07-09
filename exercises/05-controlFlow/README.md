# 05-controlFlow Exercises

**Threat-score** and **log classification** with `match`, loops, and `while let`:
keyword heuristics, severity tiers, batch limits, and budget accumulation.

## Overview

These exercises build on [`examples/05-controlFlow`](../../examples/05-controlFlow).
You classify synthetic security log lines using control-flow constructs that
appear daily in SIEM pipelines and policy engines.

## Learning Objectives

After completing these exercises, you should understand:

- [x] **`match` on ranges** — map numeric scores to `Severity` tiers
- [x] **`if` expressions** — cap raw scores inline
- [x] **`for` loops** — iterate log lines with `break` and `continue`
- [x] **`while let`** — walk iterators without manual indexing
- [x] **Enum dispatch** — `Severity` labels and escalation checks
- [x] **Event budgets** — blank lines excluded from counters

## Project Layout

```text
exercises/05-controlFlow/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs          # Public API and orchestration
│   ├── main.rs         # clap CLI
│   ├── classifier.rs   # Core scoring helpers
│   ├── scoring.rs      # Scoring exercise
│   └── batch.rs        # Batch classification exercise
└── tests/
    └── integration.rs
```

## Running the Exercises

```bash
cargo run -p exercise_controlflow
cargo run -p exercise_controlflow -- list
cargo run -p exercise_controlflow -- scoring --verbose
cargo run -p exercise_controlflow -- batch
cargo run -p exercise_controlflow -- all
```

Quality checks:

```bash
cargo test -p exercise_controlflow
cargo clippy -p exercise_controlflow --all-targets --all-features -- -D warnings
cargo fmt -p exercise_controlflow
```

## Exercises

### 1. `scoring` — Keywords and severity mapping

Scores individual log lines by keyword weights, caps to 0..=100, and maps to
`Severity` with `match`.

**Functions:** `score_log_line`, `cap_score`, `classify_score`, `severity_label`

### 2. `batch` — Batch classification with limits

Classifies log batches, skips empty lines, stops at `max_events`, and walks
lines with `while let`.

**Functions:** `classify_batch`, `walk_nonempty_lines`, `accumulate_until_budget`

## Public API

| Function / Type | Signature | Concept |
|-----------------|-----------|---------|
| `Severity` | enum | Discrete alert tiers |
| `classify_score` | `(u32) -> Severity` | Range matching |
| `score_log_line` | `(&str) -> u32` | Keyword heuristics |
| `classify_batch` | `(&[&str], usize) -> Vec<(String, Severity)>` | Batch + limit |
| `walk_nonempty_lines` | `(&[&str]) -> Vec<&str>` | while-let walk |

## Key Takeaways

1. **`match` must be exhaustive** — `_` arm handles out-of-range scores.
2. **Skip blanks explicitly** — empty lines are not security events.
3. **Budget loops use `break`** — stop early instead of over-processing.
4. **Enums encode policy** — severity drives escalation decisions.

## Related Material

- [`examples/05-controlFlow`](../../examples/05-controlFlow) — introductory walkthrough
- [`examples/06-ownership`](../../examples/06-ownership) — next topic: ownership
