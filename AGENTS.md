# AGENTS.md

## Cursor Cloud specific instructions

This is `rust-hoex`, a Cargo workspace of small Rust learning programs. Standard
commands (build/test/clippy/fmt, `./utils/check.sh`, `./utils/creator.sh`) are
documented in `CLAUDE.md` and `README.md`; use those rather than re-deriving them.

### Toolchain

- Examples, exercises, and projects use **edition 2024** and
  `rust-version = "1.85"` (workspace package defaults). That requires
  **stable Rust >= 1.85**.
- The base image may ship an older stable (e.g. 1.83) that fails with
  `feature edition2024 is required`. The update script installs/defaults the
  latest stable plus `clippy` and `rustfmt`, so a fresh session is ready to build.

### Current workspace state

- Workspace members: `examples/*/`, `exercises/*/`, `projects/*/`. Package names
  are unique (`example_*`, `exercise_*`, `project_*`); workspace-level cargo
  commands load correctly.
- Tutorial content is complete: examples `01-helloWorld`–`23-unsafe`,
  `exercises/02-variables`, and `projects/cli-tools` (seccheck). Learning docs
  live under `docs/` (`00-basics` through `03-advanced`, plus `LEARNING_PATH.md`).
- `./utils/check.sh` (fmt, clippy with `-D warnings`, tests, etc.) is expected
  to pass on the completed tutorial. Prefer it (or the commands in `README.md` /
  `CLAUDE.md`) before claiming a change is clean.
