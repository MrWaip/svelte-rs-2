---
name: spec-template
description: MUST consult when creating or updating spec files in specs/. Contains the canonical spec structure, naming rules, scope rules, effort markers, and the allowed checklist shapes including nested sub-checkbox decomposition for broad use cases. Use this skill whenever /audit creates a new spec, or when updating an existing spec's structure.
---

# Spec Template

## Naming
File name = feature name in kebab-case: `state-rune.md`, `each-block.md`.
ROADMAP tier items: `<tier-id>-<short-name>.md` (e.g. `5a-diagnostics-infrastructure.md`).

## Structure

Sections in fixed order. Most important first.

```markdown
# <Feature name>

## Current state
- **Working**: N/M use cases
- **Tests**: X/Y green
- Last updated: <date>

## Source
<ROADMAP item reference or user request>

## How It Works
<!-- Optional: operational model for the feature -->
<!-- Describe when the feature activates, what gets classified or transformed, -->
<!-- and which conditions change the emitted/runtime behavior -->

## Syntax variants
<!-- All syntactic forms of this feature, one per line -->
<!-- Extracted from reference/docs/ and reference compiler parser -->
```svelte
{#feature expression}...{/feature}
{#feature expression}...{:clause value}...{/feature}
```

## Use cases

- [x] <description> (test: <test_name>)
- [ ] <description> — if partially implemented, say what works and what remains; otherwise note the next layer/task (test: <test_name>, #[ignore], quick fix)
- [ ] <broad use case that must be decomposed before porting directly>
  - [x] <explicit closed sub-use-case>
  - [ ] <still broad sub-use-case that needs more decomposition>
    - [x] <leaf closure item already done>
    - [ ] <leaf closure item still open with next missing behavior>

## Out of scope

- <item explicitly not in scope, e.g. SSR variant or removed feature>

## Reference
### Svelte
- <list of reference compiler files>

### Our code
- <list of our crate files>

## Test cases

- [x] <test_name>
- [ ] <test_name>
```

## Scope rules
- **Client-side only.** No SSR use cases.
- `[ ]` = in scope, still open. Partial work stays `[ ]` with completed part and remaining gap described inline
- `[x]` = done with test
- `Current state` = terse resume header, not changelog. Keep only `Working`, `Tests`, `Last updated`.
- No dated `Completed (...)`, `Confirmed gap (...)`, or history bullets under `Current state`.
- Durable findings go in `Use cases`, `Reference`, `Test cases`. Not `Current state`.
- `How It Works` = optional. Use when feature has non-obvious activation, classification, lowering, or runtime behavior that future `/port` must preserve.
- Use cases: no `###` subsections or numbered groups inside `Use cases`.
- Nested sub-checkboxes allowed to any depth when broad use case needs decomposition into smaller closure items.
- Every non-leaf parent use case stays `[ ]` until every child `[x]`.
- Prefer leaf closure items that `/port` closes completely in one run.
- `Working: N/M use cases` counts leaf closure items, not parent wrappers.
- `Tests: X/Y green` counts `[x]` entries over all checklist entries in `Test cases`.
- Out of scope: plain list (no checkboxes) for things explicitly excluded (SSR, removed features, future tiers)
- Omit `Out of scope` and `Syntax variants` if empty/not applicable

## Test cases rules
- Flat checklist only. No subsections ("Existing", "Added during audit", etc.)
- `[x]` = test passes, `[ ]` = test `#[ignore]` or skipped
- Test names extracted from `(test: ...)` annotations in use cases

## Effort markers for [ ] use cases
- **quick fix** — one file, add match arm or call by analogy
- **moderate** — 2-3 files, existing infrastructure
- **needs infrastructure** — new analysis pass, AST type, or codegen module

Use case needs decomposition -> put effort markers on leaf sub-checkboxes, not only on broad parents.

## Sections that do NOT belong in specs
- `Tasks` — implementation hints go inline with `[ ]` use cases
- `Discovered bugs` — OPEN bugs become `[ ]` use cases, FIXED bugs deleted
- `Implementation order` — order implied by use case sequence
- `Test cases` subsections — always flat checklist
- session-log bullets in `Current state` — keep progress history out of resume header

## Lifecycle
1. Created by `/audit` when feature needs tracked spec. `/port` resumes from existing spec
2. Updated after each session — Current state section
3. Complete when all use cases `[x]` -> move feature to ROADMAP Done
4. Never deleted. Remains as reference
