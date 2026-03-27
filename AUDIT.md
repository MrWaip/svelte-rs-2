# Phase Boundary Audit Report

Generated: 2026-03-27

---

## Class 1: Full Re-parse in Codegen

### #1 — `parse_expression()` in Builder
- **Class**: 1
- **Complexity**: M
- **Where**:
  - `crates/svelte_codegen_client/src/builder.rs:905-915` (definition)
  - `crates/svelte_codegen_client/src/template/component.rs:274` (bind setter)
  - `crates/svelte_codegen_client/src/template/component.rs:284` (bind getter)
  - `crates/svelte_codegen_client/src/script/mod.rs:68` (prop defaults)
  - `crates/svelte_codegen_client/src/script/mod.rs:156` (prop defaults, fallback path)
  - `crates/svelte_codegen_client/src/lib.rs:149` (CE prop defaults)
- **Occurrence count**: 5 call sites
- **What is aggregated**: Codegen creates a new `oxc_parser::Parser` to parse JS expressions from text strings at runtime
- **Proposed type**: Pre-parse prop defaults and bind expressions during `svelte_parser::parse_with_js`, store in `ParsedExprs` keyed by span. For bind setters/getters, build AST directly via Builder methods instead of format+parse
- **Target layer**: parser

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
- **What is aggregated**: `OnDirectiveModifiers::from_modifiers()` iterates `Vec<String>` and matches `m.as_str()` against known modifier names
- **Proposed type**: Parse modifiers into `OnDirectiveModifiers` struct during parser phase, store as field on `OnDirectiveLegacy`
- **Target layer**: parser

### #4 — Shorthand attribute name via `source_text().trim()`
- **Class**: 2
- **Complexity**: S
- **Where**:
  - `crates/svelte_analyze/src/passes/element_flags.rs:112`
  - `crates/svelte_codegen_client/src/template/attributes.rs:487`
- **Occurrence count**: 2
- **What is aggregated**: Shorthand attribute name extracted from source text span and trimmed
- **Proposed type**: Add `name: &'a str` field to `Shorthand` attribute struct, populate during parsing
- **Target layer**: parser

### #5 — `strip_capture_event()` string slicing
- **Class**: 2
- **Complexity**: M
- **Where**:
  - `crates/svelte_analyze/src/utils/events.rs:39-44` (utility)
  - `crates/svelte_analyze/src/passes/element_flags.rs:58` (call site)
  - `crates/svelte_codegen_client/src/template/attributes.rs:92` (call site)
- **Occurrence count**: 2 call sites (analyze + codegen)
- **What is aggregated**: `&name[..name.len()-7]` to strip "capture" suffix, `name.ends_with("capture")` to detect
- **Proposed type**: Add `capture: bool` + `base_event_name: &str` fields to event attribute AST, populate during parsing
- **Target layer**: parser

### #6 — Each block collection name from source text
- **Class**: 2
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/each_block.rs:98`
- **Occurrence count**: 1
- **What is aggregated**: `source_text(expr_span).trim()` to get getter name for prop source
- **Proposed type**: Use pre-parsed/transformed expression from `ParsedExprs` instead of re-extracting from source
- **Target layer**: codegen refactor

---

## Class 3: AST Re-traversal in Codegen

### #7 — Bubble events detection
- **Class**: 3
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/lib.rs:202-211`
- **Occurrence count**: 1
- **What is aggregated**: `.iter().any()` over top-level fragment nodes checking for `OnDirectiveLegacy` with no expression on `<svelte:window>` / `<svelte:document>`
- **Proposed type**: `fn has_bubble_events(&self) -> bool` accessor on `AnalysisData`
- **Target layer**: analyze

### #8 — Bind:this double lookup
- **Class**: 3
- **Complexity**: S
- **Where**:
  - `crates/svelte_codegen_client/src/template/component.rs:206-208`
  - `crates/svelte_codegen_client/src/template/component.rs:217-219`
- **Occurrence count**: 2 lookups of same attribute in same function
- **What is aggregated**: `.iter().find_map()` over component attributes to find `BindDirective` by `NodeId`, done twice
- **Proposed type**: Indexed lookup table for bind directives in `ElementFlags`, or single lookup with local caching
- **Target layer**: analyze

