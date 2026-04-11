---
name: qa
description: Review recent implementation work for material, defensible issues in correctness, architecture, regression risk, and missing test coverage. Use after port, triage-test, improve, or other code changes when Codex should act like a strict reviewer and avoid low-value style or comment nitpicks.
---

# Review Material Quality

This command is read-only. It reviews changes and reports only material findings.

Do not apply fixes in this skill.

## Review Scope

Determine the review scope from the input:

- commit hash: review `git diff <hash>..HEAD`
- range such as `HEAD~N`: review `git diff <range>..HEAD`
- directory or file path: review all code in that path, not just a diff
- no input: auto-detect in order
  1. uncommitted changes
  2. last commit

Read enough surrounding code to understand behavior and ownership. Do not review isolated diff hunks without context.

## Review Standard

Review as a strict senior engineer, not as a linter.

Report only findings that are both:

- material
- defensible from the code and project rules

If a concern is weak, speculative, or mostly stylistic, do not report it as a finding.

## What To Look For

Prioritize in this order:

1. incorrect behavior or likely regressions
2. crate or module boundary violations
3. missing or misplaced ownership of logic between parser, analyze, transform, and codegen
4. ad hoc logic, duplicated classification, or workaround-shaped changes that will age badly
5. missing tests for logic changes

Low priority by default:

- comment wording
- naming preferences
- style nits
- formatting

Only report low-priority issues when they materially harm correctness, maintainability, or future feature work.

## Review Questions

Answer these questions while reviewing:

1. Is the behavior correct for the changed path and nearby edge cases?
2. Does the logic live in the right layer?
3. Is analysis data computed once in the right place rather than rediscovered downstream?
4. Did the change introduce shortcuts, duplicated logic, or temporary-shape code?
5. Are tests at the right level and sufficient for the behavior that changed?

## Output

If material findings exist, report them first, ordered by severity.

Use this format:

```text
FINDINGS:
1. [severity] [file:line] — [what is wrong, why it matters, and what risk it creates]
2. ...

OPEN QUESTIONS:
1. ...

RESIDUAL RISKS:
1. ...
```

Rules:

- `severity` must be one of `high`, `medium`, `low`
- include `OPEN QUESTIONS` only when they block confidence in the review
- include `RESIDUAL RISKS` only when there are no direct findings but some verification gap remains
- do not add a fix plan unless the user asks for one

If there are no material findings, say exactly:

```text
No material findings.
```

Optionally add one short `RESIDUAL RISKS` block if coverage or verification is incomplete.

## Hard Rules

- do not report speculative violations just because something might be wrong
- do not force a finding when none is well-supported
- do not turn style preferences into architectural findings
- do not auto-fix
- prefer one strong finding over five weak ones
