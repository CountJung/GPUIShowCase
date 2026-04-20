---
applyTo: '**/*'
---

# Testing Instructions

## Required Mindset

- Validation is part of implementation, not a final cleanup step.
- Prefer a small verified scope over a larger unverified scope.

## Required Coverage

- For each phase, define at least one smoke or manual check in `docs/harness-map.md`.
- For new state logic, add unit tests when the crate structure exists.
- For cross-component flows, define integration or manual scenario checks before implementation grows.
- For user-facing flows, include checks that required information and actions are visible in the UI.
- For logging-related work, validate both file output and the in-app log viewing surface.

## Failure Handling

- Never silence failing tests to keep momentum.
- Record the failing command, exact error, and current blocker state.
- If the codebase is not yet buildable, document the missing prerequisite rather than pretending the test passed.

## Quality Gates

- Build gate: `cargo check`
- Formatting gate: `cargo fmt --check`
- Lint gate: `cargo clippy --all-targets --all-features -- -D warnings`
- Test gate: `cargo test`
