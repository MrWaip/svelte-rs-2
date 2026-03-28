---
name: oxc-analyze-api
description: OXC Visit/VisitMut and Scoping API reference. Auto-loaded when working on analyzer code.
user-invocable: false
paths:
  - "crates/svelte_analyze/**/*.rs"
---

# OXC API for Analyze

## Setup

Before any code changes or review, use the Read tool to load these references:

    Read .claude/skills/oxc-analyze-api/references/visit-methods.txt
    Read .claude/skills/oxc-analyze-api/references/scoping-api.txt

For reference resolution and write detection work, also load:

    Read .claude/skills/oxc-analyze-api/references/semantic-builder-api.txt

## Visitor methods

Always use the most specific `visit_*` method for your node type. Consult the visit-methods reference to find it.

```rust
// GOOD — specific visitor
fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
    // handle update expressions directly
}

// BAD — generic visitor with manual dispatch
fn visit_expression(&mut self, expr: &Expression<'a>) {
    if let Expression::UpdateExpression(upd) = expr {
        // ...
    }
}
```

## Scoping — common operations

Consult scoping-api.txt for the full API.

```rust
// Get binding by name in a scope
let symbol_id = scoping.get_binding(scope_id, "varName");

// Get symbol name by id
let name = scoping.symbol_name(symbol_id);

// Get symbol flags
let flags = scoping.symbol_flags(symbol_id);

// Get scope flags
let scope_flags = scoping.scope_flags(scope_id);
```

## Upstream sources (for manual refresh)

- **Visit**: https://raw.githubusercontent.com/oxc-project/oxc/refs/heads/main/crates/oxc_ast_visit/src/generated/visit.rs
- **Scoping**: https://raw.githubusercontent.com/oxc-project/oxc/refs/heads/main/crates/oxc_semantic/src/scoping.rs
- **SemanticBuilder**: https://raw.githubusercontent.com/oxc-project/oxc/refs/heads/main/crates/oxc_semantic/src/builder.rs
