---
name: required
description: Required reading before any compiler work — the project's essential context from docs/. Use at session/task start before touching the compiler, or whenever work hits a layer (parser/analyze/transform/codegen/ast/css/diagnostics) and you need its invariants.
allowed-tools: Read, Bash
---

# required

Start by listing every doc:

```
tree -P '*.md' --prune docs
```

Then read `docs/context.md` and `docs/map.md`, and any doc from the tree that relates to the task. Read more as you go if the task changes.
