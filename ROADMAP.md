# Roadmap: Svelte 5 Client Compiler in Rust

Scope: client-side compilation only (no SSR, no legacy mode).

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
- [x] `$state.snapshot(val)` → `$.snapshot(val)`
- [x] `$derived` / `$derived.by` — `$.derived(() => expr)` / `$.derived(fn)`
- [x] `$props` rune (destructure, defaults, `$bindable`, rest, mutated)
- [x] `$effect(fn)` → `$.user_effect(fn)`, `$effect.pre(fn)` → `$.user_pre_effect(fn)`
- [x] `$effect.tracking()` → `$.effect_tracking()`
- [x] `$effect.root(fn)` → `$.effect_root(fn)`
- [x] `$store` auto-subscription → `$.store_get` / `$.store_set`
- [x] Import hoisting, strip TypeScript, exports (`$$.exports`)

### Template codegen
- [x] Element (with all attribute types), Component (props + children-as-snippet)
- [x] IfBlock, EachBlock, SnippetBlock, RenderTag
- [x] Text node, ExpressionTag
- [x] `{@html expr}` — raw HTML insertion
- [x] `{#key expr}` — keyed re-render block
- [x] `{@const x = expr}` — block-scoped constant (incl. destructuring)
- [x] `style:prop` directive (shorthand, expression, string, concat, `|important`)
- [x] `class` object/array syntax (Svelte 5)

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
- [x] `on:event` — legacy event directive (Svelte 4)

### Special elements
- [x] `<svelte:options>` — compiler options tag (parser + validation)
- [x] `<svelte:head>` — document head insertion

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

---

## Tier 1 — Remaining Runes

Theme: finish rune transformations. Purely script codegen (**S**), patterns already exist in `script.rs`.

Ref: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`, `ExpressionStatement.js`, `VariableDeclaration.js`
Key file: `crates/svelte_codegen_client/src/script.rs`

| # | Feature | Transform | Phases | Notes |
|---|---------|-----------|--------|-------|
| 1 | `$inspect(vals)` | `$.inspect(...)` | S | Dev-mode only — strip in prod. Needs `dev` compiler option |
| 2 | `$inspect.trace()` | dev-only trace | S | Same `dev` flag dependency |
| 3 | `$host()` | `$$props.$$host` | S | Expression replacement, for custom elements |
| 4 | `$state.eager(val)` | `$.state($.eager(val))` | S | Experimental async — forces immediate UI updates during `await`. Requires `experimental.async` flag |
| 5 | `$effect.pending()` | `$.effect_pending()` | S | Returns number of pending promises in current boundary. Used with `<svelte:boundary pending>` |
| 6 | `$props.id()` | `$$props.$$id` or inline | S | Generates unique, hydration-safe ID per component instance (v5.20+) |

---

## Tier 1b — Module Compilation (remaining)

| # | Item | Description |
|---|------|-------------|
| 1 | `ModuleCompileOptions` | Subset of `CompileOptions`: `dev`, `generate`, `filename`, `rootDir`. No `name`, `css`, `customElement`, `namespace` |
| 2 | Validation | Disallow `$props()`, `$bindable()` in modules. Disallow `$store` auto-subscriptions |

---

## Tier 2 — Remaining Template Blocks

### `{@debug vars}` — Dev-mode debugger
- **Phases**: P, T
- **AST**: `Node::DebugTag { id, span, identifiers: Vec<Span> }`
- **Parser**: Parse `{@debug x, y}`
- **Codegen**: `debugger` statement + `console.log` of variables (dev only). In prod: emit nothing
- **Dependency**: Same `dev` flag as `$inspect` (Tier 1)
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/DebugTag.js`

---

## Tier 5 — Special Elements (remaining)

Theme: `<svelte:*>` elements for global bindings, dynamic elements, error boundaries.

