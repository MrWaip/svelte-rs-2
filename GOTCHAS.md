# Gotchas, Data Flow & Quick Reference

---

## Just commands

All common operations are in the `justfile`. Use `just` instead of raw cargo commands:

| Command | What it does |
|---|---|
| `just generate` | Generate reference snapshots for both compiler-output and diagnostic-parity test cases |
| `just test-compiler` | Run all compiler integration tests |
| `just test-case <name>` | Run a single compiler test case |
| `just test-case-verbose <name>` | Run a single compiler test case with output |
| `just test-diagnostics` | Run all diagnostic parity integration tests |
| `just test-diagnostic-case <name>` | Run a single diagnostic parity test case |
| `just test-all` | Run all tests across all crates |
| `just test-parser` | Run parser tests |
| `just test-analyzer` | Run analyzer tests |
| `just generate-benchmark [name] [chunks]` | Generate benchmark `.svelte` file (default: `big_v5`, 50 chunks) |
| `just compare-benchmark [file]` | Wall-clock comparison: Rust vs Svelte JS compiler |

## Benchmarks

Two complementary systems:

- **CodSpeed (CI, automatic)** — runs on every push, tracks CPU instruction count via Valgrind. Deterministic, no noise. Alerts on regressions. Only measures Rust.
- **`just compare-benchmark` (local, manual)** — wall-clock Rust vs JS comparison. Run after major changes or for reporting.

Benchmark file is versioned (`big_vN.svelte`). When porting a feature that adds new syntax, `/port-svelte` Step 8 bumps the version.

## Testing

### Unit test pattern

Every unit test follows the same structure: **parse -> assert via helpers**. No manual field access, no `.unwrap()` chains in test bodies.

Rules:
- Each crate has a `parse_*` or `analyze_*` entry function that returns the data under test
- All assertions go through `assert_*` helpers defined in the test module
- Add new `assert_*` helpers when new features need testing -- don't inline field access
- When writing or modifying any test in `svelte_parser`, apply `/test-pattern` automatically

### Where tests live

- **Parser** -- `crates/svelte_parser` tests, span-based pattern per `/test-pattern`
- **Analyze** -- `crates/svelte_analyze/src/tests.rs`, entry: `analyze_source()` -> `(Component, AnalysisData)`
- **Compiler integration** -- `tasks/compiler_tests/cases2/`, each case has `case.svelte` (input), `case-svelte.js` (expected), `case-rust.js` (actual)
- **Diagnostic integration** -- `tasks/diagnostic_tests/cases/`, each case has `case.svelte` (input), `case-svelte.json` (reference diagnostics from npm `svelte/compiler`), `case-rust.json` (actual Rust diagnostics for visual diff)

---

## Glossary

| Term | Meaning |
|------|---------|
| **lowered** | Output of the `lower` pass: whitespace-trimmed, adjacent Text+ExprTag grouped into `FragmentItem`. Not just "simplified AST". |
| **hoisted** | Statements moved out of the component function to module scope (template declarations, snippet functions). |
| **dynamic** | Depends on a rune variable (not just "can change"). Tracked via `dynamic_nodes`, `element_flags.dynamic_attrs`. |
| **anchor** | DOM position node (`Comment`) where a block mounts (`$.if`, `$.each`, `$.comment`). |
| **tmp var** | `$$const_0`, `$$const_1`, … — generated variables for `{@const}` destructuring. |
| **prop source** | A `$props()` field that gets a `$.prop()` signal wrapper (when `$bindable` or a default with side effects). |

---

## Data Flow Per Pass

What each pass reads and writes (see `svelte_analyze/src/lib.rs` for canonical order):

