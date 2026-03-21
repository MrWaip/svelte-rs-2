# Audit: Phase Boundary Violations in Codegen & Transform

Generated: 2026-03-21

## Class 1: Full Re-parse in Codegen

### 1.1 Prop default expression re-parsing

- **Class:** 1
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/script.rs:139-144`
- **Occurrence count:** 1
- **What is re-parsed:** Property default expressions from `prop.default_text` string via `b.parse_expression(text)`
- **Why it's wrong:** `ParsedExprs::prop_default_exprs` is already populated by `parse_js.rs` — codegen should consume the pre-parsed expressions, not re-parse from string
- **Proposed type:** No new type needed — use existing `ParsedExprs::prop_default_exprs` instead of `prop.default_text`
- **Target layer:** codegen refactor

### 1.2 Bind directive setter expression construction via parse

- **Class:** 1
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/template/component.rs:210-211`
- **Occurrence count:** 1
- **What is re-parsed:** `format!("{var_name} = $$value")` → `b.parse_expression(...)` constructs an assignment expression by building a string and re-parsing it
- **Why it's wrong:** `Builder::assign_expr()` can construct this directly without parse roundtrip
- **Proposed type:** No new type — use `ctx.b.assign_expr(AssignLeft::Ident(var_name), ctx.b.rid_expr("$$value"))`
- **Target layer:** codegen refactor

### 1.3 Bind directive getter expression via parse

- **Class:** 1
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/template/component.rs:221`
- **Occurrence count:** 1
- **What is re-parsed:** `b.parse_expression(&var_name)` parses a simple identifier/member expression from string
- **Why it's wrong:** For member expressions this is arguably needed (optional chaining), but for simple identifiers `ctx.b.rid_expr()` suffices
- **Proposed type:** No new type — split: simple ident → `rid_expr`, member expr → keep `parse_expression` (or build AST directly)
- **Target layer:** codegen refactor

---

## Class 2: String Re-parsing

### 2.1 Store reference `$name` → `name` base name slicing

- **Class:** 2
- **Complexity:** S
- **Where:** `crates/svelte_transform/src/lib.rs:327,350,369,428`
- **Occurrence count:** 4
- **What is extracted:** `&name[1..]` strips the `$` prefix to get the store base name
- **Why it's wrong:** Manual byte slicing to extract structured info (store ref → base name)
- **Proposed type:** `fn store_base_name(name: &str) -> &str` method on `ComponentScoping` (validate + return cached base name), or store the base name directly in the symbol table
- **Target layer:** analyze (accessor method)

### 2.2 BindDirective expression text extraction from Span

- **Class:** 2
- **Complexity:** M
- **Where:** `crates/svelte_codegen_client/src/template/attributes.rs:589-592`, `component.rs:158-161`, `svelte_window.rs:54-57`, `svelte_document.rs:54-57`
- **Occurrence count:** 4
- **What is extracted:** `ctx.component.source_text(span).to_string()` extracts bind target variable name from expression span
- **Why it's wrong:** Same pattern in 4 files — structural data (the bind target identifier) is re-extracted from source text each time
- **Proposed type:** `BindDirective.expression_text: Option<String>` in parser, or analyze accessor `fn bind_var_name(id: NodeId) -> &str`
- **Target layer:** parser (store in AST) or analyze (accessor)

### 2.3 Shorthand attribute name extraction from Span

- **Class:** 2
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/template/attributes.rs:160,990`
- **Occurrence count:** 2
- **What is extracted:** `ctx.component.source_text(a.expression_span).trim()` / `.to_string()` extracts the shorthand identifier name
- **Why it's wrong:** The `Shorthand` struct only stores `expression_span`, forcing codegen to re-extract the identifier name from source
- **Proposed type:** `Shorthand.name: String` in parser — store the identifier name alongside the span
- **Target layer:** parser

### 2.4 StringAttribute value extraction from Span

- **Class:** 2
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/template/attributes.rs:952`
- **Occurrence count:** 1
- **What is extracted:** `ctx.component.source_text(a.value_span).to_string()` re-extracts the literal string value
- **Why it's wrong:** Parser already knows the string value (it parsed the quotes) — should store it
- **Proposed type:** `StringAttribute.value: String` in parser
- **Target layer:** parser

### 2.5 SvelteElement static tag value from Span

- **Class:** 2
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/template/svelte_element.rs:24-25`
- **Occurrence count:** 1
- **What is extracted:** `ctx.component.source_text(el.tag_span).to_string()` gets the static tag name
- **Why it's wrong:** When `static_tag` is true, the parser knows the tag value — should store it directly
- **Proposed type:** `SvelteElement.static_tag_name: Option<String>` in parser
- **Target layer:** parser

