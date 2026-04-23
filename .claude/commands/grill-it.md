---
description: Manual adversarial review that challenges recent work against the quality bar. Use only when the user explicitly asks for `grill-it` or `/grill-it`.
argument-hint: "[session | diff | commit | branch | file-path]"
---

# Grill it: $ARGUMENTS

Manual adversarial review. No auto-trigger for generic review, critique, cleanup, or post-implementation requests. Run only when user explicitly asks for `grill-it` or `/grill-it`.

## Review Scope

Interpret `$ARGUMENTS`:

- empty or `session` — grill everything done in current conversation (all code, decisions, approaches)
- `diff` — grill uncommitted changes (staged + unstaged)
- `commit` — grill last commit (`HEAD~1..HEAD`)
- `branch` — grill all changes on current branch vs master (`git diff master...HEAD`)
- file path — grill that file

## Questions

Answer these:

**Are these changes systematic, without workarounds or temporary solutions, respecting crate and module boundaries? Will these changes not create problems in the long-term perspective of the project? Did we not reinvent the wheel yet again?**

**Will another person understand this without additional explanation? Is the implementation simple and straightforward, or more complex than necessary?**
