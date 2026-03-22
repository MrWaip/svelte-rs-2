# Roadmap: Svelte 5 Client Compiler in Rust

Scope: client-side compilation only (no SSR, no legacy mode).
For a full feature parity audit, see [PARITY.md](PARITY.md).

**Phase notation**: **P** = Parser/AST, **A** = Analyze, **S** = Script codegen, **T** = Template codegen, **V** = Validation

---

## Done ✅

### AST & Parser
- [x] `Text`, `Element`, `ComponentNode`, `Comment`
- [x] `ExpressionTag` — `{expr}`
- [x] `IfBlock`, `EachBlock`, `SnippetBlock`, `RenderTag`
- [x] Attributes: string, expression, boolean, concatenation, shorthand/spread, `class:`, `bind:`
- [x] Script/Style blocks, TypeScript support
- [x] Void (self-closing) HTML elements — `VOID_ELEMENTS`, auto `self_closing`, closing tag validation

### Analyze (12 passes, composite visitor)
- [x] `js_analyze` — expression analysis from pre-parsed OXC ASTs (JS parsing done in `svelte_parser`)
- [x] `build_scoping` — unified scope tree (script + template)
- [x] `register_arrow_scopes` — arrow function scope registration
- [x] `resolve_references` — template refs → SymbolId, mutation tracking
- [x] `store_subscriptions` — `$store` subscription detection
- [x] `known_values` — static const evaluation
- [x] `props` — `$props()` destructuring ($bindable, defaults, rest)
- [x] `lower` — whitespace trim, adjacent text+expr merge
- [x] `reactivity` + `elseif` + `element_flags` + `hoistable_snippets` + `bind_semantics` — composite walk (5 visitors)
- [x] `classify_and_mark_dynamic` — fragment classification (single-element, text-only, etc.)
- [x] `needs_var` — elements needing JS variables
- [x] `validate` — semantic checks

### Script codegen
- [x] `$state` rune (read, assign, update, `$.proxy()`)
- [x] `$state.raw(val)` → `$.state(val)` (no proxy)
- [x] `$state` / `$state.raw` destructuring — object, array, defaults, rest, nested patterns
- [x] `$state` / `$state.raw` class fields — public, private, constructor, multiple fields
- [x] `$state.snapshot(val)` → `$.snapshot(val)`
- [x] `$derived` / `$derived.by` — `$.derived(() => expr)` / `$.derived(fn)`
- [x] `$props` rune (destructure, defaults, `$bindable`, rest, mutated)
- [x] `$effect(fn)` → `$.user_effect(fn)`, `$effect.pre(fn)` → `$.user_pre_effect(fn)`
- [x] `$effect.tracking()` → `$.effect_tracking()`
- [x] `$effect.root(fn)` → `$.effect_root(fn)`
- [x] `$store` auto-subscription → `$.store_get` / `$.store_set`
- [x] Import hoisting, exports (`$$.exports`)
- [x] `$inspect(vals)` → `$.inspect(...)` (dev-mode only)
- [x] `$inspect.trace()` — dev-only trace
- [x] `$props.id()` → `$.props_id()` (v5.20+)
- [x] `$host()` → `$$props.$$host` (custom element host reference)
- [x] `customElements.define()` — basic custom element wrapping (simple tag form)

### Template codegen
- [x] Element (with all attribute types), Component (props + children-as-snippet)
- [x] IfBlock, EachBlock, SnippetBlock, RenderTag
- [x] Text node, ExpressionTag
- [x] `{@html expr}` — raw HTML insertion
- [x] `{#key expr}` — keyed re-render block
- [x] `{@const x = expr}` — block-scoped constant (incl. destructuring)
- [x] `style:prop` directive (shorthand, expression, string, concat, `|important`)
- [x] `class` object/array syntax (Svelte 5)
- [x] `{#await promise}` — async blocks (full form, short then/catch, no bindings, destructured, pending only)
- [x] `{@debug vars}` — dev-mode debugger (all contexts: root, if, each)

