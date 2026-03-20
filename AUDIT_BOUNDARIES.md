# Audit: Phase Boundary Violations in Codegen & Transform

**Date:** 2026-03-20
**Scope:** `crates/svelte_codegen_client/`, `crates/svelte_transform/src/`

---

## Class 1: Full Re-parse in Codegen

### 1.1 Custom Element Config Parsing

- **Pattern**: `parse_ce_expression` — re-parses custom element config with OXC
- **Class**: 1
- **Where**: `crates/svelte_codegen_client/src/custom_element.rs:39-141`
- **Occurrence count**: 1
- **What is aggregated**: Custom element config object (`{ tag, shadow, props, extend }`) extracted from `CustomElementConfig::Expression` span by creating a new `oxc_allocator::Allocator` + `oxc_parser::Parser::new()` at codegen time
- **Proposed type**: Pre-parse in `parse_js` pass; store structured `ParsedCeConfig { tag, shadow, props, extend_expr }` in `ParsedExprs` or `AnalysisData`
- **Target layer**: analyze

### 1.2 Builder::parse_expression

- **Pattern**: Generic re-parse fallback in builder
- **Class**: 1
- **Where**: `crates/svelte_codegen_client/src/builder.rs:914-924`, called from `custom_element.rs:192`
- **Occurrence count**: 2 call sites
- **What is aggregated**: JS expressions stored as string slices (e.g., `extend_source` from CE config) re-parsed to OXC `Expression`
- **Proposed type**: Extract `extend` field as pre-parsed `Expression<'a>` during CE config parsing
- **Target layer**: analyze

### 1.3 Script Codegen parse_expression

- **Pattern**: Expression re-parsing helper in script transform
- **Class**: 1
- **Where**: `crates/svelte_codegen_client/src/script.rs:1192-1204`
- **Occurrence count**: 1
- **What is aggregated**: Defensive fallback for expressions that should have been pre-parsed
- **Proposed type**: Audit callers; construct AST directly via Builder instead of string→parse round-trip
- **Target layer**: analyze / codegen refactor

> **Root cause summary:** All three Class 1 violations stem from custom element config not being parsed during the parser→analyze pipeline. Fixing CE config parsing eliminates violations 1.1 and 1.2; 1.3 needs caller audit.

---

## Class 2: String Re-parsing

### 2.1 Await Block Destructured Binding

- **Pattern**: `gen_destructured_callback` — determines array/object destructuring via string inspection
- **Class**: 2
- **Where**: `crates/svelte_codegen_client/src/template/await_block.rs:88-120`
- **Occurrence count**: 1
- **What is aggregated**: Destructure kind (`[` vs `{`), binding names, aliases, defaults — all extracted from source text via `starts_with('[')`, `split(',')`, `split('=')`, `split_once(':')`, `trim()`, `&pattern[1..pattern.len()-1]`
- **Proposed type**: Parser should produce structured AST:
  ```
  enum AwaitBindingPattern {
      Simple(Atom),
      Destructured {
          kind: DestructureKind, // Array | Object
          bindings: Vec<DestructuredBinding>,
      },
  }
  struct DestructuredBinding { name: Atom, alias: Option<Atom>, default_value: Option<Span> }
  ```
- **Target layer**: parser

### 2.2 Directive Name Dot-Splitting

- **Pattern**: `build_directive_name_expr` — splits `"a.b.c"` into member expression chain
- **Class**: 2
- **Where**: `crates/svelte_codegen_client/src/template/attributes.rs:1302-1310`
- **Occurrence count**: 1
- **What is aggregated**: Dotted directive names split via `name.split('.')` to build `a.b.c` member expressions
- **Proposed type**: Parser could store directive name as `Vec<Atom>` parts, or codegen can construct member expr from the dotted string (this is borderline — the string IS the canonical representation for directive names, and the split is trivial)
- **Target layer**: parser (low priority — borderline)

### 2.3 Store Name `$` Prefix Stripping

