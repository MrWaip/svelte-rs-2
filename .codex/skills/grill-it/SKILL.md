---
name: grill-it
description: Challenge recent work against the quality bar. Use after implementation work to verify no shortcuts were taken, for the current session, the working tree diff, the last commit, the current branch, or a specific file.
---

## Review Scope

Interpret the user target like this:

- empty or `session`: grill everything done in the current conversation (all code written, decisions made, approaches taken)
- `diff`: grill uncommitted changes (staged + unstaged)
- `commit`: grill the last commit (`HEAD~1..HEAD`)
- `branch`: grill all changes on the current branch vs master (`git diff master...HEAD`)
- file path: grill that specific file

## Questions

Answer these questions:

**Are these changes systematic, without workarounds or temporary solutions, respecting crate and module boundaries? Will these changes not create problems in the long-term perspective of the project? Did we not reinvent the wheel yet again?**