### Event handling
- [x] Svelte 5 event attributes — `onclick={handler}` → `$.delegated()` for delegatable events
- [x] Event delegation — `$.delegate([...events])` at component level
- [x] Non-delegatable events — `onscroll={h}` → `$.event("scroll", el, h)`
- [x] Capture suffix — `onclickcapture={h}` → `$.event("click", el, h, true)`
- [x] Passive auto-detection — `ontouchstart={h}` → auto `passive: true` for touch events
- [x] Handler wrapping — imported identifiers and member expressions wrapped in `function(...$$args) { h?.apply(this, $$args) }`
- [x] `has_call` memoization — `onclick={getHandler()}` → `$.derived(getHandler)` + `$.get(event_handler)`
- [x] Component prop memoization — `has_call` and non-simple+dynamic expressions wrapped in `$.derived`/`$.get`
- [x] Render tag arg memoization — `{@render fn(getArg())}` → `$.derived`/`$.get`
- [x] Render tag dynamic callee — `{@render show(args)}` where `show` is prop/state → `$.snippet(node, () => show, ...)`
- [x] Render tag optional chaining — `{@render fn?.()}` → `fn?.(anchor)` or `$.snippet(node, () => fn ?? $.noop, ...)`
- [x] `on:event` — legacy event directive (Svelte 4)

### Bind directives
- [x] `bind:value` (input, textarea, select), `bind:checked`, `bind:group`, `bind:files`
- [x] `bind:indeterminate`, `bind:open` (generic `$.bind_property`)
- [x] `bind:innerHTML`, `bind:innerText`, `bind:textContent` (contenteditable)
- [x] `bind:clientWidth/Height`, `bind:offsetWidth/Height` (element size)
- [x] `bind:contentRect`, `bind:contentBoxSize`, `bind:borderBoxSize`, `bind:devicePixelContentBoxSize` (resize observer)
- [x] `bind:currentTime`, `bind:paused`, `bind:volume`, `bind:muted`, `bind:playbackRate` (media R/W)
- [x] `bind:buffered`, `bind:seekable`, `bind:seeking`, `bind:ended`, `bind:readyState`, `bind:played` (media RO)
- [x] `bind:duration`, `bind:videoWidth`, `bind:videoHeight`, `bind:naturalWidth`, `bind:naturalHeight` (event-based RO)
- [x] `bind:this` (element reference), `bind:focused`

### Directives
- [x] `use:action={params}` — action directive
- [x] `transition:` / `in:` / `out:` — transitions (local/global, params)
- [x] `animate:name={params}` — FLIP animations
- [x] `{@attach fn}` — element attachment (Svelte 5.29+)

### Special elements
- [x] `<svelte:options>` — compiler options tag (parser + validation)
- [x] `<svelte:head>` — document head insertion
- [x] `<svelte:window>` — window events, bindings (`scrollX/Y`, `innerWidth/Height`, `outerWidth/Height`, `online`, `devicePixelRatio`)
- [x] `<svelte:document>` — document events, bindings (`activeElement`, `fullscreenElement`, `pointerLockElement`, `visibilityState`)
- [x] `<svelte:body>` — body events, actions (`use:action`)
- [x] `<svelte:element this={tag}>` — dynamic element (`$.element()`)
- [x] `<svelte:boundary>` — error boundary (Svelte 5.3+, `$.boundary()`)
- [x] `<title>` in `<svelte:head>` — `$.document.title` with effect wrapping
- [x] Component `bind:this` — `$.bind_this(component, setter, getter)`

### Module compilation
- [x] `compile_module()` entry point + `analyze_module()` + WASM export

### Optimizations
- [x] Whitespace trimming
- [x] Merge adjacent text/interpolation
- [x] First-node-is-text optimization
- [x] Single-element optimization
- [x] Text-and-interpolation-only optimization
- [x] Non-reactive attribute optimization
- [x] Unmutated rune optimization — skip `$.state()` wrapper for `$state` that's never assigned

### WASM
- [x] Compiler compiled to WASM for browser use

