---
name: spec-template
description: MUST consult when creating or updating spec files in specs/. Contains the canonical spec structure, naming rules, scope rules, and effort markers. Use this skill whenever /port or /audit creates a new spec, or when updating an existing spec's structure.
paths:
  - "specs/*.md"
---

# Spec Template

## Naming
File name = feature name in kebab-case: `state-rune.md`, `each-block.md`.
For ROADMAP tier items: `<tier-id>-<short-name>.md` (e.g. `5a-diagnostics-infrastructure.md`).

## Structure

Sections in fixed order. Most important first.

```markdown
# <Feature name>

## Current state
- **Working**: N/M use cases
- **Missing**: K use cases (list key ones)
- **Next**: [what to do next]
- Last updated: <date>

## Source
<ROADMAP item reference or user request>

## Syntax variants
<!-- All syntactic forms of this feature, one per line -->
<!-- Extracted from reference/docs/ and reference compiler parser -->
```svelte
{#feature expression}...{/feature}
{#feature expression}...{:clause value}...{/feature}
```

## Use cases

1. [x] <description> (covered, test: <test_name>)
2. [ ] <description> (test: <test_name>, #[ignore], quick fix)
3. [ ] <description> (test: <test_name>, #[ignore], needs infrastructure)

## Out of scope

- <item explicitly not in scope, e.g. SSR variant or removed feature>

## Reference
- Svelte reference:
  - <list of reference compiler files>
- Our code:
  - <list of our crate files>

## Tasks

### AST / Parser
- [ ] <concrete change>

### Analysis
- [ ] <concrete change>

### Codegen
- [ ] <concrete change>
```

## Scope rules
- **Client-side only.** No SSR use cases.
- `[ ]` = in scope, `[x]` = done with test, `[~]` = partial (describe what works)
- Use cases: flat list of checkboxes only — no `###` subsections
- Out of scope: plain list (no checkboxes) for things explicitly excluded (SSR, removed features, future tiers)

## Effort markers for use cases
- **quick fix** — one file, add match arm or call by analogy
- **moderate** — 2-3 files, existing infrastructure
- **needs infrastructure** — new analysis pass, AST type, or codegen module

## Lifecycle
1. Created by `/port` step 3 or `/audit` step 3
2. Updated after each session — Current state section
3. Complete when all use cases `[x]` → move feature to ROADMAP Done
4. Never deleted — remains as reference