### 2.6 Each block prop source expression from Span

- **Class:** 2
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/template/each_block.rs:89-90`
- **Occurrence count:** 1
- **What is extracted:** `ctx.component.source_text(expr_span).trim()` then used as `rid_expr` — extracts expression source for prop-source each blocks
- **Why it's wrong:** The pre-parsed `Expression` from `ParsedExprs` should be used directly via `get_node_expr` (it's already parsed); the prop-source branch shouldn't need a different extraction path
- **Proposed type:** No new type — use `get_node_expr()` and wrap in identity if it's already a function reference
- **Target layer:** codegen refactor

---

## Class 3: AST Re-traversal in Codegen

### 3.1 Special element collection (4 passes over root fragment)

- **Class:** 3
- **Complexity:** M
- **Where:** `crates/svelte_codegen_client/src/template/mod.rs:138-164`
- **Occurrence count:** 4 (SvelteWindow, SvelteDocument, SvelteBody, SvelteHead)
- **What is aggregated:** NodeIds for each special element type, filtered from `component.fragment.nodes`
- **Proposed type:** `AnalysisData` accessors: `fn svelte_window_ids() -> &[NodeId]`, etc. Pre-collected during analysis in a single pass
- **Target layer:** analyze

### 3.2 Bubble event detection (double nested traversal)

- **Class:** 3
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/lib.rs:192-201`
- **Occurrence count:** 1
- **What is aggregated:** Bool: whether any SvelteWindow/SvelteDocument has `OnDirectiveLegacy` with no expression (bubble event). Double nested iter: outer over nodes, inner over attributes
- **Proposed type:** `fn has_legacy_bubble_events(&self) -> bool` accessor in `AnalysisData`
- **Target layer:** analyze

### 3.3 SVG namespace detection on SvelteElement

