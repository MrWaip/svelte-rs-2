---
name: fix-simplify
description: Implement a simplification from `SIMPLIFY_REVIEW.md`. Use after `review-simplify` produces a concrete finding and the user wants that numbered simplification, or all findings, applied without changing behavior.
---

# Fix Simplification Finding

## 1) Load the review

Read `SIMPLIFY_REVIEW.md`. If it does not exist, stop and ask for `review-simplify` first.

Support:

- one numbered finding: `#N` or `N`
- `all`, processed by impact order

Skip already struck-through findings.

## 2) Read the affected code

Read every referenced file. For Level 1 findings, read both upstream and downstream code paths before deciding how to simplify.

## 3) Plan with a balance check

Plan by level:

- Level 1: move better structure upstream, then simplify downstream
- Level 2: extract, inline, flatten, or remove dead weight within the crate
- Level 3: pick a canonical shared location and update call sites

Reject the simplification if it would create a god-function, heavy parameter plumbing, or worse readability.

## 4) Implement without changing behavior

Use the approved plan. Verify incrementally. Generated JS should remain identical.

If tests show behavior drift, treat that as a failed simplification and fix or revert your approach.

## 5) Update the review

Mark completed findings in `SIMPLIFY_REVIEW.md` with a short note explaining what was extracted, inlined, or deleted.

## Rules

- plan first
- preserve behavior exactly
- one finding at a time when processing `all`
