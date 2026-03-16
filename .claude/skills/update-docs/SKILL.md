---
name: update-docs
description: Update project documentation based on recent changes
user_invocable: true
---

# Update project documentation

Synchronize docs with the current state of the codebase.

## Step 1: Understand recent changes

```
git log --oneline -20
```

Read commit messages to understand what was added/changed since docs were last updated.

## Step 2: Update ROADMAP.md

Read `ROADMAP.md`. Check boxes for features that are now implemented based on:
- Recent commits
- Existing test cases in `tasks/compiler_tests/cases2/`
- Code in `crates/`

Only check items where the feature is actually working (has passing tests).

## Step 3: Update CODEBASE_MAP.md

Read `CODEBASE_MAP.md`. Check if new public types, methods, or modules were added in recent commits. If so, add them to the appropriate sections. Only document `pub` and `pub(crate)` items that are part of the crate API.

## Step 4: Update GOTCHAS.md

Read `GOTCHAS.md`. If recent work uncovered non-obvious behaviors, edge cases, or tricky implementation details, add them. Skip this step if nothing new is relevant.

## Step 5: Report

Summarize what was updated and any items that need manual review.

## Rules

- Do not invent features — only mark items as done if there is evidence (tests, code)
- Keep ROADMAP.md as the source of truth for feature status
- Do not rewrite existing prose — only update checkboxes, add entries, or remove completed items
