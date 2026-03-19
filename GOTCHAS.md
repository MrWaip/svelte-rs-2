# Gotchas, Data Flow & Quick Reference

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

What each pass reads and writes:

```
parse_js           reads: component (source spans)
                   writes: data.expressions, data.attr_expressions, data.script
                           data.const_tags.names, data.const_tags.by_fragment
                           parsed.exprs, parsed.attr_exprs, parsed.script_program

build_scoping      reads: component, data.script
                   writes: data.scoping (ComponentScoping with unified scope tree)

resolve_references reads: component, data.scoping
                   writes: data.scoping (mutations marked)

store_subscriptions reads: data.scoping, data.script
                   writes: data.store_subscriptions

known_values       reads: data.script
                   writes: data.known_values

props              reads: data.script, data.scoping
                   writes: data.props

lower              reads: component, data.scoping
                   writes: data.fragments.lowered

composite walk     reads: data.fragments.lowered, data.scoping, data.expressions
                   writes: data.dynamic_nodes, data.alt_is_elseif,
                           data.element_flags (has_spread, has_class/style_directives, dynamic_attrs),
                           data.snippets.hoistable,
                           data.bind_semantics (mutable_rune_targets, prop_source_nodes, bind_each_context)

classify           reads: data.fragments.lowered, data.dynamic_nodes
                   writes: data.fragments.content_types, data.fragments.has_dynamic_children

needs_var          reads: data.fragments.content_types, data.element_flags
                   writes: data.element_flags.needs_var, data.element_flags.needs_ref

validate           reads: data.* (read-only)
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

### 3. Two unrelated `ConcatPart` types

```rust
svelte_ast::ConcatPart    { Static(String), Dynamic(Span) }    // in Attribute
svelte_analyze::ConcatPart { Text(String), Expr(NodeId) }      // in LoweredFragment
```
The compiler catches confusion, but they look identical in search results.

### 4. Hoisting order in codegen is bottom-up

Hoist **after** `process_element`, not before. The element's children write into `ctx.module_hoisted` first, then the element itself is hoisted. Reversing this produces wrong template declaration order.

### 5. `SingleBlock` vs `SingleElement`

`ContentStrategy::SingleElement(NodeId)` — exactly one `Element` node. `SingleBlock(FragmentItem)` — exactly one block node (IfBlock, EachBlock, etc.), stored as a `FragmentItem` directly (no intermediate enum). Codegen paths are fundamentally different: SingleElement uses `$.template(...)`, SingleBlock uses `$.comment()` as anchor.

### 6. `IfBlock.elseif` vs `data.alt_is_elseif`

- `IfBlock.elseif: bool` (AST field) — marks that *this* IfBlock is an elseif branch.
- `data.alt_is_elseif: HashSet<NodeId>` — contains the NodeId of the *parent* IfBlock whose alternate is a single elseif. Different things.

### 7. `BindSemanticsData` is pre-computed in analysis, not codegen

Directive targets (mutable rune? prop source? each-block context?) are classified once during the composite walk via `BindSemanticsVisitor`. Codegen queries by `NodeId` (`ctx.is_mutable_rune_target(id)`, `ctx.is_prop_source_node(id)`, `ctx.bind_each_context(id)`). Do **not** re-resolve symbols from source text in codegen — use the pre-computed side tables.

### 8. `needs_var` vs `needs_ref`

- `needs_var` — element needs a JS variable in codegen (for dynamic attributes, directives, etc.)
- `needs_ref` — element needs a ref-semantic variable specifically (for `bind:this`)

---

## Checklist for a New Node Type

Adding `Node::FooBar { id, span, expression_span, fragment }`:

```
Parser / AST:
□ svelte_ast/src/lib.rs        — struct FooBar + add variant to Node enum
□ svelte_parser/src/lib.rs     — scanner token + parse the tag

Analyze:
□ svelte_analyze/src/parse_js.rs      — register expression_span in parsed.exprs
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
