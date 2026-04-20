---
applyTo: '**/*.md'
---

# Documentation Instructions

## Purpose

- Keep planning and execution documents directly usable by a coding agent.
- Write steps with clear inputs, outputs, and validation points.

## Required Standards

- Prefer explicit checklists and short sections over vague prose.
- Keep file and folder names consistent with the workspace.
- Update planning docs when scope changes.
- Document assumptions, blockers, and deferred work explicitly.
- Treat all Markdown files, local skills, and command lists as living documents that must stay aligned with the current repository state.
- When implementation changes a command, path, phase boundary, or validation rule, update the affected document in the same unit of work.

## Project-Specific Rules

- `todolist.md` is the primary execution guide.
- `docs/harness-map.md` tracks verification coverage.
- `quality/` files define the review and integration discipline.
- Avoid reintroducing stock or trading terminology in planning docs unless the scope changes intentionally.
- Skill files under `.github/skills/` are also living artifacts and should evolve with the workflow rather than remain static samples.
- If the user must know something, decide something, confirm something, or enter something, the plan and implementation must describe where that appears in the UI.
- Logging and background processing documents should identify both file-based evidence and user-visible UI surfaces.