### `<svelte:element this={tag}>` — Dynamic element
- **Phases**: P, A, T
- **AST**: `Node::SvelteElement { id, span, tag_span, attributes, fragment }`
- **Codegen**: `$.element(anchor, () => tag, ($$anchor, element) => { ... })`
- **Notes**: Namespace inference with explicit `xmlns` control, void element validation
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/SvelteElement.js` (~161 lines)

### `<svelte:window>` — Window events & bindings
- **Phases**: P, A, T
- **Codegen**: Events → `$.event($.window, ...)`. Bindings → see table below
- **Constraint**: Top-level only, no children
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/SvelteWindow.js`

| Binding | Runtime |
|---------|---------|
| `bind:scrollX` | `$.bind_window_scroll("x", get, set)` |
| `bind:scrollY` | `$.bind_window_scroll("y", get, set)` |
| `bind:innerWidth` | `$.bind_window_size("innerWidth", set)` |
| `bind:innerHeight` | `$.bind_window_size("innerHeight", set)` |
| `bind:outerWidth` | `$.bind_window_size("outerWidth", set)` |
| `bind:outerHeight` | `$.bind_window_size("outerHeight", set)` |
| `bind:online` | `$.bind_online(set)` |
| `bind:devicePixelRatio` | `$.bind_window_size("devicePixelRatio", set)` |

### `<svelte:document>` — Document events & bindings
- **Phases**: P, A, T
- **Constraint**: Top-level only, no children
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/SvelteDocument.js`

| Binding | Runtime |
|---------|---------|
| `bind:activeElement` | `$.bind_active_element(set)` |
| `bind:fullscreenElement` | `$.bind_property(document, ...)` |
| `bind:pointerLockElement` | `$.bind_property(document, ...)` |
| `bind:visibilityState` | `$.bind_property(document, ...)` |

### `<svelte:body>` — Body events & actions
- **Phases**: P, A, T
- **Codegen**: Events → `$.event($.body, ...)`. Supports `use:action`.
- **Constraint**: Top-level only, no children
- **Deps**: `use:action` (done)
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/SvelteBody.js`

### `<svelte:boundary>` — Error boundary (Svelte 5.3+)
- **Phases**: P, A, T
- **AST**: `Node::SvelteBoundary { id, span, attributes, fragment }`
- **Snippets**: `failed` (receives error + reset), `pending` (initial loading)
- **Codegen**: `$.boundary(anchor, props, ($$anchor) => { ... })`
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/SvelteBoundary.js` (~126 lines)

### `<title>` — Special handling in `<svelte:head>`
- **Phases**: T
- **Codegen**: Special text update handling for `<title>` element content
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/TitleElement.js`

---

## Tier 6 — CSS Scoping

Theme: scoped CSS compilation — largest standalone workstream, new `svelte_css` subsystem.

| # | Feature | Phases | Description |
|---|---------|--------|-------------|
| 1 | Component CSS hash | A | Deterministic hash from source/filename, stored on `AnalysisData` |
| 2 | Scoped selector transformation | New module | Parse CSS, transform selectors to add `.svelte-HASH` suffix |
| 3 | `:global()` modifier | New module | Skip scoping for `:global(selector)` and `:global { ... }` blocks |
| 4 | CSS hash injection | T | Add `class="svelte-HASH"` to template elements that match scoped selectors |
| 5 | `--css-var={expr}` custom properties | P, T | Static: `$.set_style(el, "--var", value)`. Dynamic: `$.css_props(el, { "--var": value })` |
| 6 | Keyframe scoping | New module | Mangle `@keyframes name` → `@keyframes name-HASH`. `-global-` prefix skips scoping (e.g., `@keyframes -global-fade` → `@keyframes fade`) |
| 7 | CSS pruning/tree-shaking | New module | Remove rules whose selectors don't match any template element |
| 8 | Nested `<style>` elements | P | No scoping, emit as global rules |

---

## Tier 7 — Async, Validation & Optimization

Theme: less-used features, developer experience, performance improvements.

### Template features