- **Pattern**: `&id_name[1..]` — strips leading `$` from store variable names
- **Class**: 2 (borderline)
- **Where**: `crates/svelte_codegen_client/src/script.rs:2103, 2156, 2193, 2222`
- **Occurrence count**: 4
- **What is aggregated**: Base name of store variables (stripping `$` prefix)
- **Proposed type**: Already borderline — `$` prefix is a naming convention. Could add `base_name()` accessor to binding info, but this is low-severity since the transformation is trivial and well-understood
- **Target layer**: analyze (low priority)

---

## Class 3: AST Re-traversal in Codegen

### 3.1 UseDirective Existence Check

- **Pattern**: `.iter().any(|a| matches!(a, Attribute::UseDirective(_)))`
- **Class**: 3
- **Where**: `crates/svelte_codegen_client/src/template/element.rs:55`, `crates/svelte_codegen_client/src/template/attributes.rs:1018`
- **Occurrence count**: 2
- **What is aggregated**: Boolean: "does this element have a use:action directive?"
- **Proposed type**: `fn has_use_directive(id: NodeId) -> bool` accessor on `ElementFlags`
- **Target layer**: analyze

### 3.2 SVG xmlns Attribute Check

- **Pattern**: `.iter().any(|attr| xmlns == "http://www.w3.org/2000/svg")`
- **Class**: 3
- **Where**: `crates/svelte_codegen_client/src/template/svelte_element.rs:40`
- **Occurrence count**: 1
- **What is aggregated**: Boolean: "does `<svelte:element>` have xmlns pointing to SVG namespace?"
- **Proposed type**: `fn is_svg_namespace(id: NodeId) -> bool` on `ElementFlags`
- **Target layer**: analyze

### 3.3 Animate Directive in Each Block Children

- **Pattern**: Nested `.iter().any()` over children and their attributes
- **Class**: 3
- **Where**: `crates/svelte_codegen_client/src/template/each_block.rs:49-51`
- **Occurrence count**: 1 (double-nested iteration)
- **What is aggregated**: Boolean: "does any child element have an animate directive?"
- **Proposed type**: `fn has_animated_children(block_id: NodeId) -> bool` on `EachBlockData`
- **Target layer**: analyze

### 3.4 Class Directive Collection

- **Pattern**: `.iter().filter_map(|a| ClassDirective)` + `.iter().any(|a| ClassDirective && dynamic)`
- **Class**: 3
- **Where**: `crates/svelte_codegen_client/src/template/attributes.rs:239-243, 266-268`
- **Occurrence count**: 2 (filter_map + any on same list)
- **What is aggregated**: All class directives + whether any is dynamic
- **Proposed type**: Add `class_directive_ids(id: NodeId) -> &[NodeId]` and `has_dynamic_class_directives(id: NodeId) -> bool` to `ElementFlags`
- **Target layer**: analyze

### 3.5 Style Directive Collection

- **Pattern**: `.iter().filter_map(|a| StyleDirective)`
- **Class**: 3
- **Where**: `crates/svelte_codegen_client/src/template/attributes.rs:336-343, 1039-1043`
- **Occurrence count**: 2
- **What is aggregated**: All style directives from element attributes
- **Proposed type**: Add `style_directive_ids(id: NodeId) -> &[NodeId]` to `ElementFlags`
- **Target layer**: analyze

### 3.6 Attribute Lookup by ID in Component

- **Pattern**: `.find(|a| a.id() == attr_id)` — re-searching attribute list after collecting info
- **Class**: 3
- **Where**: `crates/svelte_codegen_client/src/template/component.rs:127, 140`
- **Occurrence count**: 2
- **What is aggregated**: Looking up a specific attribute by ID from the component's attribute list after having already iterated it
- **Proposed type**: Restructure `gen_component` to collect all needed data in a single pass, or add `attr_by_id` index
- **Target layer**: codegen refactor

### 3.7 Bubble Events Check (Legacy)

