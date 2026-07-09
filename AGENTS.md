# AGENTS.md

## Cursor Cloud specific instructions

This is `rust-hoex`, a Cargo workspace of small Rust learning programs. Standard
commands (build/test/clippy/fmt, `./utils/check.sh`, `./utils/creator.sh`) are
documented in `CLAUDE.md` and `README.md`; use those rather than re-deriving them.

### Toolchain

- The example crates set `edition = "2024"`, which requires **stable Rust >= 1.85**.
  The base image may ship an older stable (e.g. 1.83) that fails with
  `feature edition2024 is required`. The update script installs/defaults the
  latest stable plus `clippy` and `rustfmt`, so a fresh session is ready to build.

### Current workspace state (gotchas)

- `cargo <cmd> --workspace` from the repo root currently **fails to even load the
  workspace**: `examples/02-variables` and `exercises/02-variables` both declare
  the package name `variables` (`error: two packages named 'variables'`). This
  blocks every workspace-level cargo command, including on individual members
  (cargo walks up to the shared workspace root). To work on the examples, resolve
  the collision (rename one package) or temporarily scope `members` in the root
  `Cargo.toml` to `examples/*/`.
- `exercises/02-variables` is unfinished WIP and will not compile even after the
  name collision is resolved: `src/lib.rs` declares `quantum`, `electromagnetic`,
  and `temperature` modules that have no source files, its first line uses `/!`
  instead of `//!`, and `src/main.rs` calls `todo!()`. Treat this crate as
  in-progress; the working "application" is the `examples/*` programs.
- `./utils/check.sh` runs `cargo clippy ... -D warnings` and `cargo fmt --all --
  --check`. Both currently fail on the existing example code (benign
  `unused_variables`/`assertions_on_constants` clippy warnings and rustfmt drift
  in `examples/02-variables`), independent of the environment. `cargo build`,
  `cargo run`, and `cargo test` succeed for the examples.
