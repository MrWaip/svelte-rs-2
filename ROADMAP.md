# Roadmap: Svelte 5 Client Compiler in Rust

Scope: client-side compilation only (no SSR, no legacy mode).
For a full feature parity audit, see [PARITY.md](PARITY.md).

**Phase notation**: **P** = Parser/AST, **A** = Analyze, **S** = Script codegen, **T** = Template codegen, **V** = Validation

---

<details>
<summary>Done ✅ (AST & Parser, Analyze, Script codegen, Template codegen, Event handling, Bind directives, Directives, Special elements, Module compilation, Optimizations, WASM, Custom Elements, Tier 1 Core Gaps, Tier 1.1 Experimental Async)</summary>

### AST & Parser (6 items)
`Text`, `Element`, `ComponentNode`, `Comment`, `ExpressionTag`, `IfBlock`, `EachBlock`, `SnippetBlock`, `RenderTag`, Attributes, Script/Style blocks, Void elements

### Analyze (11 passes)
`js_analyze`, `mark_runes`, `template_scoping`, `template_semantic`, `template_side_tables`, `collect_symbols`, `post_resolve`, `lower`, `reactivity`, `element_flags`+`hoistable`+`bind_semantics`+`content_types`, `validate`

### Script codegen (17 items)
All runes (`$state`, `$state.raw`, `$derived`, `$props`, `$effect`, `$inspect`, `$props.id`, `$host`), destructuring, class fields, `$store`, imports/exports, custom elements

### Template codegen (11 items)
Element, Component, IfBlock, EachBlock, SnippetBlock, RenderTag, Text, ExpressionTag, `{@html}`, `{#key}`, `{@const}`, `style:prop`, `class` object/array, `{#await}`, `{@debug}`

### Event handling (12 items)
Svelte 5 events, delegation, capture, passive, handler wrapping, `has_call` memoization, component prop memoization, render tag memoization, `on:event` legacy

### Bind directives (14 items)
`bind:value/checked/group/files`, `bind:innerHTML/innerText/textContent`, `bind:clientWidth/Height/offsetWidth/Height`, resize observer bindings, media bindings, `bind:this`, `bind:focused`

### Directives (4 items)
`use:action`, `transition:/in:/out:`, `animate:`, `{@attach}`

### Special elements (9 items)
`<svelte:options/head/window/document/body/element/boundary>`, `<title>` in head, component `bind:this`

### Module compilation, Optimizations, WASM, Custom Elements
All completed.

### Tier 1 — Core Gaps ✅
1a `ModuleCompileOptions`, 1b Template expression transforms (`svelte_transform`) — all completed.

### Tier 1.1 — Experimental Async ✅
All client-side async features completed: infrastructure, block wrapping (if/each/html/key/await/svelte:element), directive blockers, `{@const}` async, `$derived` async, memoizer async, `{@render}` async, `<title>` async, `<svelte:boundary>` async, `{await expr}` template syntax, pickled awaits, dev-mode reactivity loss tracking.

</details>

---

## Tier 2 — Remaining Edge Cases

Edge cases and missing features discovered during porting. Grouped by feature area.

### 2a — Runes & script
- [ ] Custom element `$.push`/`$.pop` lifecycle for `$host()` mutations

### 2b — Template tags
- [ ] `{#await}` — dev-mode `$.apply()` wrapping for await expression
- [ ] `{#snippet}` — parameter destructuring: array/object patterns with defaults → per-field `$.derived()` wrappers

### 2e — Special elements
- [ ] `<svelte:boundary>` — dev mode: snippet wrapping with `$.wrap_snippet`
- [ ] `<svelte:boundary>` — handler wrapping for snippet params as event handlers
- [ ] `<svelte:element>` — dynamic `xmlns` attribute for runtime namespace switching

### 2f — CSS
- [ ] Component CSS custom properties on `<Component>` — `$.css_props()` wrapper element injection

### 2g — Compiler infrastructure
- [ ] `fragments: 'tree'` option — alternative DOM fragment strategy