### Custom Elements (Tier 9)
- [x] `customElement` compile option — forces all props to prop sources with getter/setter exports
- [x] `$.create_custom_element()` — full argument generation (props metadata, slots, accessors, shadow config, extend)
- [x] Props metadata — auto-populated from `$props()` destructuring, explicit config with `attribute`, `reflect`, `type`
- [x] Shadow DOM config — `"open"` (default) and `"none"` (omit shadow root)
- [x] `extend` option — class inheritance for custom element
- [x] Object form parsing — `customElement={{ tag, shadow, props, extend }}` expression span re-parsed in codegen
- [x] Tag-less registration — no `customElements.define()`, just `$.create_custom_element()` call
- [x] Accessors — exported names populate accessors array

---

---

## Tier 1 — Core Gaps

### 1a — `ModuleCompileOptions` ✅
- [x] `ModuleCompileOptions` — subset of `CompileOptions`: `dev`, `generate`, `filename`, `rootDir`. No `name`, `css`, `customElement`, `namespace`
- [x] `compile_module` wires options through pipeline: `dev` → analyze/codegen, `generate: false` → analysis-only, `filename` → TypeScript detection

### 1b — Template expression transforms (`svelte_transform`) ✅

- [x] `$store = val` in template — `$.store_set(store, val)`
- [x] `$store += val` in template — `$.store_set(store, $store() + val)`
- [x] `$store++` / `$store--` in template — `$.update_store()` / `$.update_pre_store()`
- [x] `$store.field = val` in template — `$.store_mutate()`
- [x] `$store.count++` in template — `$.store_mutate()` (deep update)
- [x] Rune compound assignment in template — `state_var += val` → `$.set(name, $.get(name) + val)`
- [x] Rune logical compound — `state_var &&= other` → `$.set(name, $.get(name) && other, true)`
- [x] `$store` deep mutation in script — `$.store_mutate(store, mutation, $.untrack(store))`
- [x] `$.push/$.pop` for components with deep store mutations (`needs_context` detection)
- [x] `collect_references` fix: walk member chains in UpdateExpression/AssignmentExpression targets

### 1c - Style attribute not completed
- [ ] `style={{a: 123}}`

---

## Tier 1.1 — Experimental Async (`experimental.async`)

Theme: all features gated behind `experimental.async`. Requires analysis infrastructure (`ExpressionInfo.has_await`, `has_blockers()`, blocker tracking).

### Infrastructure
- [ ] `ExpressionInfo.has_await` — detect `await` in expression metadata
- [ ] `has_blockers()` — analysis infrastructure for dependency tracking
- [ ] Full blocker tracking: const tags with async expressions → `binding.blocker` propagation

### Template blocks
- [ ] `{#await}` — `has_blockers` / `$.async()` wrapping for experimental async mode
- [ ] `$.async()` wrapping for if/each/html/key blocks with `has_await` expressions
- [ ] `{await expr}` experimental template syntax (Svelte 5.36+)

### Bind directives
- [ ] `$.run_after_blockers()` wrapping for async bind expressions

### Actions, attachments & transitions
- [ ] `use:action` with `await` expression — `run_after_blockers`
- [ ] `{@attach}` with async/blockers — `$.run_after_blockers()` wrapping
- [ ] `transition:` async/blockers — `$.run_after_blockers()` wrapping for transitions with async expressions
- [ ] `animate:` async/blockers — `$.run_after_blockers()` wrapping for animations with async expressions

### Special elements
- [ ] `<svelte:boundary>` — `experimental.async` handling for const tag scoping changes

---

## Tier 2 — Remaining Edge Cases

Edge cases and missing features discovered during porting. Grouped by feature area.

### 2a — Runes & script
- [ ] Custom element `$.push`/`$.pop` lifecycle for `$host()` mutations

### 2b — Template tags
- [x] `{@html}` — `is_controlled` optimization (single child → innerHTML)
- [x] `{@html}` — `is_svg` / `is_mathml` namespace flags
- [x] `{@const}` — dev mode `$.tag()` wrapping + eager `$.get()` for init errors
- [x] `{@debug}` — works in if/each contexts with proper `$.get()` wrapping for each-block vars
- [x] `{#await}` — array destructuring in then/catch bindings (e.g., `{:then [a, b]}`)
- [x] `$.add_svelte_meta()` — dev-mode block wrapping for if/each/await/key blocks
- [ ] `{#await}` — dev-mode `$.apply()` wrapping for await expression
- [ ] `{#snippet}` — parameter destructuring: array/object patterns with defaults → per-field `$.derived()` wrappers

