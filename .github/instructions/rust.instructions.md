---
applyTo: '**/*.rs'
---

# Rust Instructions

## Core Rules

- Follow idiomatic Rust naming and module boundaries.
- Keep public APIs small and explicit.
- Prefer `Result<T, E>` with meaningful error types over panics.
- Avoid `unwrap` and `expect` outside tests and bootstrap code that is intentionally fatal.

## Project-Specific Rules

- Preserve the feature-oriented structure from `docs/master-plan.md` and `todolist.md`.
- Separate app shell, feature modules, shared state, and logging concerns.
- Introduce generic sample data models before domain-specific ones.
- Keep GPUI view code simple; push complex state transitions into dedicated state models.

## Validation Rules

- Run `cargo fmt` after non-trivial edits.
- Run `cargo check` before considering a change complete.
- Run `cargo clippy --all-targets --all-features -- -D warnings` for phase-level completion when the crate exists.
- Add tests with each new stateful or non-trivial behavior when practical.
