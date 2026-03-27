# Phase Boundary Audit Report

Generated: 2026-03-27 (validated for false positives)

---

## Class 1: Full Re-parse in Codegen

### #1 — `parse_expression()` in Builder
- **Class**: 1
- **Complexity**: M
- **Where**:
  - `crates/svelte_codegen_client/src/builder.rs:905-915` (definition)
  - `crates/svelte_codegen_client/src/template/component.rs:284` (bind getter — re-parses source text from `source_text(span)`)
  - `crates/svelte_codegen_client/src/script/mod.rs:68` (prop defaults — re-parses `default_text` extracted from source in analyze)
  - `crates/svelte_codegen_client/src/script/mod.rs:156` (prop defaults, fallback path)
  - `crates/svelte_codegen_client/src/lib.rs:149` (CE prop defaults)
- **Occurrence count**: 4 call sites
- **What is aggregated**: Codegen creates a new `oxc_parser::Parser` to parse JS expressions from text strings at runtime. Analyze already has the parsed AST (e.g. `assign.right` in `script_info.rs:449`) but discards it and stores only text
- **Proposed type**: Pre-parse prop defaults and bind expressions during `svelte_parser::parse_with_js`, store in `ParsedExprs` keyed by span
- **Target layer**: parser

> **Validated**: bind setter at `component.rs:274` removed — it parses GENERATED text (`format!("{var_name} = $$value")`), not source. That is legitimate AST construction.

### #2 — Script re-parse fallback
- **Class**: 1
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/script/mod.rs:142`
- **Occurrence count**: 1
- **What is aggregated**: Full `oxc_parser::Parser::new()` on script block source in a fallback path (labeled "tests calling codegen without analysis")
- **Proposed type**: Remove fallback or require pre-parsed program always. `transform_module_script` re-parse is legitimate (separate entry point)
- **Target layer**: codegen refactor

---

## Class 2: String Re-parsing

### #3 — Event modifier string matching
- **Class**: 2
- **Complexity**: M
- **Where**:
  - `crates/svelte_ast/src/lib.rs:614-641` (definition)
  - `crates/svelte_codegen_client/src/template/events.rs:187` (call site)
  - `crates/svelte_codegen_client/src/template/events.rs:391` (call site)
- **Occurrence count**: 2 call sites in codegen
- **What is aggregated**: `OnDirectiveModifiers::from_modifiers()` iterates `Vec<String>` and matches `m.as_str()` against known modifier names to populate boolean fields
- **Proposed type**: Parse modifiers into `OnDirectiveModifiers` struct during parser phase, store as field on `OnDirectiveLegacy`
- **Target layer**: parser

### #4 — Shorthand attribute name via `source_text().trim()`
- **Class**: 2
- **Complexity**: S
- **Where**:
  - `crates/svelte_analyze/src/passes/element_flags.rs:112`
  - `crates/svelte_codegen_client/src/template/attributes.rs:487`
- **Occurrence count**: 2
- **What is aggregated**: Shorthand attribute name extracted from source text span and trimmed. The `Shorthand` struct only has `id` and `expression_span`, no `name` field
- **Proposed type**: Add `name: &'a str` field to `Shorthand` attribute struct, populate during parsing
- **Target layer**: parser

### #5 — Each block collection name from source text
- **Class**: 2
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/each_block.rs:98`
- **Occurrence count**: 1
- **What is aggregated**: `source_text(expr_span).trim()` to get getter name for prop source. The parsed expression IS available via `get_node_expr()` (used in the `!is_prop_source` branch at line 101) but ignored here
- **Proposed type**: Use pre-parsed/transformed expression from `ParsedExprs` instead of re-extracting from source
- **Target layer**: codegen refactor

---

## Class 3: AST Re-traversal in Codegen

### #6 — Bubble events detection
- **Class**: 3
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/lib.rs:202-211`
- **Occurrence count**: 1
- **What is aggregated**: `.iter().any()` over top-level fragment nodes checking for `OnDirectiveLegacy` with no expression on `<svelte:window>` / `<svelte:document>`. Used to determine function parameters
- **Proposed type**: `fn has_bubble_events(&self) -> bool` accessor on `AnalysisData`
- **Target layer**: analyze

### #7 — Sole static class detection on `<svelte:element>`
- **Class**: 3
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/svelte_element.rs:57-69`
- **Occurrence count**: 1
- **What is aggregated**: `attributes.len() == 1` + match on `StringAttribute` + name check `"class"` + source text extraction
- **Proposed type**: `fn sole_static_class(&self, id: NodeId) -> Option<&str>` on `ElementFlags`
- **Target layer**: analyze

### #8 — SVG namespace detection on `<svelte:element>`
- **Class**: 3
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/svelte_element.rs:40-47`
- **Occurrence count**: 1
- **What is aggregated**: `.iter().any()` over attributes checking `xmlns == "http://www.w3.org/2000/svg"`
- **Proposed type**: `fn is_svg_ns(&self, id: NodeId) -> bool` on `ElementFlags`
- **Target layer**: analyze

