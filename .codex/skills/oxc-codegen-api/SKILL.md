---
name: oxc-codegen-api
description: OXC traversal API reference for `svelte_transform` and `svelte_codegen_client`. Use when implementing or reviewing `Traverse` visitors, when you need exact `enter_*` or `exit_*` method signatures, or when checking that codegen and transform logic stay out of semantic-analysis territory.
---

# OXC API For Codegen

Load this bundled reference before changing traversal code:

- `.codex/skills/oxc-codegen-api/references/traverse-methods.txt`

## Boundary rule

Transform and codegen must not perform semantic analysis. No ad hoc symbol resolution, type inference, or scope reconstruction in these crates. If you need semantic facts, get them from `AnalysisData`.

## Traversal rule

Use the most specific `enter_*` or `exit_*` hook for the node type.

```rust
fn exit_update_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
    // good: specific hook
}
```

Avoid generic hooks plus manual dispatch when a specific generated hook exists.

## Working rule

If you are unsure about the exact traversal signature, read `traverse-methods.txt` first instead of guessing from memory.
