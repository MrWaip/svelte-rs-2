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

### Analyze (11 passes, composite visitor)
- [x] `parse_js` — JS expression parsing, rune detection
- [x] `build_scoping` — unified scope tree (script + template)
- [x] `resolve_references` — template refs → SymbolId, mutation tracking
- [x] `store_subscriptions` — `$store` subscription detection
- [x] `known_values` — static const evaluation
- [x] `props` — `$props()` destructuring ($bindable, defaults, rest)
- [x] `lower` — whitespace trim, adjacent text+expr merge
- [x] `reactivity` + `elseif` + `element_flags` + `hoistable_snippets` — composite walk (4 visitors)
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
- [x] `{@debug vars}` — dev-mode debugger (partial: 3/5 tests, 2 blocked on `$.add_svelte_meta`/`$.tag_proxy`)

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

## Tier 1 — Module Compilation (remaining)

- [ ] `ModuleCompileOptions` — subset of `CompileOptions`: `dev`, `generate`, `filename`, `rootDir`. No `name`, `css`, `customElement`, `namespace`

---

## Tier 2 — Strip TypeScript

Theme: remove TypeScript type annotations from compiled output.

- [ ] ~~Strip type annotations~~ (S) — ~~remove type annotations, interfaces, type aliases from `<script>` output~~
- [ ] ~~Strip from expressions~~ (T) — ~~remove type assertions, `as` casts, generics from template expressions~~

---

## Tier 3 — Remaining Edge Cases

Edge cases and missing features discovered during porting. Grouped by feature area.

### Runes & script
- [ ] Custom element `$.push`/`$.pop` lifecycle for `$host()` mutations

### Template tags
- [ ] `{@html}` — `is_controlled` optimization (single child → innerHTML)
- [ ] `{@html}` — `is_svg` / `is_mathml` namespace flags
- [ ] `{@const}` — dev mode `$.tag()` wrapping
- [ ] `{@debug}` — `debug_in_if`, `debug_in_each` tests: blocked on `$.add_svelte_meta` + `$.tag_proxy`/`$.get(item)`
- [ ] `{#await}` — `has_blockers` / `$.async()` wrapping for experimental async mode
- [ ] `{#await}` — dev-mode `$.apply()` wrapping for await expression
- [ ] `{#await}` — array destructuring in then/catch bindings (e.g., `{:then [a, b]}`)

### Bind directives
- [ ] `bind:property={get, set}` — function bindings (Svelte 5)

### Actions & attachments
- [ ] `use:action` with `await` expression (requires `run_after_blockers`)
- [ ] `{@attach}` on component nodes — generates `$.attachment()` property in props
- [ ] `{@attach}` with async/blockers — `$.run_after_blockers()` wrapping

### Special elements
- [ ] `<svelte:options>` — `namespace` affecting codegen: `$.from_svg()` / `$.from_mathml()` instead of `$.from_html()`
- [ ] `<svelte:element>` inside `{#if}` block
- [ ] `<svelte:element>` with `class:` directives
- [ ] `<svelte:element>` with `style:` directives
- [ ] `<svelte:head>` — `filename` parameter for correct hash (currently `"(unknown)"`)
- [ ] `<svelte:boundary>` — `@const` duplication into hoisted snippets
- [ ] `<svelte:boundary>` — import reactivity: imported identifiers in boundary attrs should generate getters (`has_state`)
- [ ] `<svelte:boundary>` — `experimental.async` handling for const tag scoping changes
- [ ] `<svelte:boundary>` — dev mode: snippet wrapping with `$.wrap_snippet`
- [ ] `<svelte:boundary>` — handler wrapping for snippet params as event handlers
- [ ] `bind:this` — SequenceExpression custom getter/setter (rarely used)

### CSS
- [ ] Component CSS custom properties on `<Component>` — `$.css_props()` wrapper element injection

### Compiler infrastructure
- [ ] `fragments: 'tree'` option — alternative DOM fragment strategy
- [ ] `{await expr}` experimental template syntax (Svelte 5.36+, requires `experimental.async`)