- **Class:** 3
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/template/svelte_element.rs:40-47`
- **Occurrence count:** 1
- **What is aggregated:** Bool: whether `svelte:element` has `xmlns="http://www.w3.org/2000/svg"` attribute. Iterates attributes with `.any()`
- **Proposed type:** `fn svelte_element_is_svg(&self, id: NodeId) -> bool` accessor in `ElementFlags`
- **Target layer:** analyze

### 3.4 Bind directive lookup by linear search

- **Class:** 3
- **Complexity:** S
- **Where:** `crates/svelte_codegen_client/src/template/component.rs:154-156`
- **Occurrence count:** 1
- **What is aggregated:** Single `BindDirective` looked up by `bind_id` via linear search through component attributes
- **Proposed type:** Store the bind expression text or var_name in the component props analysis data, avoiding the lookup entirely
- **Target layer:** analyze

---

## Class 4: Derived Flags Without a Name

### 4.1 Component function push/exports composite flags

- **Class:** 4
- **Complexity:** M
- **Where:** `crates/svelte_codegen_client/src/lib.rs:46-48`
- **Occurrence count:** 6 uses across lib.rs (lines 47, 48, 67, 108, 150, 158, 159, 203)
- **What is aggregated:**
  - `has_ce_props = is_custom_element && props.is_some_and(|p| !p.props.is_empty())`
  - `needs_push = has_bindable || has_exports || has_ce_props || needs_context || dev`
  - `has_component_exports = has_exports || has_ce_props || dev`
- **Proposed type:** Enum or struct in `AnalysisData`:
  ```rust
  pub struct ComponentFunctionMode {
      pub needs_push: bool,       // excl. dev — codegen adds dev check
      pub has_component_exports: bool, // excl. dev
      pub has_ce_props: bool,
  }
  ```
  Note: `dev` is a codegen-time flag, so analyze can pre-compute the non-dev parts and codegen ORs in `dev`
- **Target layer:** analyze

### 4.2 `should_proxy` decision scattered across script codegen

- **Class:** 4
- **Complexity:** M
- **Where:** `crates/svelte_codegen_client/src/script.rs:540,799,1776,1809,2153`
- **Occurrence count:** 5
- **What is aggregated:** Each call combines `RuneKind` + expression shape check (`should_proxy`) + sometimes `is_bindable` or `is_mutated` to decide proxy wrapping. The `should_proxy` function itself (line 398) inspects expression AST structure
- **Why it's borderline:** `should_proxy` operates on post-transform expressions (after rune rewrites), so analyze can't see the final expression shape. The decision genuinely belongs in codegen. However, the pattern `kind == State && should_proxy(value)` repeats 3+ times
- **Proposed type:** Extract a single `fn wrap_state_proxy(&self, kind: RuneKind, value: Expression<'a>) -> Expression<'a>` method to centralize the decision. This is a codegen-internal refactor, not an analyze migration
- **Target layer:** codegen refactor

### 4.3 Rune get decision: `is_mutated || kind.is_derived()`

- **Class:** 4
- **Complexity:** S
- **Where:** `crates/svelte_transform/src/lib.rs:309`
- **Occurrence count:** 1
- **What is aggregated:** `is_mutated(sym_id) || kind.is_derived()` → determines whether identifier needs `$.get()` wrapping
- **Why it's borderline:** Single occurrence, both flags come from analysis. But this is a core semantic decision ("does this rune ref need reactive read?")
- **Proposed type:** `fn needs_rune_get(&self, sym_id: SymbolId) -> bool` accessor on `ComponentScoping`
- **Target layer:** analyze (low priority — single occurrence)

---

## Good Examples

### `gen_key_block` — `template/key_block.rs`
Takes `build_node_thunk`, `gen_fragment`, assembles `$.key()` call. No re-parsing, no flag combination, no AST traversal.

### `emit_debug_tags` — `template/mod.rs`
Takes `debug_tags_for_fragment`, pre-transformed expressions from `ctx.parsed.debug_tag_exprs`, assembles output. Zero analysis logic.

### `EventHandlerMode` enum — `template/attributes.rs:82-123`
Codegen matches on `ctx.event_handler_mode(attr_id)` — a single enum from analyze with `Delegated { passive }` / `Direct { capture, passive }`. No re-derivation.

### `RenderTagCalleeMode` enum — `template/mod.rs:364`
Four-variant enum (`Direct | Chain | DynamicRegular | DynamicChain`) pre-computed in analyze. Codegen matches, no boolean combination.

### `class_needs_state` accessor — `ElementFlags`
Encapsulates `class_attr_dynamic || has_dynamic_class_directives`. Codegen calls `ctx.class_needs_state(el.id)` — one accessor, no flag recombination.

### `ComponentPropInfo` — component attribute classification
Pre-classified attribute kinds with computed `is_dynamic` flag. Codegen iterates and matches, doesn't re-classify.

### Custom element exports — `lib.rs:112-120`
Maps `ctx.analysis.exports` directly to output. Data came ready, codegen only formats.

### `needs_input_defaults` — `element.rs:38`
Pre-computed `FxHashSet<NodeId>` in analyze, exposed via accessor. Codegen does one boolean check.

---

## Summary

| Class | Count | S | M | L |
|-------|-------|---|---|---|
| 1: Full re-parse | 3 | 3 | 0 | 0 |
| 2: String re-parsing | 6 | 4 | 1 | 0 |*
| 3: AST re-traversal | 4 | 3 | 1 | 0 |
| 4: Derived flags | 3 | 1 | 2 | 0 |
| **Total** | **16** | **11** | **3** | **0** |

*\*2.2 (BindDirective) counted as M due to 4 occurrences across 4 files*

### Recommended migration order

1. **2.3 Shorthand.name** (S, parser) — trivial parser change, removes 2 source_text calls
2. **2.4 StringAttribute.value** (S, parser) — same pattern
3. **2.5 SvelteElement.static_tag_name** (S, parser) — same pattern
4. **1.1 Prop default re-parse** (S, codegen) — use existing `ParsedExprs`, zero new types
5. **1.2 + 1.3 Bind setter/getter** (S, codegen) — use Builder directly
6. **2.6 Each block prop source** (S, codegen) — use `get_node_expr`
7. **2.1 Store base name** (S, analyze) — add `store_base_name()` accessor
8. **3.2 Bubble events** (S, analyze) — single bool precomputation
9. **3.3 SVG namespace** (S, analyze) — single bool per SvelteElement
10. **3.4 Bind lookup** (S, analyze) — store bind var_name in analysis
11. **2.2 BindDirective text** (M, parser/analyze) — 4 occurrences, needs design choice
12. **3.1 Special elements** (M, analyze) — 4 collections, moderate refactor
13. **4.1 Component function mode** (M, analyze) — composite struct
14. **4.2 should_proxy centralize** (M, codegen) — internal refactor
15. **4.3 needs_rune_get** (S, analyze) — low priority, single occurrence
