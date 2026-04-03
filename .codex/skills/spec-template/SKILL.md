---
name: spec-template
description: Canonical structure and rules for files under `specs/`. Use when creating a new spec, revising a spec layout, or updating `Current state`, `Use cases`, `Reference`, or `Tasks` sections during `audit` or `port`.
---

# Spec Template

## Naming

Use kebab-case:

- `state-rune.md`
- `each-block.md`

For roadmap-tier work, use `<tier-id>-<short-name>.md`.

## Fixed section order

```markdown
# <Feature name>

## Current state
- **Working**: N/M use cases
- **Missing**: K use cases
- **Next**: ...
- Last updated: <date>

## Source

## Syntax variants

## Use cases

## Out of scope

## Reference

## Tasks

## Implementation order

## Discovered bugs

## Test cases
```

Put `Current state` first. It is the session handoff section.

## Scope rules

- client-side only unless explicitly stated otherwise
- `[ ]` means in scope and not done
- `[x]` means implemented and covered by test
- `[~]` means partial
- Use cases: flat list of checkboxes only — no `###` subsections
- Out of scope: plain list (no checkboxes) for things explicitly excluded (SSR, removed features, future tiers)

## Effort markers

- quick fix
- moderate
- needs infrastructure

## Lifecycle

1. Create the spec during `audit` or `port`.
2. Update `Current state` after each session.
3. Keep completed specs as long-term reference instead of deleting them.