| Feature | Phases | Description |
|---------|--------|-------------|
| `{#await promise}` | P, A, T | `$.await(anchor, () => promise, pending_fn, then_fn, catch_fn)`. AST: `Node::AwaitBlock`. Needs child scopes for `then`/`catch` bindings. Ref: `AwaitBlock.js` (~124 lines) |
| Await expressions (experimental) | P, A, T | `{await expr}` in templates. Svelte 5.36+, requires `experimental.async: true`. Ref: `AwaitExpression.js` |

### Validation (**V**)

| Feature | Description | Ref |
|---------|-------------|-----|
| Bind directive validation | Validate binding vs element compatibility (e.g., `bind:checked` only on checkbox/radio) | `2-analyze/visitors/BindDirective.js` |
| Assignment validation | Error on assignments to `const`, imports, `$derived` runes | `2-analyze/visitors/AssignmentExpression.js` |
| Rune argument validation | Validate rune call signatures (e.g., `$state()` takes 0-1 args) | `2-analyze/visitors/CallExpression.js` |
| Directive placement validation | `transition:` not on components, `animate:` only in keyed each | `2-analyze/visitors/Component.js` |
| A11y warnings | Missing `alt`, ARIA errors, form label association, etc. | `2-analyze/a11y.js` |
| `<!-- svelte-ignore -->` comments | Suppress specific compiler warnings for the next sibling node | `1-parse/` + `2-analyze/` |

### Optimization

| Feature | Phases | Description |
|---------|--------|-------------|
| Event delegation refinement | A, T | Refine `is_delegatable_event` analysis, track `$.delegate()` calls at component level |
| CSS hash injection | T | Add scoped class to elements (requires Tier 6) |

---

## Tier 8 — Legacy Svelte 4 (Lowest Priority)

Theme: deprecated syntax superseded by Svelte 5 features. Only needed for migrating codebases.

| Feature | Svelte 5 replacement | Transform | Phases |
|---------|----------------------|-----------|--------|
| `<slot>` + `let:` | `{#snippet}` + `{@render}` | `$.slot(...)` | P, A, T |
| `<svelte:component this={X}>` | `<X />` with capitalized variable | `$.component(...)` | P, A, T |
| `<svelte:self>` | Import component directly | Recursive ref | P, T |
| `<svelte:fragment>` | `{#snippet}` | Fragment wrapper | P, T |
| `export let` (props) | `$props()` | Different script transform | S |
| `$:` reactive assignments | `$derived` / `$effect` | Labeled statement → `$.derived`/`$.effect` | S |
| `$$props` / `$$restProps` / `$$slots` | `$props()` with rest | Runtime vars | S, T |
| `beforeUpdate` / `afterUpdate` | `$effect.pre` / `$effect` | `$.legacy_pre_effect` / `$.user_effect` | S |
| `createEventDispatcher` | Callback props | Runtime only, no compiler changes | — |

---

## Just Runtime (No Compiler Changes Needed)

These are imported from `svelte` and used as regular function calls. The compiler passes them through unchanged:

- **Lifecycle**: `onMount()`, `onDestroy()`
- **Scheduling**: `tick()`, `flushSync()`, `settled()`
- **Context**: `setContext()`, `getContext()`, `hasContext()`, `getAllContexts()`, `createContext()`
- **Mounting**: `mount()`, `unmount()`, `hydrate()`
- **Utilities**: `untrack()`, `createRawSnippet()`, `getAbortSignal()`, `fork()` (experimental async)
- **Stores**: `writable()`, `readable()`, `derived()`, `readonly()`, `get()` — from `svelte/store`
- **Motion**: `Spring`, `Tween` (v5.8+ class-based), `tweened()`, `spring()` (deprecated) — from `svelte/motion`
- **Easing**: `linear`, `cubicInOut`, `elasticOut`, etc. — from `svelte/easing`
- **Transitions**: `fade`, `fly`, `slide`, `scale`, `blur`, `draw`, `crossfade` — from `svelte/transition` (runtime; compiler only needs directive support)
- **Animation**: `flip` — from `svelte/animate`
- **Events**: `on()` — from `svelte/events` (programmatic event attachment)
- **Attachments**: `createAttachmentKey()`, `fromAction()` — from `svelte/attachments`
- **Reactive collections**: `SvelteMap`, `SvelteSet`, `SvelteDate`, `SvelteURL`, `SvelteURLSearchParams`, `MediaQuery`, `createSubscriber` — from `svelte/reactivity`
- **Reactive window**: `innerWidth`, `innerHeight`, `scrollX`, `scrollY`, `online`, `devicePixelRatio` — from `svelte/reactivity/window` (reactive window property accessors)

