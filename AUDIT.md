# Phase Boundary Audit

Generated: 2026-03-21

---

## Class 1: Full Re-parse in Codegen

### #1 — Each block destructuring default re-parse

- **Pattern**: destructuring default value re-parse ✅
- **Class**: 1
- **Complexity**: M
- **Where**:
  - `crates/svelte_codegen_client/src/template/each_block.rs:264-320` (`extract_default_value` + `parse_inline_expr`)
  - `crates/svelte_codegen_client/src/template/each_block.rs:185-241` (caller: `gen_each_block`)
- **Occurrence count**: 1 (single call site, but covers all each blocks with destructuring defaults)
- **What is aggregated**: `starts_with('[')` / `starts_with('{')` syntax detection, `find('=')` default extraction, `rfind(':')` alias extraction, depth-tracking comma split, then `oxc_parser::Parser::new()` to re-parse the extracted default value string
- **Proposed type**: `parse_js` phase walks the OXC `BindingPattern` AST of the each context expression and extracts structured data into a side table:
  ```rust
  struct EachContextInfo {
      bindings_with_defaults: Vec<(CompactStr, ExprId)>,
      // ExprId → pre-parsed default value expression in ParsedExprs
  }
  ```
  OXC already parses destructuring — `parse_js` just needs to walk `BindingPatternKind::{Object,Array}` and collect `BindingRestElement`/`AssignmentPattern` nodes with their `.right` default expressions. Codegen retrieves ready expressions, no string ops.
- **Target layer**: analyze (parse_js pass — walk OXC destructuring AST, store defaults in `ParsedExprs`)

### ~~#2 — Bind directive setter/getter construction~~ ✅ PARTIALLY MIGRATED

- **Pattern**: bind setter/getter format-and-reparse
- **Class**: 1
- **Complexity**: M
- **Where**:
  - `crates/svelte_codegen_client/src/template/component.rs`
- **Migration**: Simplified `AttrKind::BindThis` from 4 fields to 1 (`bind_id`). Expression text extracted on-demand from AST instead of pre-collected. Identifier path (simple `bind:this={name}`) fully uses pre-computed analysis (`is_mutable_rune_target`). Member expression path (`bind:this={obj.prop}`) still re-parses from source — unavoidable because reactive transforms (`$.get(i)`) conflict with bind:this parameter shadowing.
- **Remaining**: member expression setter/getter still uses `parse_expression` (2 calls). Full elimination requires storing raw (pre-transform) expressions separately in `ParsedExprs`.

### ~~#3 — Props default value fallback re-parse~~ ✅ MIGRATED

- **Pattern**: prop default fallback re-parse
- **Class**: 1
- **Complexity**: S
- **Where**:
  - `crates/svelte_codegen_client/src/script.rs` (`transform_script_text`)
- **Migration**: `transform_script_text` now pre-parses all prop defaults via `Builder::parse_expression` before creating `ScriptTransformer`, mirroring what `parse_js` does for the full pipeline path. Fallback removed — all paths now use pre-parsed expressions.

---

## Class 2: String Re-parsing

### ~~#4 — Directive name dot-notation splitting~~ ✅

~~Pre-parsed directive name expressions via OXC in `parse_js`. Stored in `ParsedExprs::directive_name_exprs`, consumed by codegen via `remove()`. `build_directive_name_expr` deleted.~~

### ~~#5 — Each block key_is_item detection~~ ✅

~~Stored as `EachBlockData::key_is_item: FxHashSet<NodeId>` in scope building (already computed for `mark_each_non_reactive`). Codegen reads via `ctx.each_key_is_item(block_id)`. Local `is_simple_identifier` in `each_block.rs` deleted.~~

### #6 — Expression text shorthand detection ✅

