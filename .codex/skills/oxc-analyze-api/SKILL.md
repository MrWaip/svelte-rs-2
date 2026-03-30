---
name: oxc-analyze-api
description: OXC visitor and semantic API reference for `svelte_analyze`. Use when writing or reviewing `Visit` or `VisitMut` code, scope or symbol-resolution logic, reference collection, write detection, or when you need exact visitor method signatures for a specific OXC AST node.
---

# OXC API For Analyze

Load the bundled references before changing visitor or scoping code:

- `.codex/skills/oxc-analyze-api/references/visit-methods.txt`
- `.codex/skills/oxc-analyze-api/references/scoping-api.txt`

Load this too for reference resolution, symbol tracking, or write detection:

- `.codex/skills/oxc-analyze-api/references/semantic-builder-api.txt`

## Visitor rule

Use the most specific visitor method for the node type you are handling.

```rust
fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
    // good: specific hook
}
```

Avoid generic `visit_expression` plus manual variant dispatch unless there is a strong reason the specific hook cannot express the behavior.

## Scoping rule

Consult `scoping-api.txt` for exact methods. Typical operations include:

- look up a binding by name inside a scope
- get symbol names or flags by `SymbolId`
- inspect scope flags

## Working rule

If you are unsure which method signature exists, read the reference file first instead of guessing. Wrong OXC signatures can compile badly or fail silently.