### #9 — Sole static class detection on `<svelte:element>`
- **Class**: 3
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/svelte_element.rs:57-69`
- **Occurrence count**: 1
- **What is aggregated**: `attributes.len() == 1` + match on `StringAttribute` + name check `"class"` + source text extraction
- **Proposed type**: `fn sole_static_class(&self, id: NodeId) -> Option<&str>` on `ElementFlags`
- **Target layer**: analyze

### #10 — SVG namespace detection on `<svelte:element>`
- **Class**: 3
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/svelte_element.rs:40-47`
- **Occurrence count**: 1
- **What is aggregated**: `.iter().any()` over attributes checking `xmlns == "http://www.w3.org/2000/svg"`
- **Proposed type**: `fn is_svg_ns(&self, id: NodeId) -> bool` on `ElementFlags`
- **Target layer**: analyze

### #11 — Props declaration detection in codegen
- **Class**: 3
- **Complexity**: M
- **Where**:
  - `crates/svelte_codegen_client/src/script/props.rs:12-24` (`is_props_declaration`)
  - `crates/svelte_codegen_client/src/script/props.rs:27-40` (`is_props_id_declaration`)
- **Occurrence count**: 2 functions
- **What is aggregated**: `.iter().any()` over `VariableDeclaration` checking for `$props()` / `$props.id()` patterns — already detected during `extract_script_info` in analyze
- **Proposed type**: Use pre-computed data from `ComponentScoping` instead of re-detecting
- **Target layer**: analyze

### #12 — Transition "global" modifier check
- **Class**: 3
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/events.rs:103`
- **Occurrence count**: 1
- **What is aggregated**: `.iter().any(|m| m == "global")` on transition directive modifiers
- **Proposed type**: `has_global_modifier: bool` on `TransitionDirective` AST or side table
- **Target layer**: parser

### #13 — Custom element props deduplication
- **Class**: 3
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/custom_element.rs:105-127`
- **Occurrence count**: 1
- **What is aggregated**: `Vec<&str>` of CE config prop names iterated with `.iter().any()` in a loop
- **Proposed type**: Pre-compute CE prop name set as `FxHashSet` in analyze, or at minimum use `FxHashSet` locally
- **Target layer**: analyze

---

## Class 4: Derived Flags Without a Name

### #14 — Event modifier capture/passive argument routing
- **Class**: 4
- **Complexity**: M
- **Where**:
  - `crates/svelte_codegen_client/src/template/events.rs:202-207`
  - `crates/svelte_codegen_client/src/template/events.rs:405-410`
  - `crates/svelte_codegen_client/src/template/events.rs:442-447`
  - `crates/svelte_codegen_client/src/template/attributes.rs:121-126`
- **Occurrence count**: 4 across 2 files
- **What is aggregated**: `capture || passive.is_some()` to decide whether to push args, then individual flag values for arg content
- **Proposed type**: `enum EventModifierFlags { None, CaptureOnly(bool), CaptureAndPassive(bool, bool) }` in analyze
- **Target layer**: analyze

### #15 — Title element nullish coalesce decision
- **Class**: 4
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/title_element.rs:56-62`
- **Occurrence count**: 1
- **What is aggregated**: `has_state && items.first().is_some_and(|item| matches!(item, TextConcat { parts, .. } if parts.len() == 1 && matches!(parts[0], Expr(_))))`
- **Proposed type**: `needs_nullish_coalesce: bool` on `TitleElementData` or similar
- **Target layer**: analyze

### #16 — Dynamic/import attribute routing
- **Class**: 4
- **Complexity**: S
- **Where**: `crates/svelte_codegen_client/src/template/svelte_boundary.rs:104-108`
- **Occurrence count**: 1
- **What is aggregated**: `*is_dynamic || is_import` to decide `ObjProp::Getter` vs `ObjProp::KeyValue`
- **Proposed type**: `enum ComponentPropValueMode { Static, Dynamic, Import }` in analyze
- **Target layer**: analyze

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
| 2 — String Re-parsing | 4 | 2 | 2 | 0 |
| 3 — AST Re-traversal | 7 | 6 | 1 | 0 |
| 4 — Derived Flags | 3 | 2 | 1 | 0 |
| **Total** | **16** | **11** | **5** | **0** |

### Recommended migration order

1. **#14** (M) — Event modifier flags: 4 occurrences, highest repetition
2. **#1** (M) — `parse_expression()`: 5 call sites, most severe class
3. **#11** (M) — Props declaration re-detection: duplicates existing analyze work
4. **#3** (M) — Event modifier string parsing: related to #14
5. **#5** (M) — Capture event string slicing: related to #3
6. Remaining S-complexity items in any order
