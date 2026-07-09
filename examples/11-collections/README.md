# 11-collections

Intrusion log aggregation with Rust's core collections: `Vec`, `HashMap`, and
`HashSet`.

## What this teaches

- Growing an ordered event stream with `Vec::push` and iterating with `.iter()`
- Counting with `HashMap` and the **entry API** (`entry(...).or_insert(0)`)
- Deduplicating usernames with `HashSet`
- Filtering into a new `Vec` with iterator adapters
- Ownership: collections own their elements; `merge_logs` moves two `Vec`s

## How to run

```bash
cd examples/11-collections
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_collections
cargo test -p example_collections
```

## Key concepts

### `Vec<T>` — ordered list

```rust
let mut log = IntrusionLog::new();
log.push(LogEvent::new("10.0.0.1", "alice", "login_fail"));
```

`Vec` owns each `LogEvent`. Iterating with `for event in &log.events` borrows.

### `HashMap<K, V>` — keyed lookup

```rust
let counter = counts.entry(ip.clone()).or_insert(0);
*counter += 1;
```

`entry` avoids a separate contains/insert dance and yields a mutable reference.

### `HashSet<T>` — unique values

```rust
users.insert(event.user.clone()); // false if already present
```

Ideal for “have we seen this username before?” without caring about order.

### Ownership with collections

`merge_logs(left, right)` **consumes** both aggregators and `extend`s one `Vec`
with the other. After the call, `left` and `right` are gone — their events live
in the returned log.

## Exercises

1. Add `fn actions(&self) -> HashSet<String>` returning every distinct action.
2. Change `hot_ips` to return `HashMap<String, u32>` (IP → count) instead of a
   sorted `Vec`.
3. Implement `fn top_user(&self) -> Option<String>` using a `HashMap` of user →
   event count and picking the max.
4. Avoid cloning IPs in `ip_counts` by returning `HashMap<&str, u32>` with a
   lifetime tied to `&self` (advanced).

## Further reading

- [The Rust Book — Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [HashMap::entry](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
- [Rust by Example — HashMap](https://doc.rust-lang.org/rust-by-example/std/hash/hashmap.html)