```
classify_render_tags  reads: parsed.exprs, component
                      writes: data.render_tag_*

extract_script_info   reads: parsed.program
                      writes: ScriptInfo (declarations, props_declaration, exports)

analyze_script        reads: parsed, data.script (ScriptInfo)
                      writes: data.expressions, data.attr_expressions, data.needs_context,
                              data.exports, data.has_class_state_fields, data.proxy_state_inits
                      returns: OXC Scoping → ComponentScoping

mark_runes            reads: data.script, parsed.program
                      writes: data.scoping (rune kinds, derived deps)

template_scoping      reads: component, parsed (stmts for snippet arrow scope pre-set)
                      writes: data.scoping (fragment scopes: each, snippet, if, await, key, head,
                              boundary, svelte:element. Pre-sets ArrowFunctionExpression.scope_id
                              for snippets so SemanticCollector reuses the scope)

template_semantic     reads: component, parsed (expressions/statements), data.scoping
  (SemanticCollector,   writes: OXC AST nodes mutated in-place: BindingIdentifier.symbol_id,
   OXC Visit)                  IdentifierReference.reference_id. data.scoping gets bindings,
                               resolved references (Read/Write/ReadWrite flags), JS scopes
                               (arrow, function, block, for, catch). After this pass ALL template
                               expression identifiers have valid reference_id → symbol_id.
  + template_side_tables  writes: data.each_blocks, data.snippets, data.const_tags, data.element_flags

collect_symbols       reads: data.scoping (OXC references), data.expressions
                      writes: ExpressionInfo.ref_symbols, store detection, index usage

classify_needs_context reads: data.expressions, data.scoping (import_syms, prop syms)
                      writes: ExpressionInfo.needs_context, data.needs_context (aggregated)

post_resolve          reads: data.script, data.scoping
                      writes: data.props (PropsAnalysis), data.scoping (prop_source, rest_prop),
                              data.known_values, data.needs_context (store aggregation)

classify_dynamicity   reads: data.expressions, data.scoping (dynamic cache)
                      writes: ExpressionInfo.is_dynamic

lower                 reads: component, data.scoping
                      writes: data.fragments.lowered

Walk 1: reactivity    reads: data.fragments.lowered, data.scoping, data.expressions
                      writes: data.dynamic_nodes, data.element_flags.dynamic_attrs

Walk 2: element_flags reads: data.fragments.lowered, data.dynamic_nodes, data.element_flags
  + hoistable         writes: data.element_flags (spread, class/style directives, needs_var, needs_ref),
  + bind_semantics        data.snippets.hoistable, data.bind_semantics,
  + content_types         data.fragments.content_types, data.fragments.has_dynamic_children

validate              reads: data.* (read-only)
                      writes: diags

── after analyze ──

transform          reads: component, data.props, data.scoping, data.const_tags.by_fragment
                   writes: parsed.exprs (in-place rewrite), data.const_tags.tmp_names

── after transform ──

codegen            reads: component, data.*, parsed.*
                   writes: JS output (String)
```

---

## Traps

### 1. `ConstTagData.tmp_names` is filled by transform, not analyze

`ctx.analysis.const_tags.tmp_name(id)` returns `None` unless `transform_component` has already run. Codegen relies on this — pipeline order matters.

### 2. `ParsedExprs` is separate from `AnalysisData` by design

`ParsedExprs<'a>` carries an OXC lifetime. It cannot live inside `AnalysisData` (would propagate `'a` everywhere). Pass it separately through the entire pipeline. `parsed.script_program` is `Option` — consumed once via `.take()` in codegen.

### 3. Two unrelated concat-part types

```rust
svelte_ast::ConcatPart          { Static(String), Dynamic(Span) }                 // in Attribute
svelte_analyze::LoweredTextPart { TextSpan(Span), TextOwned(String), Expr(NodeId) } // in LoweredFragment
```
`TextSpan` — source slice (raw text), `TextOwned` — decoded HTML entities (when `Text::value()` differs from source).
Different names now, but be aware they serve analogous roles in different phases.

### 4. Hoisting order in codegen is bottom-up

Hoist **after** `process_element`, not before. The element's children write into `ctx.module_hoisted` first, then the element itself is hoisted. Reversing this produces wrong template declaration order.

### 5. `SingleBlock` vs `SingleElement`

`ContentStrategy::SingleElement(NodeId)` — exactly one `Element` node. `SingleBlock(FragmentItem)` — exactly one block node (IfBlock, EachBlock, etc.), stored as a `FragmentItem` directly (no intermediate enum). Codegen paths are fundamentally different: SingleElement uses `$.template(...)`, SingleBlock uses `$.comment()` as anchor.

### 6. `BindSemanticsData` is pre-computed in analysis, not codegen

