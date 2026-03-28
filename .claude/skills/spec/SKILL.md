# Spec: $ARGUMENTS

Create a spec file that decomposes a ROADMAP item into atomic, layer-scoped tasks.

## Purpose

Every implementation task MUST have a spec file in `specs/` before any code is written.
The spec captures: what the expected output looks like, what tasks are needed per layer,
and what progress has been made. It persists between sessions.

## Inputs

`$ARGUMENTS` is a feature description matching a ROADMAP item (e.g., `snippet param destructuring`, `muted attribute`, `css hash`).

## Execution

### Step 1: Find the ROADMAP item

Read `ROADMAP.md`, find the matching item(s). Extract:
- Tier and section
- Checkbox items (done/pending)
- Reference file pointers (if listed)

If the argument doesn't match any ROADMAP item, ask the user which item they mean.

### Step 2: Research (parallel agents)

Launch 2 Explore agents simultaneously:

1. **Agent 1 — Reference compiler**: trace the feature through `reference/compiler/` phases. Extract:
   - What runtime `$.helper()` calls are produced
   - What analysis metadata / flags are needed
   - Edge cases (if/switch branches in reference code)
   - Expected JS output shape

2. **Agent 2 — Our codebase**: check what's already implemented for this feature in:
   - `crates/svelte_ast/src/lib.rs` — AST types
   - `crates/svelte_analyze/src/` — analysis passes
   - `crates/svelte_codegen_client/src/` — codegen
   - `crates/svelte_parser/src/` — parser
   - `tasks/compiler_tests/cases2/` — existing test cases

### Step 3: Decompose into tasks

Break the feature into atomic tasks. Each task:
- Touches **one layer** (parser, ast, analyze, transform, codegen, test)
- Is **one commit**
- Has a clear **done condition**
- Lists **files to modify**

Order: test cases → parser/ast → analyze → transform → codegen → verify.

Tests come first: create `case.svelte` + generate `case-svelte.js` before implementing.
This establishes the expected output as ground truth.

**Layer rules** (from CLAUDE.md):
- **parser**: structured data, not Span-only. If JS is involved, parse via `walk_js`
- **analyze**: all classification, flags, precomputation. Codegen should not dig deeper than one accessor call
- **codegen**: flat mapper. Match on enums, format output. No decision logic

For each task, explicitly state which layer it belongs to and why.

### Step 4: Write spec file

Write `specs/<feature-slug>.md` with this format:

```markdown
# <Feature name>

**Source**: ROADMAP Tier N — section name
**Status**: not started

## Expected output

(JS output examples from reference compiler research — 1-2 representative cases)

## Tasks

- [ ] 1. **test**: create test cases `<names>` + `just generate`
  - Files: `tasks/compiler_tests/cases2/<name>/case.svelte`, `test_v3.rs`
  - Done: `case-svelte.js` generated, matches expected output

- [ ] 2. **parser**: <what changes>
  - Files: `crates/svelte_parser/src/<file>`
  - Done: parser unit tests pass

- [ ] 3. **analyze**: <what classification/flags/accessors>
  - Files: `crates/svelte_analyze/src/<file>`
  - Done: analyze unit tests pass

- [ ] 4. **transform**: <what rewrites if any>
  - Files: `crates/svelte_transform/src/<file>`
  - Done: N/A or transform tests pass

- [ ] 5. **codegen**: <what output generation>
  - Files: `crates/svelte_codegen_client/src/<file>`
  - Done: `just test-case <name>` passes for all test cases

- [ ] 6. **verify**: `just test-compiler` passes, update ROADMAP

## Edge cases

(from reference research — things that might be missed)

## Progress

(updated after each completed task)
```

Adjust the number and content of tasks based on what the feature actually needs.
Small features (single attribute, one flag) may have 3-4 tasks.
Large features (new block type, CSS pipeline) may have 8-10+.

Skip layers that aren't needed (e.g., no transform task if no rune rewrites).

### Step 5: Present to user

Show:
- Summary of what was found
- The task list (numbered, with layers)
- Any blockers or dependencies on other features
- Ask for approval before writing the file

After approval, write the spec file and confirm the path.

## Rules

- **Read-only until Step 5** — no code changes, only research and spec writing
- Do not implement anything — this skill only produces the spec
- If a spec already exists for this feature, read it, show current status, and ask if the user wants to update it
- The spec IS the task list — no separate todo tracking needed