### 2c — Bind directives
- [x] `bind:property={get, set}` — function bindings (Svelte 5) ✅
- [x] `bind:group` — value attribute `__value` pattern for elements with `bind:group` ✅
- [x] `bind:group` — value attribute dependency: wrap getter to include value expression ✅
- [x] Bind directive deferral with `use:` — wrap non-`bind:this` directives in `$.effect()` when parent has `UseDirective` ✅
- [x] `contenteditable` detection — `bound_contenteditable` flag affecting text update behavior in fragment codegen ✅
- [x] `$state(array/object)` — wrap inner value in `$.proxy()` for mutated $state signals ✅
- [x] `bind:group` — index array from `parent_each_blocks` when expression references each-block vars ✅

### 2d — Actions & attachments
- [ ] `{@attach}` on component nodes — generates `$.attachment()` property in props

### 2e — Special elements
- [x] `<svelte:options>` — `namespace` affecting codegen: `$.from_svg()` / `$.from_mathml()` instead of `$.from_html()`
- [x] `<svelte:element>` inside `{#if}` block
- [x] `<svelte:element>` with `class:` directives
- [x] `<svelte:element>` with `style:` directives
- [x] `<svelte:head>` — `filename` parameter for correct hash (already correct)
- [x] `<svelte:boundary>` — `@const` duplication into hoisted snippets
- [x] `<svelte:boundary>` — import reactivity: imported identifiers in boundary attrs generate getters
- [ ] `<svelte:boundary>` — dev mode: snippet wrapping with `$.wrap_snippet`
- [ ] `<svelte:boundary>` — handler wrapping for snippet params as event handlers
- [ ] `<svelte:element>` — dynamic `xmlns` attribute for runtime namespace switching
- [x] `bind:this` — SequenceExpression custom getter/setter

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
- [x] `__value` property — hidden internal property for `<input type="checkbox">` with `bind:group` ✅ (option/select not yet)
- [ ] `$.init_select(el)` — initialize select element for value tracking
- [ ] `$.select_option(el, value)` — sync select option when value changes dynamically
- [ ] `customizable_select` — rich HTML content in `<select>`/`<option>`/`<optgroup>` → `$.customizable_select()` wrapper

Ref: `RegularElement.js` lines 166–202, 470–725

### 2l — Event replay
- [ ] `$.replay_events(el)` — re-trigger queued load/error events for `<img>`, `<video>`, `<audio>`, `<source>`, etc. when element has spread, `use:`, or `onload`/`onerror` attribute

Ref: `RegularElement.js` lines 280–284

### 2m — EachBlock edge cases
- [x] Fallback (`{:else}`) codegen — 6th argument to `$.each()`: `($$anchor) => fallback_fragment`
- [x] `EACH_INDEX_REACTIVE` flag (value 2) — set when keyed block has index variable; index becomes signal, read via `$.get(i)`
- [x] Key function index parameter — include index in key arrow `(item, i) => key_expr` when key expression references the index variable
- [x] Destructuring context — `{#each items as { x, y }}` → per-field `$.derived_safe_equal()` wrappers inside render function
- [x] Destructuring array context — `{#each items as [a, b]}` → `$.to_array` + `$.derived` intermediate + per-element thunks
- [x] Destructuring with defaults — `{#each items as { x = 5 }}` → `$.derived_safe_equal(() => $.fallback(...))`
- [x] `key_is_item` optimization — when key expression equals context identifier, skip `EACH_ITEM_REACTIVE` and `$.get()` wrapping
- [x] `body_uses_index` optimization — index param only emitted in render fn when body actually uses it
- [ ] Collection ID (scope shadowing) — when context variable shadows outer scope binding, store array in `$$array` and pass as extra render_fn arg
- [ ] Store invalidation — `$.invalidate_store($$stores, 'name')` when collection expression uses `$store` subscription
- [x] Flag refinement: `EACH_ITEM_REACTIVE` — runes mode: skip when `key_is_item` (no store deps)
- [x] Flag refinement: `EACH_ITEM_IMMUTABLE` — always set in runes mode (store detection deferred to legacy mode)
- [x] `bind:group` + keyed each: existing tests pass (5 bind_group+each tests)
- [x] Nested each blocks: `bind:group` expression propagates through multiple levels (existing tests pass)

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

