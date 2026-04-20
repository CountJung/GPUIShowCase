# GPUIShowCase Copilot Instructions

## Project Scope

- Build a Rust desktop showcase app around `gpui` and `gpui-component`.
- Treat this repository as an app, not a reusable library first.
- Keep the scope aligned with `docs/master-plan.md`, but prefer generic showcase data over domain-specific finance or stock scenarios.

## Phase Execution Rules

- Work one phase at a time from `todolist.md`.
- Do not start the next phase until the current phase has code, docs, validation, and harness-map updates.
- Update `docs/harness-map.md` whenever a new verification path appears.
- Keep `quality/` documents synchronized with the actual implementation and risks.
- Treat all Markdown documents, skill files, and command checklists as living artifacts that must be updated whenever the working state changes.
- If a command, step list, or verification rule becomes outdated during implementation, update the source document in the same phase instead of leaving it stale.

## Dependency Rules

- Validate `gpui` and `gpui-component` APIs against upstream examples before introducing new abstractions.
- Prefer minimal wrappers around upstream APIs in the first implementation.
- Record unresolved API gaps as spikes or blockers instead of inventing unstable local conventions.

## Error Handling Rules

- Never ignore build errors, runtime errors, warnings, or failing tests.
- Record repro steps, observed messages, likely cause, and follow-up action before moving on.
- If a workaround is necessary, document the trade-off and the condition for removing it.

## Data Showcase Rules

- Use neutral operational datasets such as component metadata, system metrics, file trees, or activity events.
- Avoid stock, trading, or market-data assumptions unless explicitly reintroduced later.

## User Visibility Rules

- Any information the user needs to know or any input the user must provide must be surfaced in the UI.
- Background work, loading states, blockers, confirmations, and required actions must not exist only in logs or files.
- Logging must support rolling files and an in-app log viewing surface so the user can inspect runtime activity from the UI.

## Completion Gate

- Phase work is only complete when the relevant docs, validation commands, and harness entries are updated.
- A phase is not complete if the code changed but the corresponding living documents did not.
