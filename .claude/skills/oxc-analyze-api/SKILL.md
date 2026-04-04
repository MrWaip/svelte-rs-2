---
name: oxc-analyze-api
description: MUST consult before writing or modifying any OXC Visit/VisitMut visitor or scoping code in svelte_analyze or svelte_component_semantics. Contains the exact method signatures for all visit_* methods and scoping API. Scope/symbol/reference infrastructure now lives in `svelte_component_semantics` (not oxc_semantic). Use this skill whenever implementing a new analysis visitor, adding scope/symbol resolution logic, working with reference detection, or unsure which visit_* method to use for a specific AST node type. Using wrong method signatures causes silent bugs.
paths:
  - "crates/svelte_analyze/**/*.rs"
  - "crates/svelte_component_semantics/**/*.rs"
---

# OXC API for Analyze

## Setup

Before any code changes or review, use the Read tool to load:

    Read .claude/skills/oxc-analyze-api/references/visit-methods.txt

**Note:** Scope/symbol/reference infrastructure now lives in `svelte_component_semantics`, not `oxc_semantic`. Read `crates/svelte_component_semantics/src/lib.rs` for the current API.

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

Scoping API is in `svelte_component_semantics::ComponentSemantics`. `ComponentScoping` in `svelte_analyze` Deref's to it.

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
