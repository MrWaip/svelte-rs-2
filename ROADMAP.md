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

### Event handling
- [x] Svelte 5 event attributes — `onclick={handler}` → `$.delegated()` for delegatable events
- [x] Event delegation — `$.delegate([...events])` at component level
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
- [x] `<svelte:window>` — window events (`on:`, `onscroll`), bindings (`scrollX/Y`, `innerWidth/Height`, `outerWidth/Height`, `online`, `devicePixelRatio`)
- [x] `<svelte:document>` — document events (`on:`, `onkeydown`), bindings (`activeElement`, `fullscreenElement`, `pointerLockElement`, `visibilityState`)
- [x] `<svelte:body>` — body events (`on:`, `onclick`), actions (`use:action`)

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
| 4 | `$props.id()` | `$$props.$$id` or inline | S | Generates unique, hydration-safe ID per component instance (v5.20+) |

---

## Tier 1b — Module Compilation (remaining)

| # | Item | Description |
|---|------|-------------|
| 1 | `ModuleCompileOptions` | Subset of `CompileOptions`: `dev`, `generate`, `filename`, `rootDir`. No `name`, `css`, `customElement`, `namespace` |
| 2 | Validation | Disallow `$props()`, `$bindable()` in modules. Disallow `$store` auto-subscriptions |

---

## Tier 1c — Event Attributes (fix + extend)

Theme: complete the Svelte 5 event system. `onclick={handler}` is the standard way; `on:click` is legacy.

**Current state**: delegatable events (click, input, change, etc.) correctly generate `$.delegated()`. Non-delegatable events (scroll, resize, etc.) incorrectly fall through to `$.set_attribute()`.

Key file: `crates/svelte_codegen_client/src/template/attributes.rs`
Ref: `reference/compiler/phases/3-transform/client/visitors/shared/events.js`, `Attribute.js`

| # | Feature | Current | Correct | Phases |
|---|---------|---------|---------|--------|
| 1 | Non-delegatable event attrs | `$.set_attribute("onscroll", fn)` | `$.event("scroll", el, fn)` | T |
| 2 | Event capture suffix | Not handled | `onclickcapture` → `capture: true` flag | A, T |
| 3 | Passive event auto-detection | Not handled | Auto-passive for `touchstart`, `wheel`, etc. | T |
| 4 | Handler wrapping (non-inline) | Not handled | `(...$$args) => handler.apply(this, $$args)` | T |

---

## Tier 1d — Expression Memoization

Theme: prevent over-firing of reactive expressions containing function calls.

The reference compiler's `Memoizer` class wraps expressions with `has_call` in `$.derived()` to memoize them. Without this, expressions like `onclick={getHandler()}` or `{@render fn(getArg())}` re-evaluate on every render cycle.

Ref: `reference/compiler/phases/3-transform/client/visitors/shared/utils.js` (Memoizer class)

| # | Feature | Phases | Description |
|---|---------|--------|-------------|
| 1 | `has_call` detection in analysis | A | Track whether expression contains `CallExpression` |
| 2 | Memoizer codegen utility | T | Generate `$.derived(() => expr)` + `$.get(id)` pairs |
| 3 | Integration: event handlers | T | Memoize `onclick={getHandler()}` |
| 4 | Integration: component props | T | Memoize non-simple prop expressions with calls |
| 5 | Integration: render tag args | T | Memoize `{@render fn(getArg())}` |

---

## Tier 2 — Remaining Template Blocks

### `{#await promise}` — Async blocks
- **Phases**: P, A, T
- **AST**: `Node::AwaitBlock { id, span, expression_span, pending, then_binding, then, catch_binding, catch }`
- **Parser**: `{#await expr}...{:then val}...{:catch err}...{/await}`, short form `{#await expr then val}...{/await}`
- **Codegen**: `$.await(anchor, () => promise, pending_fn, then_fn, catch_fn)`
- **Notes**: Needs child scopes for `then`/`catch` bindings. Common in real apps (data fetching).
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/AwaitBlock.js` (~124 lines)

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

### ~~`<svelte:element this={tag}>` — Dynamic element~~ ✅
- **Phases**: P, A, T
- **AST**: `Node::SvelteElement { id, span, tag_span, attributes, fragment }`
- **Codegen**: `$.element(anchor, () => tag, ($$anchor, element) => { ... })`
- **Notes**: Namespace inference with explicit `xmlns` control, void element validation
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/SvelteElement.js` (~161 lines)
- [ ] `svelte:element` inside `{#if}` block *(deferred)*
- [ ] `svelte:element` with `class:` directives *(deferred)*
- [ ] `svelte:element` with `style:` directives *(deferred)*

### ~~`<svelte:window>` — Window events & bindings~~ ✅
- **Phases**: P, A, T
- **Codegen**: Events → `$.event($.window, ...)`. Bindings → `$.bind_window_scroll`, `$.bind_window_size`, `$.bind_online`, `$.bind_property`
- **Constraint**: Top-level only, no children
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/SvelteWindow.js`

### ~~`<svelte:document>` — Document events & bindings~~ ✅
- **Phases**: P, A, T
- **Constraint**: Top-level only, no children
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/SvelteDocument.js`

### ~~`<svelte:body>` — Body events & actions~~ ✅
- **Phases**: P, A, T
- **Codegen**: Events → `$.event($.document.body, ...)`. Supports `use:action`.
- **Constraint**: Top-level only, no children

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