### #9 — Transition "global" modifier check
- **Class**: 3
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/events.rs:103`
- **Occurrence count**: 1
- **What is aggregated**: `.iter().any(|m| m == "global")` on transition directive modifiers
- **Proposed type**: `has_global_modifier: bool` on `TransitionDirective` AST or side table
- **Target layer**: parser

---

## Removed (False Positives)

The following findings from the initial scan were invalidated during validation:

| Original # | Finding | Reason for removal |
|---|---|---|
| #1 (partial) | Bind setter `component.rs:274` | Parses **generated** text (`"x = $$value"`), not source — legitimate AST construction |
| #5 | `strip_capture_event()` string slicing | Semantic classification already done in analyze → `EventHandlerMode`; codegen extracts base name for output argument |
| #8 | Bind:this double lookup | Two-phase pattern: 1st lookup consumes expression from side table, 2nd gets metadata — not duplication |
| #11 | Props declaration detection (`script/props.rs`) | Functions belong to script **transform** phase, not codegen — re-examining parsed OXC AST to transform declarations is appropriate |
| #13 | CE props deduplication | Output filtering/routing during generation, not data collection for classification |
| #14 | Event modifier capture/passive routing | Output **formatting** with pre-computed flags; patterns differ across locations (different variable sources) |
| #15 | Title element nullish coalesce | One-off check in single location on pre-computed data — no repetition |
| #16 | Dynamic/import attribute routing | Single occurrence (doesn't meet 2+ threshold); flags pre-computed by analyze |

---

## Good Examples

| Where | Why it's good |
|---|---|
| `svelte_codegen_client/src/template/key_block.rs` — `gen_key_block` | Takes `build_node_thunk` + `gen_fragment`, assembles `$.key()` — zero re-parsing or flag logic |
| `svelte_codegen_client/src/template/attributes.rs:101-128` — `EventHandlerMode` match | Consumes pre-computed `EventHandlerMode` enum from analyze via single accessor call |
| `svelte_codegen_client/src/template/render_tag.rs` — `gen_render_tag` | Uses pre-computed `RenderTagCalleeMode` enum (4 modes from 3 flags), clean match dispatch |
| `svelte_codegen_client/src/context.rs:220-245` — `ElementFlags` accessors | All element properties via O(1) lookups: `has_spread`, `class_needs_state`, etc. |
| `svelte_codegen_client/src/template/component.rs:26-127` — component props | Pre-classified `ComponentPropInfo` from analyze, codegen only iterates for output |
| `svelte_codegen_client/src/template/debug_tag.rs` — `emit_debug_tags` | Takes pre-transformed expressions from `ctx.parsed.debug_tag_exprs`, zero analysis logic |
| `svelte_transform/src/lib.rs` — expression transform | Walks pre-parsed expressions from `ParsedExprs`, mutates in-place via `VisitMut` — never re-parses |
| `svelte_analyze/src/types/data.rs:336-342` — `class_needs_state` | Derived flag (`class_attr_dynamic || has_dynamic_class_directives`) computed in analyze, not codegen |

---

## Summary

| Class | Count | S | M | L |
|---|---|---|---|---|
| 1 — Full Re-parse | 2 | 1 | 1 | 0 |
| 2 — String Re-parsing | 3 | 2 | 1 | 0 |
| 3 — AST Re-traversal | 4 | 4 | 0 | 0 |
| 4 — Derived Flags | 0 | 0 | 0 | 0 |
| **Total** | **9** | **7** | **2** | **0** |

### Recommended migration order

1. **#1** (M) — `parse_expression()`: 4 call sites, most severe class — pre-parse prop defaults in parser
2. **#3** (M) — Event modifier string parsing: 2 call sites — parse modifiers to struct in parser
3. **#6** (S) — Bubble events: add `has_bubble_events` bool to AnalysisData
4. **#7** (S) — Sole static class: add accessor to ElementFlags
5. **#8** (S) — SVG namespace: add accessor to ElementFlags
6. **#9** (S) — Transition global modifier: add bool to AST or side table
7. **#2** (S) — Script re-parse fallback: require pre-parsed program
8. **#4** (S) — Shorthand name: add `name` field to AST
9. **#5** (S) — Each block collection: use `ParsedExprs` instead of source text
