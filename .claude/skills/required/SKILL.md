---
name: required
description: Required reading before any compiler work — the project's essential context from docs/. Use at session/task start before touching the compiler, or whenever work hits a layer (parser/analyze/transform/codegen/ast/css/diagnostics) and you need its invariants.
allowed-tools: Read, Bash
---

# required

Read before touching the compiler.

`context.md` indexes every doc with a one-line **purpose**. `map.md` indexes the code.

**Decide a doc's relevance from its purpose line, never from its filename.** Names undersell scope: a narrow-sounding name is often a foundation used across layers. If a purpose line overlaps your task at all — or you're unsure — read the whole doc. A skip is a decision you must be able to justify from the purpose, not a silent default.

Process:

1. Read `context.md` and `map.md` in full.
2. Walk the doc index in `context.md`. Read every entry whose purpose touches your task; skip one only when its purpose is clearly disjoint.
3. Read the touched layer's root PRD; designs under `docs/designs/*` when scope overlaps.
4. Re-walk the index when the task shifts — keep reading as scope grows.

The recurring failure: see the index, judge by the name, call it "not relevant," proceed to wrong conclusions. The purpose line exists precisely so you don't guess from the name — read it before you skip.
