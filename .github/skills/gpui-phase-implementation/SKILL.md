---
name: gpui-phase-implementation
description: Execute one GPUIShowCase phase with matching docs, validation, and harness-map updates.
---

# GPUI Phase Implementation

## Use This Skill When

- A single phase from `todolist.md` needs to be executed.
- The implementation must stay aligned with `docs/master-plan.md`.
- The work requires code, documentation, and verification updates together.

## Required Inputs

- Current target phase and step IDs from `todolist.md`
- Relevant architecture context from `docs/master-plan.md`
- Current status of `docs/harness-map.md`
- Relevant `quality/` rules

## Workflow

1. Confirm the target phase and its stop condition.
2. Read the relevant plan and harness-map sections.
3. Implement the smallest slice that satisfies the phase step.
4. Update all impacted living documents, including Markdown plans, skill files, and command checklists.
5. Run the required validation commands.
6. Update harness evidence or status.
7. Mark blockers instead of skipping broken validation.

## Project Constraints

- Stay with generic showcase data.
- Prefer upstream `gpui-component` patterns before inventing local frameworks.
- Do not move to the next phase early.
- Do not leave stale instructions, stale commands, or stale progress documents behind after implementation changes.
