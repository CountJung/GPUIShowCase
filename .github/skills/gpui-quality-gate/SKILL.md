---
name: gpui-quality-gate
description: Apply GPUIShowCase quality gates for planning, review, integration checks, and error handling.
---

# GPUI Quality Gate

## Use This Skill When

- A phase is nearing completion.
- A change affects state flow, rendering, data views, theme behavior, or logging.
- A blocker or warning needs to be triaged instead of ignored.

## Required Checks

1. Confirm the expected validation commands for the current phase.
2. Confirm the corresponding harness-map entry exists.
3. Confirm `quality/QUALITY.md` rules still match the implementation.
4. Confirm warnings, failures, and temporary workarounds are documented.
5. Confirm the scope stayed generic and did not drift into stock-specific behavior.
6. Confirm all living documents and skill files reflect the current working state and command set.

## Output

- Pass with evidence
- Blocked with exact reason
- Deferred with explicit debt item and re-entry condition
