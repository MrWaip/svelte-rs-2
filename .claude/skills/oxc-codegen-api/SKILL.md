---
name: oxc-codegen-api
description: OXC Traverse API reference and codegen constraints. Auto-loaded when working on codegen/transform code.
user-invocable: false
paths:
  - "crates/svelte_codegen_client/**/*.rs"
  - "crates/svelte_transform/**/*.rs"
---

# OXC API for Codegen

## Setup

Before any code changes or review, use the Read tool to load the reference:

    Read .claude/skills/oxc-codegen-api/references/traverse-methods.txt

This file contains all available OXC Traverse `enter_*/exit_*` visitor method signatures.

## Boundary rule

Codegen must NOT perform semantic analysis — no symbol resolution, no type inference, no scope queries. If you need this data, it must come from `AnalysisData`.

The `svelte_transform` crate follows the same boundary rules and Traverse API conventions as `svelte_codegen_client`.

## Visitor methods

Always use the most specific `enter_*/exit_*` method for your node type. Consult the reference file above to find it.

```rust
// GOOD — specific visitor
fn exit_update_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
    // handle update expressions directly
}

// BAD — generic visitor with manual dispatch
fn exit_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
    if let Expression::UpdateExpression(upd) = node {
        // ...
    }
}
```

## Upstream sources (for manual refresh)

- **Traverse**: https://raw.githubusercontent.com/oxc-project/oxc/refs/heads/main/crates/oxc_traverse/src/generated/traverse.rs
- **Visit**: https://raw.githubusercontent.com/oxc-project/oxc/refs/heads/main/crates/oxc_ast_visit/src/generated/visit.rs