### 2h — Custom Elements
- [ ] HMR conditional registration: `if (customElements.get(tag) == null)`
- [ ] Shadow DOM custom `ObjectExpression` (non-literal config)
- [ ] `$.push`/`$.pop` lifecycle for `$host()` mutations (reference compiler bug — see GOTCHAS.md #9)
- [ ] Auto-detect boolean type from prop default literal value (in CE props config)

### 2i — Legacy `on:directive`
- [ ] Call memoization: `on:click={getHandler()}` → `$.derived(() => getHandler())` + `$.get()`
- [ ] SvelteDocument/SvelteBody routing: events on special elements → `init` not `after_update`
- [ ] Dev-mode `$.apply()` wrapping for imported identifier handlers

### 2j — Element attribute edge cases
- [ ] `muted` attribute — direct property assignment (`el.muted = value`) instead of `$.set_attribute`
- [ ] `checked` attribute — `$.set_checked(el, value)` instead of generic attribute
- [ ] `selected` attribute — `$.set_selected(el, value)` instead of generic attribute
- [ ] `defaultValue` — `$.set_default_value(el, value)` when static `value` attr present
- [ ] `defaultChecked` — `$.set_default_checked(el, value)` when static `checked=true` present
- [ ] `xlink:*` attributes — `$.set_xlink_attribute(el, name, value)` for SVG xlink namespace
- [ ] DOM properties — `is_dom_property()` check → direct assignment `el[name] = value` instead of setAttribute
- [ ] `$.set_custom_element_data(el, name, value)` — attribute setting for custom elements (non-idempotent, needs `$.template_effect`)
- [ ] `autofocus` — `$.autofocus(el, value)` instead of generic attribute
- [ ] `dir` attribute — Chromium workaround: re-assign `el.dir = el.dir` after text content update

Ref: `RegularElement.js` lines 583–725, `shared/element.js`

### 2k — Form element special handling
- [ ] `$.remove_textarea_child(el)` — called for `<textarea>` with spread, `bind:value`, or dynamic value
- [ ] `$.init_select(el)` — initialize select element for value tracking
- [ ] `$.select_option(el, value)` — sync select option when value changes dynamically
- [ ] `customizable_select` — rich HTML content in `<select>`/`<option>`/`<optgroup>` → `$.customizable_select()` wrapper

Ref: `RegularElement.js` lines 166–202, 470–725

### 2l — Event replay
- [ ] `$.replay_events(el)` — re-trigger queued load/error events for `<img>`, `<video>`, `<audio>`, `<source>`, etc. when element has spread, `use:`, or `onload`/`onerror` attribute

Ref: `RegularElement.js` lines 280–284

### 2m — EachBlock edge cases
- [ ] Collection ID (scope shadowing) — when context variable shadows outer scope binding, store array in `$$array` and pass as extra render_fn arg
- [ ] Store invalidation — `$.invalidate_store($$stores, 'name')` when collection expression uses `$store` subscription

Ref: `EachBlock.js` lines 45–110 (flags), 139–288 (context/index), 293–354 (key/fallback/async)

---

## Tier 3 — CSS Scoping

Theme: scoped CSS compilation — largest standalone workstream, new `svelte_css` subsystem.

Pipeline: parse → hash → analyze → prune → transform → inject into template.
Ref: `1-parse/read/style.js`, `2-analyze/css/`, `3-transform/css/index.js`

### 3.0 — Research: выбор CSS-стека
- [ ] Оценить варианты: OXC css parser (`oxc_css`), `lightningcss`, `cssparser` (Servo), свой парсер
- [ ] Критерии: полнота CSS3 selectors, `:global()` / nesting (`&`), доступ к AST для мутаций (scoping, pruning), source map support, размер зависимости, lifetime ergonomics
- [ ] Проверить можно ли переиспользовать существующий парсер и надстроить metadata enrichment, или проще свой мини-парсер (Svelte парсит только selectors + at-rules, declarations хранит как строки)
- [ ] Решение зафиксировать в ADR или комментарием в этом разделе

### 3a — CSS AST & parser (new crate `svelte_css`)
- [ ] CSS AST types: `StyleSheet`, `Rule`, `Atrule`, `SelectorList`, `ComplexSelector`, `RelativeSelector`, `SimpleSelector` variants (type, class, id, attribute, pseudo-class, pseudo-element, combinator, nesting `&`)
- [ ] Parse `<style>` content into CSS AST — selectors, declarations, at-rules (`@media`, `@keyframes`, etc.)
- [ ] Nested rules support (CSS nesting with `&`)

Ref: `1-parse/read/style.js` (~638 lines), `types/css.d.ts` (~201 lines)

### 3b — CSS hash computation
- [ ] Deterministic hash from `filename` (preferred) or CSS content (fallback)
- [ ] Format: `svelte-{hash}` (e.g., `svelte-a1b2c3d4`)
- [ ] Store on `AnalysisData.css.hash`
- [ ] Support custom `cssHash` option

Ref: `2-analyze/index.js` lines 536–548, `validate-options.js` line 73

### 3c — CSS analysis: global/local classification
- [ ] Walk CSS AST, enrich selector metadata: `is_global`, `is_global_like`
- [ ] `:global(selector)` — contents unscoped
- [ ] `:global { ... }` — entire block unscoped
- [ ] `:global` bare (no args) — everything after is unscoped
- [ ] `:global` in pseudo-classes (`:is()`, `:has()`, `:where()`, `:not()`)
- [ ] `is_global_like` for `:root`, `:host`, `::view-transition-*`
- [ ] Collect keyframe names (skip `-global-` prefixed)
- [ ] Validation: invalid `:global()` placement

Ref: `2-analyze/css/css-analyze.js` (~332 lines)

### 3d — CSS pruning: selector → template matching
- [ ] Backward selector matching against template elements
- [ ] Combinator traversal: descendant (` `), child (`>`), adjacent (`+`), general sibling (`~`)
- [ ] Mark `element.metadata.scoped = true` for matched elements
- [ ] Mark `selector.metadata.used = true` for matched selectors
- [ ] Handle `:has()`, `:not()`, `:is()`, `:where()` argument matching
- [ ] Conservative matching for components and snippets

Ref: `2-analyze/css/css-prune.js` (~1248 lines) — самый большой файл в CSS pipeline

### 3e — CSS transformation: scoping & output
- [ ] Append `.svelte-HASH` class to scoped selectors (first bump, rest via `:where()`)
- [ ] Remove `:global()` / `:global` syntax from output
- [ ] Scope `@keyframes name` → `@keyframes svelte-HASH-name`
- [ ] Patch `animation` / `animation-name` property values
- [ ] Prune unused rules (comment out in dev, remove in prod)
- [ ] Unwrap `:global { ... }` blocks
- [ ] Minification in prod mode (whitespace removal)
- [ ] CSS source map generation

Ref: `3-transform/css/index.js` (~480 lines)

### 3f — Template hash injection
- [ ] For scoped elements, add `class="svelte-HASH"` in template codegen
- [ ] Pass hash to `$.attribute_effect()` runtime call
- [ ] `css: 'injected'` — embed CSS in JS, runtime injects `<style>` tag
- [ ] `css: 'external'` — extract CSS to separate file

Ref: `3-transform/client/visitors/shared/element.js` lines 93–95

### 3g — CSS custom properties
- [ ] `--css-var={expr}` on elements — static: `$.set_style(el, "--var", value)`, dynamic: effect wrapping
- [ ] `<Component --css-var={expr}>` — `$.css_props()` wrapper element injection
- [ ] Nested `<style>` elements — no scoping, emit as global rules

---

## Tier 5 — Validation & Diagnostics

Theme: developer experience — errors, warnings, and diagnostic infrastructure.

### 5a — Infrastructure setup ✅

Spec: `specs/diagnostics-infrastructure.md`

- [x] `DiagnosticKind` — 81 warning variants + ~165 semantic error variants (all from reference `warnings.js` + `errors.js`)
- [x] Parameterized messages via enum fields + `format!()` in `message()`
- [x] `<!-- svelte-ignore -->` parsing (runes: comma-separated strict, legacy: space-separated lenient)
- [x] Legacy code migration map (9 mappings) + fuzzy-match suggestions
- [x] `IgnoreData` side table in `AnalysisData` with interned snapshots
- [x] Ignore stack in walker `VisitContext` — push/pop around nodes, preceding comment scan
- [x] `ctx.warn(node_id, kind, span)` API respecting ignore map
- [x] `AnalyzeOptions { custom_element, runes, dev, warning_filter }`
- [x] Warning filter applied after validate in `analyze_with_options()`
- [ ] Unused selector warnings — `css-warn.js` pattern (зависит от Tier 3)

### 5b — Runes & script

- [ ] `$state()` takes 0-1 args (`rune_invalid_arguments`). Ref: `2-analyze/visitors/CallExpression.js`
- [ ] `$state.frozen` → `$state.raw` rename validation
- [ ] `$derived` / `$derived.by` argument validation
- [ ] `$inspect` argument count (requires 1+ args)
- [ ] `$inspect().with(callback)` argument count
- [ ] `$inspect.trace()` must be first statement in function body (`inspect_trace_invalid_placement`)
- [ ] `$inspect.trace()` cannot be in generator function (`inspect_trace_generator`)
- [ ] `$inspect.trace()` 0-1 arguments (`rune_invalid_arguments_length`)
- [ ] `$props.id()` duplicate declarations (`props_duplicate`)
- [ ] `$props.id(arg)` no arguments allowed (`rune_invalid_arguments`)
- [ ] `$props.id()` wrong placement — inside function, module script (`props_id_invalid_placement`)
- [ ] `$props.id()` destructuring pattern (`const { x } = $props.id()`)
- [ ] `$props.id()` reassignment (`constant_assignment`)
- [ ] `$host()` must have zero arguments (`rune_invalid_arguments`)
- [ ] `$host()` only in custom element context (`host_invalid_placement`)
- [ ] `$host()` not in `<script module>`
- [ ] `constant_assignment` — error on assignment/update to `const`, imports, `$derived`/`$derived.by` variables (Ref: `2-analyze/visitors/shared/utils.js:validate_no_const_assignment`)
- [ ] `constant_binding` — error on `bind:` to `const`/import bindings
- [ ] `each_item_invalid_assignment` — error on assignment to `{#each}` iteration variable
- [ ] `snippet_parameter_assignment` — error on assignment to snippet parameter
- [ ] `state_field_invalid_assignment` — error on assignment to class state field before its declaration in constructor
- [ ] Module: disallow `$props()`, `$bindable()`, `$store` auto-subscriptions
- [ ] `store_invalid_scoped_subscription` — `$store` in nested scope (e.g., function inside instance script)
- [ ] `store_invalid_subscription_module` — `$store` in non-`.svelte` files (module compilation)

### 5c — Elements & special elements

- [ ] Void elements: error if void element has children
- [ ] `<svelte:window>` — only at root level, no children, no spread attrs, only one per component
- [ ] `<svelte:document>` — only at root level, no children, no spread attrs, only one per component
- [ ] `<svelte:body>` — only event attrs/directives, no children, only at root level, only one per component
- [ ] `<svelte:head>` — only at root level, no attributes allowed
- [ ] `<title>` in `<svelte:head>` — no attributes (`title_illegal_attribute`), children must be Text or ExpressionTag only (`title_invalid_content`)
- [ ] `<svelte:boundary>` — reject non-`onerror`/`failed`/`pending` attrs, reject string/boolean values
- [ ] `custom_element_props_identifier` warning when `$props()` used without CE props config

### 5d — Directives

- [ ] `bind:` vs element compatibility (e.g., `bind:checked` only on checkbox/radio). Ref: `2-analyze/visitors/BindDirective.js`
- [ ] `transition:` not on components. Ref: `2-analyze/visitors/Component.js`
- [ ] `transition:` duplicate directives on same element
- [ ] `transition:` + `in:`/`out:` conflicting on same element
- [ ] `animate:` only inside keyed `{#each}` blocks
- [ ] `animate:` duplicate directives on same element

### 5e — Template blocks

- [ ] `{@const}` placement validation
- [ ] `{#await}` duplicate `{:then}` or `{:catch}` clauses

### 5f — A11y warnings (~40 checks)
- Missing `alt` on `<img>`, `<area>`, `<input type="image">`
- ARIA attribute validation (`role`, `aria-*` correctness)
- Form label association (`<label>` + `for`/`id`)
- Keyboard event pairing (`onclick` → needs `onkeydown`)
- Heading hierarchy (`<h1>`–`<h6>` order)
- Interactive role focus management
- Media caption requirements
- Redundant/conflicting attributes
- **Ref**: `reference/compiler/phases/2-analyze/visitors/shared/a11y/` (~954 lines)

### 5g — `<!-- svelte-ignore -->` comments
- **Phases**: P, A
- Parse `<!-- svelte-ignore warning_name -->` from HTML comments
- Suppress specific warnings for the next sibling node
- `extract_svelte_ignore()` + `is_ignored(node, 'rule')` check
- **Ref**: `reference/compiler/phases/2-analyze/index.js`

---

## Tier 6 — Compiler Infrastructure

Theme: compiler options, source maps, dev mode support.

### 6a — `discloseVersion` option
- [ ] `import {} from 'svelte/internal/disclose-version'` when `discloseVersion: true`

### 6b — `preserveComments` option
- [ ] Keep HTML comments in template output (`push_comment()` in fragment codegen)

### 6c — Dev: `$.tag()` / `$.tag_proxy()` rune tagging ✅
- [x] `$state` — `$.tag($.state(val), "name")` / `$.tag_proxy($.proxy(val), "name")` in VariableDeclaration
- [x] `$derived` — `$.tag($.derived(...), "name")` in VariableDeclaration
- [x] Class fields — `$.tag(val, "Class.field")` in ClassBody, AssignmentExpression
- [x] `$bindable` — `$.tag_proxy($.proxy(val), "name")` for bindable props
- [x] Snippets — `$.wrap_snippet(componentName, function)` in SnippetBlock
- [x] `{@const}` — `$.tag()` wrapping in ConstTag
- [x] Destructured state/derived — `$.tag`/`$.tag_proxy` on intermediates and leaves

### 6d — Dev: strict equality transforms
- [ ] `===` / `!==` → `$.strict_equals(left, right)` / `$.strict_equals(left, right, false)`
- [ ] `==` / `!=` → `$.equals(left, right)` / `$.equals(left, right, false)`

Single visitor: BinaryExpression

### 6e — Dev: `$.apply()` + event handler naming
- [ ] Arrow→named function: `(e) => handler` → `function click() { ... }` for event handlers
- [ ] `$.apply()` wrapping: `$.apply(thunk, this, args, ComponentName, [line, col])` for location tracking

Ref: `reference/compiler/phases/3-transform/client/visitors/shared/events.js`

### 6g — Dev: ownership validation
- [ ] `$.create_ownership_validator($$props)` setup in component body
- [ ] Mutation wrapping: assignments/updates → `$$ownership_validator.mutation(prop, path, val, line, col)`
- [ ] `ownership_invalid_binding` suppression via `svelte-ignore`

Ref: `reference/compiler/phases/3-transform/client/visitors/shared/component.js`

### 6h — Dev: runtime validations (batch)
- [ ] `$.validate_store(ref, name)` — store subscription validation
- [ ] `$.validate_dynamic_element_tag()` / `$.validate_void_dynamic_element()` — svelte:element checks
- [ ] `console.log(…)` → `console.log(...$.log_if_contains_state('log', ...args))` — console state logging
- [ ] `await expr` → `await $.track_reactivity_loss(expr)` — await reactivity loss tracking
- [ ] `$.rest_props($$props, seen, restName)` — pass name as dev-only 3rd arg

### 6i — Source maps
- [ ] JS source maps — map generated JS back to `.svelte` source
- [ ] CSS source maps — map scoped CSS back to source `<style>`
- [ ] Preprocessor merge — merge incoming preprocessor source maps

### 6j — HMR (low priority)
- [ ] `$.hmr()` wrapper — wrap component export for hot reload
- [ ] `import.meta.hot.accept()` + `$.cleanup_styles()`
- [ ] Custom element guard — `if (customElements.get(tag) == null)` conditional registration

---

## Tier 7 — Legacy Svelte 4 (Lowest Priority)

Theme: deprecated syntax superseded by Svelte 5 features. Only needed for migrating codebases.

### 7a — `<slot>` + `let:`
- [ ] `$.slot(...)` (P, A, T). Svelte 5: `{#snippet}` + `{@render}`

### 7b — `<svelte:component>`
- [ ] `$.component(...)` (P, A, T). Svelte 5: `<X />` with capitalized variable

### 7c — `<svelte:self>`
- [ ] Recursive ref (P, T). Svelte 5: import component directly

### 7d — `<svelte:fragment>`
- [ ] Fragment wrapper (P, T). Svelte 5: `{#snippet}`

### 7e — `export let` props
- [ ] Different script transform (S). Svelte 5: `$props()`

### 7f — `$:` reactive assignments
- [ ] Labeled statement → `$.derived`/`$.effect` (S). Svelte 5: `$derived` / `$effect`

### 7g — `$$props` / `$$restProps` / `$$slots`
- [ ] Runtime vars (S, T). Svelte 5: `$props()` with rest

### 7h — `beforeUpdate` / `afterUpdate`
- [ ] `$.legacy_pre_effect` / `$.user_effect` (S). Svelte 5: `$effect.pre` / `$effect`

### 7i — `createEventDispatcher`
- [ ] Runtime only, no compiler changes

---

## Architecture

### Arena-based AST with NodeStore trait

Replace tree-owned `Fragment { nodes: Vec<Node> }` with arena storage: all nodes live in a flat `Vec<Node>` on `Component`, fragments hold `Vec<NodeId>`. Access through `NodeStore` trait:

```rust
trait NodeStore {
    fn get(&self, id: NodeId) -> &Node;
    fn children(&self, id: NodeId) -> &[NodeId];
}
```

**Benefits:**
- O(1) node lookup by NodeId — any pass can get `&Node` from a side-table NodeId
- Cache-friendly sequential layout
- Simpler lifetime management (`&arena[id]` lives as long as `Component`)
- Walker, visitors, codegen work through `NodeStore` — migration is trait impl swap

**Unlocks:**
- `element_flags.rs`: attribute visitors get parent element name/attrs without state machines
- Validate pass: check cross-node relationships without tree traversal
- Post-resolve passes: work with NodeId from side tables and access full AST node
- Eliminates need for `element_name` field on VisitContext

**Scope:** parser + svelte_ast + all consumers (analyze, codegen, transform). Separate milestone.

## Deferred

### experimental.async (Tier 1.1)
- Function blocker analysis: deferred max-blocker tracking for function declarations

### $state rune — legacy (Tier 7)
- `$.deep_read_state()` for bindable props in `$:` reactive statements — only used in non-runes mode (LabeledStatement.js, shared/utils.js build_expression)

### Early bail on parser errors
- `compile()` currently runs analyze + codegen even when parser returned fatal errors
- Reference compiler throws on first error and never reaches analyze
- Should skip analyze/codegen if `diagnostics` contains `Severity::Error` after parsing
- Avoids panics on broken AST and gives cleaner error reporting
- Affects: `crates/svelte_compiler/src/lib.rs`

### Move `should_proxy` classification to analyze
- `should_proxy()` is called from codegen in 6+ sites to classify whether an expression needs `$.proxy()` wrapping
- This is classification logic that belongs in `svelte_analyze` per architecture boundaries (Rule 3)
- Should precompute `needs_proxy` flag in `AnalysisData` (per binding/field) so codegen just consumes it
- Affected files: `state.rs`, `traverse.rs`, `props.rs` in codegen; `rune_refs.rs` in transform; `lib.rs` in transform