---

## Architectural Notes

- **OXC** — JS expression parsing/scoping, only `Span` stored in AST
- **Side tables** (`AnalysisData`) — no AST mutations
- **Analyze**: composite visitor (tuple `TemplateVisitor`) — single tree walk for all passes
- **Codegen**: direct recursion, no visitor pattern
- **Scope system NOT needed** for Tiers 1-5 (runes mode). Current approach (OXC + side tables) is sufficient
- Each feature: test case → expected output via reference compiler → `cargo test`

---

## Deferred

Items discovered during porting but not critical for the feature to work. Grouped by parent feature.

### Void elements (Tier 0)
- [ ] Validation: emit error if void element has children (requires parser-level check for content between void open tags)

### Runes (Tier 1)
- [ ] `$state` / `$state.raw` destructuring support in script codegen
- [ ] `$state` / `$state.raw` class field support
- [ ] `$state.frozen` → `$state.raw` rename validation

### Module compilation (Tier 1b)
- [ ] `ModuleCompileOptions` type — subset of `CompileOptions`
- [ ] Validation: disallow `$props()`, `$bindable()`, `$store` auto-subscriptions in modules

### `{@html expr}` (Tier 2)
- [ ] `is_controlled` optimization (single child → innerHTML)
- [ ] `is_svg` / `is_mathml` namespace flags

### `{@const}` (Tier 2)
- [ ] Dev mode `$.tag()` wrapping
- [ ] Placement validation

### Bind directives (Tier 3)
- [ ] `bind:property={get, set}` — function bindings (Svelte 5)
- [ ] Window bindings (`scrollX`, `scrollY`, `innerWidth`, etc.) — blocked on `<svelte:window>` (Tier 5)
- [ ] Document bindings (`activeElement`, `fullscreenElement`, etc.) — blocked on `<svelte:document>` (Tier 5)

### `use:action` (Tier 4)
- [ ] `use:action` with `await` expression (requires `run_after_blockers`)

### Transitions (Tier 4)
- [ ] Validation: duplicate transition directives on same element
- [ ] Validation: conflicting `transition:` + `in:`/`out:` on same element

### Animate (Tier 4)
- [ ] Validation: `animate:` only valid inside keyed `{#each}` blocks
- [ ] Validation: duplicate `animate:` directives on same element

### `{@attach}` (Tier 4)
- [ ] `{@attach}` on component nodes — generates `$.attachment()` property in props
- [ ] `{@attach}` with async/blockers — `$.run_after_blockers()` wrapping

### `<svelte:options>` (Tier 5)
- [ ] `customElement` object form: full parsing of `tag`, `shadow`, `props`, `extend` properties (expression span stored, analysis-phase parsing needed)
- [ ] `namespace` affecting codegen: `$.from_svg()` / `$.from_mathml()` instead of `$.from_html()`

### `<svelte:head>` (Tier 5)
- [ ] Validation: only allowed at root level
- [ ] Validation: no attributes allowed (diagnostic)
- [ ] `filename` parameter for `compile()` to produce correct hash (currently uses `"(unknown)"` default)

### `on:directive` legacy (Tier 8)
- [ ] Call memoization: `on:click={getHandler()}` → `$.derived(() => getHandler())` + `$.get()`. Needs `ExpressionMetadata.has_call` in analysis
- [ ] SvelteDocument/SvelteWindow/SvelteBody routing: events on special elements should go to `init` not `after_update`. Blocked on Tier 5
- [ ] Dev-mode `$.apply()` wrapping for imported identifier handlers. Blocked on `dev` compiler option
