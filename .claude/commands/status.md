---
description: Project status overview — active specs, ignored tests, ROADMAP next items, known debt. Use at session start or when deciding what to work on next.
allowed-tools: Bash, Read, Grep, Glob
---

# Project Status

## Step 1: Active specs

Run `Glob("specs/*.md")`. For each spec, read the **Current state** section (first section after title). List:

```
- <spec name> — <status summary> — next: <action>
```

Skip specs where Current state = "Complete".

## Step 2: Ignored tests

```
grep -c '#\[ignore' tasks/compiler_tests/test_v3.rs
```

If count > 0, extract the ignore reasons:
```
grep '#\[ignore' tasks/compiler_tests/test_v3.rs
```

Group by reason pattern (missing/bug) and layer. Classify by effort:
- **Quick fix** — reason mentions one file or "by analogy"
- **Moderate** — reason mentions 2-3 files, existing infrastructure
- **Infrastructure** — reason mentions new pass, new type, new module

## Step 3: ROADMAP next items

Read `ROADMAP.md`. Find the first unchecked `[ ]` item in the lowest active tier (Tier 1.1 → Tier 2 → Tier 3 → ...).

## Step 4: Known debt

Read `ROADMAP.md` Deferred section.

```
grep -rn 'TODO' crates/ --include='*.rs' | head -20
```

## Step 5: Report

```
## Project Status

### Active specs
- [spec] — [status] — next: `/port specs/<name>.md`

### Ignored tests (N total)
- quick fix (M): `test1`, `test2` → `/fix-test <name>`
- moderate (K): `test3` → `/fix-test <name>`
- infrastructure (L): `test4` → `/port specs/<name>.md`

### ROADMAP next
- [tier] [item] → `/audit <feature>` or `/port`

### Known debt (N items)
- [item] → `/improve <description>`

### Suggested next action
→ [one concrete command based on: active spec > quick-fix test > ROADMAP next > debt]
```

## Rules

- **Read-only** — never edit any files
- Prioritize: active specs first, then quick-fix ignored tests, then ROADMAP items
- Keep the report concise — max 3 items per section
