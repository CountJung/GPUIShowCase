---
applyTo: '**/*'
---

# Code Review Instructions

## Review Priorities

- Find correctness risks first.
- Then review state flow, rendering behavior, logging, and maintainability.
- Treat undocumented assumptions as review findings.

## Required Checks

- Verify changed code against the current plan in `todolist.md`.
- Verify that new behavior has a harness-map entry or a justified exemption.
- Verify that warnings and known errors are not being deferred silently.
- Verify that sample data remains generic and does not drift into unnecessary finance-specific behavior.
- Verify that user-relevant information and required inputs are surfaced in the UI rather than hidden only in files or terminal output.
- Verify that logging features have both runtime persistence and an explicit in-app viewing path when they are part of user workflows.

## Review Guardrails

- Cite specific files and lines when raising findings.
- Read the affected code or document section before claiming a mismatch.
- If a validation step was skipped, record it explicitly as a gap.