Directive targets (mutable rune? prop source? each-block context?) are classified once during the composite walk via `BindSemanticsVisitor`. Codegen queries by `NodeId` (`ctx.is_mutable_rune_target(id)`, `ctx.is_prop_source_node(id)`, `ctx.bind_each_context(id)`). Do **not** re-resolve symbols from source text in codegen — use the pre-computed side tables.

### 7. `needs_var` vs `needs_ref`

- `needs_var` — element needs a JS variable in codegen (for dynamic attributes, directives, etc.)
- `needs_ref` — element needs a ref-semantic variable specifically (for `bind:this`)

### 8. `$host()` without props/exports breaks at runtime (reference compiler bug)

`$host()` transforms to `$$props.$$host`. But `$$props` is only added as a function parameter when `needs_push` is true (has props, exports, effects, or bindable). A custom element component that uses `$host()` alone (no `$props()`, no exports, no `$effect`) compiles to:

```js
export default function App($$anchor) {   // ← no $$props param
  let host = $$props.$$host;              // ← ReferenceError at runtime
}
```

**This is a bug in the reference Svelte compiler that we intentionally replicate.** The fix would be: detect `$host()` in script analysis → set `needs_context = true` → triggers `$$props` param + `$.push`/`$.pop`. Not implemented because the reference compiler has the same bug. See `host_basic` test case.

### 9. Counter-alignment hacks in codegen

The reference compiler's identifier counter (`scope.generate('fragment')`, `unique('root')`) increments unconditionally along certain code paths, even when the identifier produced is ultimately unused. Our `gen_ident` calls must mirror that order exactly, or all downstream names shift.

Two known cases:

**`gen_ident("fragment")` for non-dynamic top-level `ComponentNode`**
(`svelte_codegen_client/src/template/mod.rs`, `emit_single_block`)

In the reference compiler, `Fragment.visit()` always calls `scope.generate('fragment')` before checking `is_standalone`. Even on the standalone path (where no `fragment` variable is emitted), the counter advances. We replicate this by calling `ctx.gen_ident("fragment")` unconditionally for non-dynamic, non-svelte:self components at root level.

**`gen_ident("root")` for `<svelte:fragment slot="name">` wrappers**
(`svelte_codegen_client/src/template/component.rs`, `gen_component`)

The reference compiler visits the `SvelteFragment` wrapper as its own `Fragment` node, which calls `unique('root')` for its template_name — even though no template is emitted. We replicate this by calling `ctx.gen_ident("root")` for any named slot whose element was a `<svelte:fragment>` wrapper. The flag `ElementFlags::svelte_fragment_slots` (set in `lower.rs`, keyed by slot element NodeId) identifies these slots.

If the reference compiler changes its counter allocation order, these calls will silently misalign. Track via `svelte_fragment_named_slot` and `svelte_self_if` compiler test cases.

### 10. `Text::raw_value()` and `Text::value()` are intentionally different

- `Text::raw_value(source)` is for span-accurate diagnostics and source slicing.
- `Text::value(source)` is for semantic text content and may return decoded HTML entities.

Lowering and text-concat code must use `value()`, while diagnostics that report original source
locations should keep using spans or `raw_value()`. If you use `raw_value()` in lowering, mixed
text/expression output will regress back to escaped `&amp;`-style strings.

---

## Checklist for a New Node Type

Adding `Node::FooBar { id, span, expression_span, fragment }`:

```
Parser / AST:
□ svelte_ast/src/lib.rs        — struct FooBar + add variant to Node enum
□ svelte_parser/src/lib.rs     — scanner token + parse the tag

Parse JS:
□ svelte_parser/src/parse_js.rs       — register expression_span in parsed.exprs

Analyze:
□ svelte_analyze/src/lower.rs         — add FooBar to FragmentItem enum + handle in lower()
□ svelte_analyze/src/walker.rs        — add on_foo_bar() to TemplateVisitor, call from walk_fragment()
□ svelte_analyze/src/reactivity.rs    — handle in ReactivityVisitor if expression can be dynamic
□ svelte_analyze/src/data.rs          — add FragmentKey::FooBarBody(NodeId) if it has a child fragment
□ svelte_analyze/src/content_types.rs — account for FooBar in fragment classification

Transform:
□ svelte_transform/src/lib.rs         — add transform_foo_bar() if it has expressions to rewrite

Codegen:
□ svelte_codegen_client/src/template/mod.rs      — dispatch on FragmentItem::FooBar(id)
□ svelte_codegen_client/src/template/foo_bar.rs  — gen_foo_bar() function
□ svelte_codegen_client/src/context.rs           — add to NodeIndex if O(1) lookup needed
```