### Component `bind:this`
- **Phases**: T
- **Codegen**: `$.bind_this(component, setter, getter)` — different from element `bind:this`, binds to component instance
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/shared/component.js`

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

## Tier 7 — Validation & Diagnostics

Theme: developer experience — errors, warnings, and diagnostic infrastructure.

### Validation (**V**)

| Feature | Description | Ref |
|---------|-------------|-----|
| Bind directive validation | Validate binding vs element compatibility (e.g., `bind:checked` only on checkbox/radio) | `2-analyze/visitors/BindDirective.js` |
| Assignment validation | Error on assignments to `const`, imports, `$derived` runes | `2-analyze/visitors/AssignmentExpression.js` |
| Rune argument validation | Validate rune call signatures (e.g., `$state()` takes 0-1 args) | `2-analyze/visitors/CallExpression.js` |
| Directive placement validation | `transition:` not on components, `animate:` only in keyed each | `2-analyze/visitors/Component.js` |

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

### Ownership validation (dev mode)
- `$.create_ownership_validator($$props)` — detect invalid mutations of bound props
- `svelte-ignore ownership_invalid_binding` suppression
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/shared/component.js`

---

## Tier 8 — Compiler Infrastructure

Theme: compiler options, source maps, dev mode support.

### `CompileOptions` structure

The reference compiler accepts these options (we currently accept only `source`):

| Option | Type | Description |
|--------|------|-------------|
| `dev` | `boolean` | Enable runtime checks, `$inspect`, `{@debug}`, ownership validation, `$.apply()` wrapping |
| `css` | `'injected' \| 'external'` | CSS handling mode |
| `generate` | `'client' \| 'server' \| false` | Output target |
| `filename` | `string` | Source filename for diagnostics and CSS hash |
| `rootDir` | `string` | Root directory for relative paths |
| `name` | `string` | Component class name |
| `namespace` | `'html' \| 'svg' \| 'mathml'` | Element namespace |
| `runes` | `boolean \| undefined` | Force runes mode on/off |
| `preserveComments` | `boolean` | Keep HTML comments in output |
| `preserveWhitespace` | `boolean` | Keep whitespace as typed |
| `discloseVersion` | `boolean` | Expose Svelte version in `window.__svelte.v` |
| `hmr` | `boolean` | Hot module replacement support |
| `sourcemap` | `object` | Initial source map for preprocessing |
| `warningFilter` | `function` | Custom warning filter |

### Source maps
- JS source map generation (via magic-string in reference, needs equivalent in Rust)
- CSS source map generation
- Merged preprocessor source maps

### Dev mode (`dev: true`)
Gates these features:
- `$inspect()` / `$inspect.trace()` rune transforms
- `{@debug}` tag codegen
- `$.apply()` wrapping for better stack traces
- Ownership validation (`$.create_ownership_validator`)
- Snippet wrapping (`$.wrap_snippet`)
- Component naming for devtools

---

## Tier 9 — Custom Elements

Theme: Web Component compilation — alternative output format.

- **`customElement` option** — compile component as custom element with shadow DOM
- **`$.create_custom_element()`** — wrapper for component function
- **Shadow DOM config** — `{ mode: 'open' }`, `'none'`, or custom
- **Props metadata** — `attribute`, `reflect`, `type` for each prop
- **`extend` option** — class inheritance for custom element
- **`customElements.define(tag, element)`** call generation
- **Ref**: `reference/compiler/phases/3-transform/client/transform-client.js` lines 598–677

---

## Tier 10 — Legacy Svelte 4 (Lowest Priority)

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
- [ ] `$state.eager(val)` — experimental async, requires `experimental.async` flag
- [ ] `$effect.pending()` — requires `<svelte:boundary>` (Tier 5)
- [ ] `$inspect().with(callback)` — custom inspection callback

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

### `<svelte:window>` (Tier 5)
- [ ] Validation: only allowed at root level (not nested)
- [ ] Validation: no children allowed (diagnostic)
- [ ] Validation: no spread attributes, only event/bind directives
- [ ] Validation: only one `<svelte:window>` per component

### `<svelte:document>` (Tier 5)
- [ ] Validation: only allowed at root level (not nested)
- [ ] Validation: no children allowed (diagnostic)
- [ ] Validation: no spread attributes, only event/bind directives
- [ ] Validation: only one `<svelte:document>` per component

### `<svelte:head>` (Tier 5)
- [ ] Validation: only allowed at root level
- [ ] Validation: no attributes allowed (diagnostic)
- [ ] `filename` parameter for `compile()` to produce correct hash (currently uses `"(unknown)"` default)

### Render tag
- [ ] Optional chaining: `{@render fn?.()}` → `$.noop` fallback when fn is nullish

### Event attributes (Tier 1c)
- [ ] Handler memoization for expressions with calls (`has_call`) — depends on Tier 1d memoization

### CSS (Tier 6)
- [ ] Component CSS custom properties on `<Component>` — `$.css_props()` wrapper element injection

### Compiler infrastructure (Tier 8)
- [ ] HMR support — `$.hmr()` wrapper, `import.meta.hot.accept()`
- [ ] `fragments: 'tree'` option — alternative DOM fragment strategy
- [ ] `{await expr}` experimental template syntax (Svelte 5.36+, requires `experimental.async`)

### `<svelte:body>` (Tier 5)
- [ ] Validation: only event attributes and directives allowed (reject non-event attrs, spreads)
- [ ] Validation: no children allowed (`disallow_children`)
- [ ] Validation: only allowed at root level (not nested)
- [ ] Validation: only one `<svelte:body>` per component

### `on:directive` legacy (Tier 10)
- [ ] Call memoization: `on:click={getHandler()}` → `$.derived(() => getHandler())` + `$.get()`. Needs `ExpressionMetadata.has_call` in analysis
- [ ] SvelteDocument/SvelteBody routing: events on special elements should go to `init` not `after_update`. Blocked on Tier 5
- [ ] Dev-mode `$.apply()` wrapping for imported identifier handlers. Blocked on `dev` compiler option