- **Pattern**: Nested `.iter().any()` over root fragment children → attributes → `OnDirectiveLegacy` without expression
- **Class**: 3
- **Where**: `crates/svelte_codegen_client/src/lib.rs:191-200`
- **Occurrence count**: 1 (nested double iteration)
- **What is aggregated**: Boolean: "does any SvelteWindow/SvelteDocument have a bubble event (on:event with no handler)?"
- **Proposed type**: `fn has_bubble_events() -> bool` at component level in `AnalysisData`
- **Target layer**: analyze

---

## Class 4: Derived Flags Without a Name

### 4.1 Expression Memoization (Duplicated)

- **Pattern**: `has_call && references.iter().any(|r| r.symbol_id.is_some())`
- **Class**: 4
- **Where**: `crates/svelte_codegen_client/src/template/if_block.rs:87-89`, `crates/svelte_codegen_client/src/template/expression.rs:226-242`
- **Occurrence count**: 2 (same logic in both locations)
- **What is aggregated**: `has_call` (expression contains function call) + `has_resolved_refs` (any reference resolves to a binding)
- **Proposed type**: `fn needs_expr_memoization(id: NodeId) -> bool` accessor in `AnalysisData`, computed during reactivity visitor pass
- **Target layer**: analyze
- **Priority**: HIGH — same logic duplicated across two files

### 4.2 Component Attribute Memoization

- **Pattern**: `has_call || (is_non_simple && is_dynamic)`
- **Class**: 4
- **Where**: `crates/svelte_codegen_client/src/template/component.rs:100`
- **Occurrence count**: 1
- **What is aggregated**: `has_call` + `is_non_simple` (expression kind) + `is_dynamic` (dynamic attribute flag) → determines whether to wrap in `$.derived()`
- **Proposed type**: `fn component_attr_needs_memo(attr_id: NodeId) -> bool` or extend `ExpressionInfo` with `needs_memoization` field
- **Target layer**: analyze
- **Priority**: HIGH — memoization decision belongs in analyze alongside `has_call` computation

---

## Good Examples

| Where | Why it's good |
|---|---|
| `template/expression.rs:get_node_expr` | Consumes pre-parsed expression via `ctx.parsed.exprs.remove()` — ownership transfer, no re-parsing |
| `template/attributes.rs:650-655` | Looks up pre-parsed attribute expressions from `ctx.parsed.attr_exprs` |
| `script.rs:48-60` | Takes pre-parsed `Program` from `ctx.parsed.script_program.take()` |
| `template/svelte_boundary.rs:138-140` | Borrows pre-parsed expressions from `ctx.parsed.exprs.get()`, clones only when needed |
| `template/if_block.rs:46-54` | Uses `ctx.is_elseif_alt()` — pre-computed accessor from `AnalysisData`, no AST inspection |
| `template/mod.rs:emit_content_strategy` | Matches on pre-computed `ContentStrategy` enum from analyze — the ideal pattern for multi-way decisions |
| `template/element.rs:50` | Uses `ctx.has_spread(el_id)` — accessor from `ElementFlags`, no iteration |

---

## Summary

| Class | Count | Severity | Fix effort |
|---|---|---|---|
| **1 — Full Re-parse** | 3 | Critical | Medium (CE config parsing pipeline) |
| **2 — String Re-parsing** | 3 (1 significant) | High | Medium (await binding AST restructure) |
| **3 — AST Re-traversal** | 7 | Medium | Low (add accessors/flags to analyze) |
| **4 — Derived Flags** | 2 | Medium | Low (add accessor methods) |
| **Total** | **15** | | |

### Recommended Fix Priority

1. **Class 2.1** — Await block destructured binding (highest bang-for-buck: eliminates most string parsing)
2. **Class 1.1 + 1.2** — Custom element config parsing (eliminates all 3 Class 1 violations)
3. **Class 4.1** — Expression memoization (duplicated logic, easy to centralize)
4. **Class 3.1 + 3.4 + 3.5** — Directive collection (3 related violations in attributes.rs, batch-fixable)
5. **Class 3.3** — Animated children (nested double iteration)
6. Remaining Class 3 + 4 items (individual accessor additions)
