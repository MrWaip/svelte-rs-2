---
name: fix-boundary
description: Implement a fix for a finding from `BOUNDARY_REVIEW.md`. Use after `review-boundaries` identifies a concrete violation and the user wants that numbered finding, or all findings, fixed in boundary-correct upstream-first order.
---

# Fix Boundary Finding

## 1) Load the review

Read `BOUNDARY_REVIEW.md`. If it does not exist, stop and ask for `review-boundaries` first.

Support:

- one numbered finding: `#N` or `N`
- `all`, processed in order

Skip already struck-through findings.

## 2) Understand the finding fully

Read every file listed in the finding's evidence, plus the relevant upstream provider files such as `data.rs`, `scope.rs`, or parser types.

Use `CODEBASE_MAP.md` and `GOTCHAS.md` when the finding touches unfamiliar territory.

## 3) Plan before editing

Plan by class:

- re-parsing or string classification: add upstream structure or analysis classification, then replace downstream logic
- AST re-traversal: add precomputed analysis data, then remove downstream collection logic
- raw handoff: add an enum or accessor in analyze, then replace multi-flag branching
- late knowledge: preserve and pass through the lost data upstream
- misleading types: split or rename the type, then update consumers

Present the file order and exact provider-before-consumer changes before implementation.

## 4) Implement upstream first

Change the provider before the consumer. These fixes are refactors unless the review says otherwise, so generated JS should remain unchanged.

After meaningful steps, run the narrowest compile or test command that proves safety. Finish with `just test-all` when feasible.

## 5) Update the review

Mark fixed findings with strikethrough and a short fix note in `BOUNDARY_REVIEW.md`.

## Rules

- do not start coding without a plan
- one finding at a time when processing `all`
- stop after three failed attempts on the same approach