---

## Typical Input → Output Patterns

### ExpressionTag (dynamic)
```svelte
<script>let count = $state(0); count++;</script>{count}
```
```js
// hoisted:
var root = $.template(`<!> `);
// body:
$.next();
var text = $.text();
$.template_effect(() => $.set_text(text, $.get(count)));
$.append($$anchor, text);
```

### IfBlock
```svelte
{#if show}<p>hi</p>{/if}
```
```js
var fragment = $.comment();
var node = $.first_child(fragment);
$.if(node, () => $.get(show), ($$anchor) => {
  var p = $.template(`<p>hi</p>`);
  $.append($$anchor, p());
});
$.append($$anchor, fragment);
```

### EachBlock
```svelte
{#each items as item}<p>{item}</p>{/each}
```
```js
var fragment = $.comment();
var node = $.first_child(fragment);
$.each(node, 16, () => $.get(items), $.index, ($$anchor, item) => {
  // ...
});
$.append($$anchor, fragment);
```

### `{@const}` (destructured)
```svelte
{@const { x, y } = point}
```
```js
// transform generates tmp var, rewrites aliases:
const $$const_0 = $.derived(() => point);
// x → $.get($$const_0).x in subsequent expressions
```

---

## EachBlock Compilation

Input variants and their codegen output:

| Svelte syntax | Callback param | Notes |
|---|---|---|
| `{#each expr as name}` | `($$anchor, name)` | Simple identifier from parsed `let name = x;` |
| `{#each expr as name, i}` | `($$anchor, name, i)` | + index param |
| `{#each expr as name (key)}` | `($$anchor, name)` | Keyed, key fn gets separate arrow |
| `{#each expr as { id, name }}` | `($$anchor, $$item)` | Destructured → `$$item` + getter declarations |
| `{#each expr as [a, b]}` | `($$anchor, $$item)` | Array destructured → `$$item` + `$.to_array` |
| `{#each expr}` | `($$anchor, $$item)` | No context → `$$item` |
| `{#each expr, i}` | `($$anchor, $$item, i)` | No context + index |

Context name resolution (codegen, `each_block.rs`):
- Parser wraps context as `let PATTERN = x;` → stored in `parsed.stmts[context_span.start]`
- Codegen checks `declarator.id`: `BindingIdentifier` → use name, otherwise → `"$$item"`

---

## #11 Rest prop member access: `props.label` → `$$props.label`

When `$props()` uses rest destructuring (`let { id, ...props } = $props()`), member access on the rest variable is rewritten to access `$$props` directly: `props.label` → `$$props.label`. This avoids going through the `$.rest_props()` proxy.

- Only `StaticMemberExpression` — computed access (`props["label"]`) is not rewritten (matches reference)
- Properties explicitly destructured before rest are excluded (e.g. `props.id` stays as-is when `id` is destructured)
- Applies in both script (OXC Traverse, resolved via `reference_id → symbol_id`) and template (OXC VisitMut, resolved via `reference_id` set by `TemplateSemanticVisitor`)
- Triggers `needs_context = true` → `$.push`/`$.pop` injection (rest prop bindings are "unsafe" in `is_safe_identifier` terms)

## #12 TS type-only import removal: orphaned comment reattachment

When TypeScript type-only imports (`import type { ... }`) are stripped during script transformation, comments between the removed import and adjacent statements become orphaned — their `Comment::attached_to` no longer matches any statement's `span.start`. OXC codegen groups comments by `attached_to` so orphaned comments silently disappear.

Fix: `reattach_orphaned_comments()` runs after TS stripping, finds comments whose `attached_to` doesn't match any remaining statement, and reassigns them to the next statement by span position.