### 5a — Infrastructure setup

Текущее состояние: `svelte_diagnostics` имеет ~25 error-вариантов в `DiagnosticKind`, severity только Error, нет warnings. `validate()` в `svelte_analyze` — пустая заглушка.

- [ ] Extend `DiagnosticKind` with warning variants (~39 кодов в reference: `warnings.js`)
- [ ] Parameterized messages — поддержка `%placeholder%` подстановки в сообщениях
- [ ] `<!-- svelte-ignore -->` parsing — извлечение кодов из HTML-комментариев (runes: comma-separated, legacy: space-separated)
- [ ] Legacy code migration map (e.g., `empty-block` → `block_empty`)
- [ ] Ignore stack + ignore map — `push_ignore(codes)` / `pop_ignore()` / `is_ignored(node, code)` threading через analysis walk
- [ ] Warning filter — поддержка `warningFilter` из `CompileOptions`
- [ ] Unused selector warnings — `css-warn.js` pattern (зависит от Tier 4d)

Ref: `reference/compiler/warnings.js` (~39 codes), `reference/compiler/state.js` (ignore stack), `reference/compiler/utils/extract_svelte_ignore.js`

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

### `CompileOptions` structure ✅

`CompileOptions` and `ModuleCompileOptions` types defined in `svelte_compiler::options`. Piped through pipeline; `component_name()` derives name from `filename`. Behavioral changes (dev mode, css injection, etc.) are deferred.

### 6a — `discloseVersion` option
- [ ] `import {} from 'svelte/internal/disclose-version'` when `discloseVersion: true`

### 6b — `preserveComments` option
- [ ] Keep HTML comments in template output (`push_comment()` in fragment codegen)

### 6c — Dev: `$.tag()` / `$.tag_proxy()` rune tagging
- [ ] `$state` — `$.tag($.proxy(val), "name")` / `$.tag($.state(val), "name")` in VariableDeclaration
- [ ] `$derived` — `$.tag($.derived(...), "name")` in VariableDeclaration
- [ ] Class fields — `$.tag(val, "Class.field")` in ClassBody, AssignmentExpression
- [ ] `$bindable` — `$.tag_proxy($.proxy(val), "name")` for bindable props
- [ ] Snippets — `$.tag(snip, "snippetName")` in SnippetBlock
- [ ] `{@const}` — `$.tag()` wrapping in ConstTag

Unblocks: remaining `{@debug}` tests (`$.tag_proxy`/`$.get(item)`)

### 6d — Dev: strict equality transforms
- [ ] `===` / `!==` → `$.strict_equals(left, right)` / `$.strict_equals(left, right, false)`
- [ ] `==` / `!=` → `$.equals(left, right)` / `$.equals(left, right, false)`

Single visitor: BinaryExpression

### 6e — Dev: `$.apply()` + event handler naming
- [ ] Arrow→named function: `(e) => handler` → `function click() { ... }` for event handlers
- [ ] `$.apply()` wrapping: `$.apply(thunk, this, args, ComponentName, [line, col])` for location tracking

Ref: `reference/compiler/phases/3-transform/client/visitors/shared/events.js`

### 6f — Dev: `$.add_svelte_meta()` block wrapping
- [x] IfBlock — `$.add_svelte_meta(() => $.if(...), 'if', ComponentName, line, col)`
- [x] EachBlock — same pattern
- [x] AwaitBlock — same pattern
- [x] KeyBlock — same pattern

Implemented in Tier 2b. Ref: `reference/compiler/phases/3-transform/client/visitors/shared/utils.js`

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

## Deferred
