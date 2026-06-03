---
name: required
description: Required reading before any compiler work — the project's essential context from docs/. Use at session/task start before touching the compiler, or whenever work hits a layer (parser/analyze/transform/codegen/ast/css/diagnostics) and you need its invariants.
allowed-tools: Read, Bash
---

# required

`docs/context.md` is the single entry point — dogmas, cross-cutting conventions, the live index of every other doc, and the glossary; `docs/map.md` has the crate layers and entry points. Read both at the start of any compiler work, then follow the index to the one root PRD (and any `docs/designs/*.md`) that matches your layer. Always follow the cross-links you hit instead of guessing, and trust the filesystem over memory if the index looks stale.
