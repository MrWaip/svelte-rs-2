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

Build a single prioritized list of **runnable commands**. Each line = one command the user can copy-paste.

Priority order (top to bottom):
1. Quick-fix ignored tests → `/fix-test <name>`
2. Active specs (not complete) → `/port specs/<name>.md`
3. First unchecked ROADMAP item → `/audit <feature>` or `/port specs/<name>.md`
4. Tech debt TODOs → `/improve <description>`

For each item, format as:

```
/command argument — one-line description (effort hint if available)
```

Output the report:

```
## What to do next

1. `/fix-test <test-name>` — <bug description>
2. `/fix-test <test-name>` — <bug description>
3. `/port specs/<spec>.md` — <N remaining use cases, what's next>
4. `/audit <feature>` — <what and how many items>
5. `/improve <file-or-description>` — <what to fix>
...
```

Cap at 7 items. No sub-sections, no grouping headers — just a numbered list.

## Rules

- **Read-only** — never edit any files
- Prioritize: active specs first, then quick-fix ignored tests, then ROADMAP items
- Keep the report concise — max 3 items per section