- **Status**: DONE — `expression_shorthand` flag in `ElementFlags`, computed in `parse_js`, 5 codegen occurrences replaced
- **Pattern**: shorthand detection via trimmed source text comparison
- **Class**: 2
- **Complexity**: M
- **Remaining `source_text()` uses** (NOT shorthand detection — identifier name extraction, out of scope):
  - `attributes.rs:1011` — `Shorthand` attribute name extraction
  - `component.rs:137` — Shorthand attribute name extraction for component props
  - `component.rs:189` — bind:this variable name extraction
  - ~~`each_block.rs:42-43` — key_is_item check (finding #5)~~ ✅ migrated
  - `each_block.rs:105` — prop source identifier name
  - `each_block.rs:186` — context pattern text

### ~~#7 — Prop name $$ prefix check~~ ✅

- ~~**Pattern**: reserved prop name prefix check~~
- ~~**Class**: 2~~
- ~~**Complexity**: S~~
- ~~**Where**:~~
  - ~~`crates/svelte_codegen_client/src/lib.rs:126` (`prop.prop_name.starts_with("$$")`)~~
  - ~~`crates/svelte_codegen_client/src/custom_element.rs:119` (same)~~
- ~~**Occurrence count**: 2~~
- ~~**What is aggregated**: `starts_with("$$")` string check on prop name~~
- ~~**Proposed type**: accessor `fn is_reserved_prop(&self) -> bool` on `PropAnalysis`, or filter reserved props out in analyze~~
- ~~**Target layer**: analyze~~
- **Resolution**: `is_reserved: bool` field added to `PropAnalysis`, computed in `analyze_props()`

### ~~#8 — HTML video tag detection~~ ✅

- ~~**Pattern**: `<video>` substring search in template HTML~~
- ~~**Class**: 2~~
- ~~**Complexity**: S~~
- ~~**Where**:~~
  - ~~`crates/svelte_codegen_client/src/template/mod.rs:90-92` (`html.contains("<video")`)~~
- ~~**Occurrence count**: 1~~
- ~~**What is aggregated**: searches generated HTML string for `<video` to determine `importNode` flag~~
- ~~**Proposed type**: accessor `fn needs_import_node(&self, fragment_id) -> bool` pre-computed during codegen template assembly or analyze~~
- ~~**Target layer**: codegen refactor (track during template HTML assembly, not post-hoc string search)~~
- **Resolution**: `element_html()` and `fragment_html()` return `(String, bool)` — the flag is set when `el.name == "video"` and OR-propagated upward through recursive calls. `needs_import_node()` deleted.

---

## Class 3: AST Re-traversal in Codegen

### #9 — Class attribute + class directives collection

- **Pattern**: class attr lookup + directive collection double traversal
- **Class**: 3
- **Complexity**: M
- **Where**:
  - `crates/svelte_codegen_client/src/template/attributes.rs:220` (`.find()` for class attr)
  - `crates/svelte_codegen_client/src/template/attributes.rs:240` (`.filter_map()` for class directives)
  - `crates/svelte_codegen_client/src/template/attributes.rs:1102` (`.filter_map()` duplicate for svelte:element)
- **Occurrence count**: 3 (2 collection passes + 1 duplicate in svelte:element path)
- **What is aggregated**: `el.attributes` traversed to find class expression attribute, then again to collect class directive NodeIds
- **Proposed type**: `ClassOutputInfo { class_attr_id: Option<NodeId>, directive_ids: Vec<NodeId> }` stored per element in analyze
- **Target layer**: analyze
- **Note**: `has_class_attribute()`, `has_class_directives()`, `has_dynamic_class_directives()` accessors already exist but only return bools — the actual NodeIds still require re-traversal

### #10 — Component attribute lookup re-traversal

- **Pattern**: component attribute re-lookup by NodeId
- **Class**: 3
- **Complexity**: S
- **Where**:
  - `crates/svelte_codegen_client/src/template/component.rs:122` (`.find(|a| a.id() == attr_id)`)
  - `crates/svelte_codegen_client/src/template/component.rs:135` (`.find(|a| a.id() == attr_id)`)
- **Occurrence count**: 2
- **What is aggregated**: after collecting AttrKind metadata in first pass, re-traverses `cn.attributes` to look up the same attribute by ID
- **Proposed type**: store needed attribute data (parts, expression_span) directly in `AttrKind` enum during first pass — codegen-internal refactor
- **Target layer**: codegen refactor

### #11 — Style directives in spread context

- **Pattern**: style directive extraction in spread path
- **Class**: 3
- **Complexity**: S
- **Where**:
  - `crates/svelte_codegen_client/src/template/attributes.rs:1037` (`.filter_map()`)
- **Occurrence count**: 1
- **What is aggregated**: `el.attributes` traversed to collect style directives during spread processing
- **Proposed type**: accessor `fn style_directive_ids(&self, element_id) -> &[NodeId]` in analyze
- **Target layer**: analyze

### #12 — Each block animate directive detection

- **Pattern**: nested traversal for animate directive existence
- **Class**: 3
- **Complexity**: S
- **Where**:
  - `crates/svelte_codegen_client/src/template/each_block.rs:74-76` (`nodes.iter().any()` + `el.attributes.iter().any()`)
- **Occurrence count**: 1
- **What is aggregated**: nested iteration over each body nodes → element attributes to check for AnimateDirective
- **Proposed type**: accessor `fn has_animate_in_body(&self, each_block_id: NodeId) -> bool` in analyze
- **Target layer**: analyze

### #13 — OnDirective modifiers repeated traversal

- **Pattern**: on:directive modifiers multi-pass
- **Class**: 3
- **Complexity**: M
- **Where**:
  - `crates/svelte_codegen_client/src/template/attributes.rs:1351-1359` (`.any()` + `.find_map()`)
  - `crates/svelte_codegen_client/src/template/attributes.rs:1571-1578` (duplicate pattern)
- **Occurrence count**: 2 (same pattern in `emit_on_directive_legacy` and `emit_on_directive_element`)
- **What is aggregated**: `od.modifiers` iterated 4+ times per directive for: prevent_default, stop_propagation, self, trusted, once, capture, passive/nonpassive
- **Proposed type**:
  ```rust
  struct OnDirectiveModifiers {
      has_prevent_default: bool,
      has_stop_propagation: bool,
      has_self: bool,
      has_trusted: bool,
      has_once: bool,
      has_capture: bool,
      passive: Option<bool>, // Some(true)=passive, Some(false)=nonpassive, None=unset
  }
  ```
- **Target layer**: analyze (pre-compute per directive during analysis pass)

---

## Class 4: Derived Flags Without a Name

### #14 — Event handler delegation routing

- **Pattern**: event delegation vs direct binding decision
- **Class**: 4
- **Complexity**: M
- **Where**:
  - `crates/svelte_codegen_client/src/template/attributes.rs:95-98`
  - `crates/svelte_codegen_client/src/template/attributes.rs:1358-1375`
  - `crates/svelte_codegen_client/src/template/attributes.rs:1609-1627`
- **Occurrence count**: 3
- **What is aggregated**: `!capture && is_delegatable_event(name)` → delegated vs direct, then nested `capture || passive` for arg building
- **Proposed type**:
  ```rust
  enum EventHandlerMode {
      Delegated { passive: bool },
      Direct { capture: bool, passive: bool },
  }
  ```
- **Target layer**: analyze

### #15 — Class directive dynamic state

- **Pattern**: class attr dynamic + directives dynamic combined decision
- **Class**: 4
- **Complexity**: S
- **Where**:
  - `crates/svelte_codegen_client/src/template/attributes.rs:266-314`
- **Occurrence count**: 1
- **What is aggregated**: `class_attr_is_dynamic || directives_are_dynamic` → `has_state`, then 4-way branching on has_directives x has_state
- **Proposed type**: accessor `fn class_needs_state(&self, element_id: NodeId) -> bool` in analyze
- **Target layer**: analyze

### #16 — Render tag callee routing (borderline)

- **Pattern**: render tag callee mode decision
- **Class**: 4
- **Complexity**: S
- **Where**:
  - `crates/svelte_codegen_client/src/template/render_tag.rs:24-110`
- **Occurrence count**: 1
- **What is aggregated**: `is_dynamic`, `is_chain`, `callee_is_getter` — three separate accessor calls
- **Proposed type**: enum `RenderTagMode { Dynamic, Chain, Direct }` — but single use site, low priority
- **Target layer**: analyze (low priority)

### #17 — Boundary attribute is_import deep chain (borderline)

- **Pattern**: boundary attr import symbol resolution
- **Class**: 4
- **Complexity**: S
- **Where**:
  - `crates/svelte_codegen_client/src/template/svelte_boundary.rs:100-111`
- **Occurrence count**: 1
- **What is aggregated**: `ctx.analysis.attr_expression(id).and_then(|info| info.references.first()).and_then(|r| r.symbol_id).is_some_and(|sym| ctx.is_import_sym(sym))` — deep chaining into AnalysisData
- **Proposed type**: accessor `fn attr_is_import(&self, attr_id: NodeId) -> bool`
- **Target layer**: analyze (low priority)

---

## Already Migrated

~~#3.1 — use directive existence check via `has_use_directive()` accessor~~

~~#3.4 — class directive dynamic check via `has_dynamic_class_directives()` accessor~~

~~#4.1 — expression memoization via `needs_expr_memoization()` accessor~~

~~#4.2 — component attribute memoization via `component_attr_needs_memo()` accessor~~

---

## Good Examples

| Where | Why it's good |
|---|---|
| `template/expression.rs:gen_expression_tag` | Retrieves pre-parsed expression via `ctx.parsed.exprs.remove(&id)` — zero re-parsing |
| `template/debug_tag.rs:emit_debug_tags` | Uses `ctx.parsed.debug_tag_exprs.remove(&(id, i))` — pre-parsed, no analysis logic |
| `template/each_block.rs:gen_each_block` (key expr) | Uses `ctx.parsed.key_exprs.remove(&block_id)` — pre-parsed key expression |
| `template/if_block.rs:gen_if_block` (memo) | Uses single `needs_expr_memoization()` accessor — flat match, no flag combination |
| `template/element.rs:has_class_attribute` | Boolean accessor from analyze — no AST traversal in codegen |
| `script.rs:gen_script` | Takes pre-parsed `ctx.parsed.script_program` — correct phase boundary |
| `template/render_tag.rs` (accessors) | Each flag comes from a dedicated accessor — no recomputation, just borderline on combination |
