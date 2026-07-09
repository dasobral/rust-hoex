# 17-iterators

Iterator adapters for processing firewall-style log lines and IP lists.

## What this teaches

- The `Iterator` trait and `next`
- Adapters: `map`, `filter`, `filter_map`, `enumerate`, `take`
- Consumers: `collect`, `fold`, `for`
- Lazy adapter chains vs consuming
- `into_iter` vs `iter` vs `iter_mut`

## How to run

```bash
cd examples/17-iterators
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
```

```bash
cargo run -p example_iterators
cargo test -p example_iterators
```

## Key concepts

### Lazy chains

```rust
logs.iter()
    .filter(|log| log.action == "DENY")
    .map(|log| log.src.as_str())
    .collect()
```

Nothing runs until `collect` (or another consumer) pulls values.

### Three ways to iterate

- `iter()` — borrow each element (`&LogLine`)
- `iter_mut()` — mutate in place (`&mut LogLine`)
- `into_iter()` — consume the `Vec` and take ownership of each `LogLine`

## Exercises

1. Add `allowed_bytes(logs) -> u64` using `filter` + `map` + `sum`.
2. Rewrite `count_allows_with_next` using `.filter(...).count()`.
3. Use `inspect` to print each denied IP while building `denied_ips`.

## Further reading

- [The Rust Book — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