### Custom Elements
- [ ] HMR conditional registration: `if (customElements.get(tag) == null)`
- [ ] Shadow DOM custom `ObjectExpression` (non-literal config)
- [ ] `$.push`/`$.pop` lifecycle for `$host()` mutations (reference compiler bug — see GOTCHAS.md #9)
- [ ] Auto-detect boolean type from prop default literal value (in CE props config)

### Legacy `on:directive`
- [ ] Call memoization: `on:click={getHandler()}` → `$.derived(() => getHandler())` + `$.get()`
- [ ] SvelteDocument/SvelteBody routing: events on special elements → `init` not `after_update`
- [ ] Dev-mode `$.apply()` wrapping for imported identifier handlers

---

## Tier 4 — CSS Scoping

Theme: scoped CSS compilation — largest standalone workstream, new `svelte_css` subsystem.

Pipeline: parse → hash → analyze → prune → transform → inject into template.
Ref: `1-parse/read/style.js`, `2-analyze/css/`, `3-transform/css/index.js`

### 4.0 — Research: выбор CSS-стека
- [ ] Оценить варианты: OXC css parser (`oxc_css`), `lightningcss`, `cssparser` (Servo), свой парсер
- [ ] Критерии: полнота CSS3 selectors, `:global()` / nesting (`&`), доступ к AST для мутаций (scoping, pruning), source map support, размер зависимости, lifetime ergonomics
- [ ] Проверить можно ли переиспользовать существующий парсер и надстроить metadata enrichment, или проще свой мини-парсер (Svelte парсит только selectors + at-rules, declarations хранит как строки)
- [ ] Решение зафиксировать в ADR или комментарием в этом разделе

### 4a — CSS AST & parser (new crate `svelte_css`)
- [ ] CSS AST types: `StyleSheet`, `Rule`, `Atrule`, `SelectorList`, `ComplexSelector`, `RelativeSelector`, `SimpleSelector` variants (type, class, id, attribute, pseudo-class, pseudo-element, combinator, nesting `&`)
- [ ] Parse `<style>` content into CSS AST — selectors, declarations, at-rules (`@media`, `@keyframes`, etc.)
- [ ] Nested rules support (CSS nesting with `&`)

Ref: `1-parse/read/style.js` (~638 lines), `types/css.d.ts` (~201 lines)

### 4b — CSS hash computation
- [ ] Deterministic hash from `filename` (preferred) or CSS content (fallback)
- [ ] Format: `svelte-{hash}` (e.g., `svelte-a1b2c3d4`)
- [ ] Store on `AnalysisData.css.hash`
- [ ] Support custom `cssHash` option

Ref: `2-analyze/index.js` lines 536–548, `validate-options.js` line 73

### 4c — CSS analysis: global/local classification
- [ ] Walk CSS AST, enrich selector metadata: `is_global`, `is_global_like`
- [ ] `:global(selector)` — contents unscoped
- [ ] `:global { ... }` — entire block unscoped
- [ ] `:global` bare (no args) — everything after is unscoped
- [ ] `:global` in pseudo-classes (`:is()`, `:has()`, `:where()`, `:not()`)
- [ ] `is_global_like` for `:root`, `:host`, `::view-transition-*`
- [ ] Collect keyframe names (skip `-global-` prefixed)
- [ ] Validation: invalid `:global()` placement

Ref: `2-analyze/css/css-analyze.js` (~332 lines)

### 4d — CSS pruning: selector → template matching
- [ ] Backward selector matching against template elements
- [ ] Combinator traversal: descendant (` `), child (`>`), adjacent (`+`), general sibling (`~`)
- [ ] Mark `element.metadata.scoped = true` for matched elements
- [ ] Mark `selector.metadata.used = true` for matched selectors
- [ ] Handle `:has()`, `:not()`, `:is()`, `:where()` argument matching
- [ ] Conservative matching for components and snippets

Ref: `2-analyze/css/css-prune.js` (~1248 lines) — самый большой файл в CSS pipeline

### 4e — CSS transformation: scoping & output
- [ ] Append `.svelte-HASH` class to scoped selectors (first bump, rest via `:where()`)
- [ ] Remove `:global()` / `:global` syntax from output
- [ ] Scope `@keyframes name` → `@keyframes svelte-HASH-name`
- [ ] Patch `animation` / `animation-name` property values
- [ ] Prune unused rules (comment out in dev, remove in prod)
- [ ] Unwrap `:global { ... }` blocks
- [ ] Minification in prod mode (whitespace removal)
- [ ] CSS source map generation

Ref: `3-transform/css/index.js` (~480 lines)

### 4f — Template hash injection
- [ ] For scoped elements, add `class="svelte-HASH"` in template codegen
- [ ] Pass hash to `$.attribute_effect()` runtime call
- [ ] `css: 'injected'` — embed CSS in JS, runtime injects `<style>` tag
- [ ] `css: 'external'` — extract CSS to separate file

Ref: `3-transform/client/visitors/shared/element.js` lines 93–95

### 4g — CSS custom properties
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

### Runes & script

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
- [ ] Assignment to `const`, imports, `$derived` runes. Ref: `2-analyze/visitors/AssignmentExpression.js`
- [ ] Module: disallow `$props()`, `$bindable()`, `$store` auto-subscriptions

### Elements & special elements

- [ ] Void elements: error if void element has children
- [ ] `<svelte:window>` — only at root level, no children, no spread attrs, only one per component
- [ ] `<svelte:document>` — only at root level, no children, no spread attrs, only one per component
- [ ] `<svelte:body>` — only event attrs/directives, no children, only at root level, only one per component
- [ ] `<svelte:head>` — only at root level, no attributes allowed
- [ ] `<title>` in `<svelte:head>` — no attributes (`title_illegal_attribute`), children must be Text or ExpressionTag only (`title_invalid_content`)
- [ ] `<svelte:boundary>` — reject non-`onerror`/`failed`/`pending` attrs, reject string/boolean values
- [ ] `custom_element_props_identifier` warning when `$props()` used without CE props config

### Directives

- [ ] `bind:` vs element compatibility (e.g., `bind:checked` only on checkbox/radio). Ref: `2-analyze/visitors/BindDirective.js`
- [ ] `transition:` not on components. Ref: `2-analyze/visitors/Component.js`
- [ ] `transition:` duplicate directives on same element
- [ ] `transition:` + `in:`/`out:` conflicting on same element
- [ ] `animate:` only inside keyed `{#each}` blocks
- [ ] `animate:` duplicate directives on same element

### Template blocks

- [ ] `{@const}` placement validation
- [ ] `{#await}` duplicate `{:then}` or `{:catch}` clauses

### A11y warnings (~40 checks)
- Missing `alt` on `<img>`, `<area>`, `<input type="image">`
- ARIA attribute validation (`role`, `aria-*` correctness)
- Form label association (`<label>` + `for`/`id`)
- Keyboard event pairing (`onclick` → needs `onkeydown`)
- Heading hierarchy (`<h1>`–`<h6>` order)
- Interactive role focus management
- Media caption requirements
- Redundant/conflicting attributes
- **Ref**: `reference/compiler/phases/2-analyze/visitors/shared/a11y/` (~954 lines)

### `<!-- svelte-ignore -->` comments
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
- [ ] IfBlock — `$.add_svelte_meta(() => $.if(...), 'if', ComponentName, line, col)`
- [ ] EachBlock — same pattern
- [ ] AwaitBlock — same pattern

Unblocks: remaining `{@debug}` tests (`$.add_svelte_meta` wrapping).
Ref: `reference/compiler/phases/3-transform/client/visitors/shared/utils.js`

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

- [ ] `<slot>` + `let:` (P, A, T) — `$.slot(...)`. Svelte 5: `{#snippet}` + `{@render}`
- [ ] `<svelte:component this={X}>` (P, A, T) — `$.component(...)`. Svelte 5: `<X />` with capitalized variable
- [ ] `<svelte:self>` (P, T) — recursive ref. Svelte 5: import component directly
- [ ] `<svelte:fragment>` (P, T) — fragment wrapper. Svelte 5: `{#snippet}`
- [ ] `export let` props (S) — different script transform. Svelte 5: `$props()`
- [ ] `$:` reactive assignments (S) — labeled statement → `$.derived`/`$.effect`. Svelte 5: `$derived` / `$effect`
- [ ] `$$props` / `$$restProps` / `$$slots` (S, T) — runtime vars. Svelte 5: `$props()` with rest
- [ ] `beforeUpdate` / `afterUpdate` (S) — `$.legacy_pre_effect` / `$.user_effect`. Svelte 5: `$effect.pre` / `$effect`
- [ ] `createEventDispatcher` — runtime only, no compiler changes
