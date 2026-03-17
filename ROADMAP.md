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

### Analyze (9 passes, composite visitor)
- [x] `parse_js` — JS expression parsing, rune detection
- [x] `scope` — OXC scoping (script + template)
- [x] `mutations` — rune mutation tracking
- [x] `known_values` — static const evaluation
- [x] `props` — `$props()` destructuring ($bindable, defaults, rest)
- [x] `lower` — whitespace trim, adjacent text+expr merge
- [x] `reactivity` + `elseif` — dynamic node/attribute marking
- [x] `content_types` — fragment classification (single-element, text-only, etc.)
- [x] `needs_var` — elements needing JS variables

### Script codegen
- [x] `$state` rune (read, assign, update, `$.proxy()`)
- [x] `$derived` / `$derived.by` — `$.derived(() => expr)` / `$.derived(fn)`
- [x] `$props` rune (destructure, defaults, `$bindable`, rest, mutated)
- [x] Import hoisting
- [x] Strip TypeScript
- [x] Exports (`$$.exports`)
- [x] `$effect(fn)` → `$.user_effect(fn)`, `$effect.pre(fn)` → `$.user_pre_effect(fn)`

### Template codegen
- [x] Element (with all attribute types)
- [x] Component (props + children-as-snippet)
- [x] IfBlock, EachBlock
- [x] SnippetBlock, RenderTag
- [x] Text node, ExpressionTag

### Bind directives (Tier 3)
- [x] `bind:value` (input, textarea, select), `bind:checked`, `bind:group`, `bind:files`
- [x] `bind:indeterminate`, `bind:open` (generic `$.bind_property`)
- [x] `bind:innerHTML`, `bind:innerText`, `bind:textContent` (contenteditable)
- [x] `bind:clientWidth/Height`, `bind:offsetWidth/Height` (element size)
- [x] `bind:contentRect`, `bind:contentBoxSize`, `bind:borderBoxSize`, `bind:devicePixelContentBoxSize` (resize observer)
- [x] `bind:currentTime`, `bind:paused`, `bind:volume`, `bind:muted`, `bind:playbackRate` (media R/W)
- [x] `bind:buffered`, `bind:seekable`, `bind:seeking`, `bind:ended`, `bind:readyState`, `bind:played` (media RO)
- [x] `bind:duration`, `bind:videoWidth`, `bind:videoHeight`, `bind:naturalWidth`, `bind:naturalHeight` (event-based RO)
- [x] `bind:this` (element reference)
- [x] `bind:focused`
- [ ] `bind:property={get, set}` — function bindings (Svelte 5) *(deferred)*
- [ ] Window bindings: `scrollX`, `scrollY`, `innerWidth`, `innerHeight`, `outerWidth`, `outerHeight`, `online`, `devicePixelRatio` *(requires `<svelte:window>` parser)*
- [ ] Document bindings: `activeElement`, `fullscreenElement`, `pointerLockElement`, `visibilityState` *(requires `<svelte:document>` parser)*

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

## Tier 0 — Parser Fundamentals ✅

Theme: critical parser capabilities needed for correct HTML handling. Without these, common valid HTML fails to parse.

### ~~Void (self-closing) HTML elements~~ ✅
- **Phases**: P, V
- **Work items**:
  1. [x] Add `VOID_ELEMENTS` constant and `is_void(name)` helper
  2. [x] In scanner/parser: auto-set `self_closing: true` when tag name is void (even without `/>`)
  3. [x] Validation: emit error on `</input>` and similar (closing tag for void element) — `void_element_invalid_content`
  4. [ ] Validation: emit error if void element has children (deferred — requires parser-level check for content between void open tags)

---

## Tier 1 — Complete Rune Coverage

Theme: finish all rune transformations. Purely script codegen (**S**), patterns already exist in `script.rs`.

Ref: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`, `ExpressionStatement.js`, `VariableDeclaration.js`
Key file: `crates/svelte_codegen_client/src/script.rs`

| # | Feature | Transform | Phases | Notes |
|---|---------|-----------|--------|-------|
| 1 | ~~`$effect(fn)`~~ | `$.user_effect(fn)` | S | ✅ Done |
| 2 | ~~`$effect.pre(fn)`~~ | `$.user_pre_effect(fn)` | S | ✅ Done |
| 3 | ~~`$state.raw(val)`~~ | `$.state(val)` (no `$.proxy()`) | S | ✅ Done. Deferred: destructuring (shared gap with `$state`), class fields, `$state.frozen` rename validation |
| 4 | ~~`$state.snapshot(val)`~~ | `$.snapshot(val)` | S | ✅ Done |
| 5 | ~~`$effect.tracking()`~~ | `$.effect_tracking()` | S | ✅ Done. Trivial call rewrite, no args |
| 6 | ~~`$effect.root(fn)`~~ | `$.effect_root(fn)` | S | ✅ Done. Simple callee rewrite |
| 7 | `$inspect(vals)` | `$.inspect(...)` | S | Dev-mode only — strip in prod. Needs `dev` compiler option |
| 8 | `$inspect.trace()` | dev-only trace | S | Same `dev` flag dependency |
| 9 | `$host()` | `$$props.$$host` | S | Expression replacement, for custom elements |
| 10 | `$state.eager(val)` | `$.state($.eager(val))` | S | Experimental async — forces immediate UI updates during `await`. Requires `experimental.async` flag |
| 11 | `$effect.pending()` | `$.effect_pending()` | S | Returns number of pending promises in current boundary. Used with `<svelte:boundary pending>` |
| 12 | `$props.id()` | `$$props.$$id` or inline | S | Generates unique, hydration-safe ID per component instance (v5.20+) |
| 13 | ~~`$store` auto-subscription~~ | ~~`$.store_get`/`$.store_set` with scope analysis~~ | ~~S, A~~ | ✅ Done — read, assign, compound assign, update (++/--) in both template and script |

---

## Tier 1b — Module Compilation (`.svelte.js` / `.svelte.ts`)

Theme: separate entry point for compiling rune-enabled JS/TS modules (no template, no CSS).

In Svelte, the bundler plugin detects `.svelte.js`/`.svelte.ts` extensions and calls `compileModule()` instead of `compile()`. The compiler itself does not inspect filenames — it exposes two distinct functions.

Ref: `reference/compiler/index.js` (`compileModule`), `reference/compiler/phases/2-analyze/index.js` (`analyze_module`), `reference/compiler/phases/3-transform/client/transform-client.js` (`client_module`)

### Pipeline

```
source (JS/TS) → analyze_module() → client_module() → JS output
```

### Comparison with `compile()`

| Aspect | `compile()` | `compileModule()` |
|--------|-------------|-------------------|
| Input | `.svelte` (HTML + Script + Style) | JS/TS only |
| Runes mode | Inferred or forced | Always `runes: true` |
| CSS output | Yes | `null` |
| Template codegen | Yes | No |
| Output shape | Component class (default export) | Plain JS module |

### Work items

| # | Item | Description |
|---|------|-------------|
| 1 | ~~`compile_module()` entry point~~ | ~~New public function: takes JS/TS source + `ModuleCompileOptions`, returns `CompileResult`~~ ✅ Done |
| 2 | ~~`analyze_module()`~~ | ~~Simplified analysis: OXC parse → scopes → rune detection. No template, no props, no content_types. Hardcode `runes: true`~~ ✅ Done |
| 3 | ~~Script transforms reuse~~ | ~~Apply existing `script.rs` rune transformations ($state, $derived, $effect, etc.) to module AST~~ ✅ Done |
| 4 | `ModuleCompileOptions` | Subset of `CompileOptions`: `dev`, `generate`, `filename`, `rootDir`. No `name`, `css`, `customElement`, `namespace` |
| 5 | Validation | Disallow `$props()`, `$bindable()` in modules. Disallow `$store` auto-subscriptions |
| 6 | ~~WASM export~~ | ~~Expose `compileModule()` alongside existing `compile()` in WASM build~~ ✅ Done |

### Dependencies
- Tier 1 rune transforms (reused, not duplicated)

---

## Tier 2 — Essential Template Blocks

Theme: most commonly needed template features. Requires parser + AST + analyze + codegen.

Key files: `svelte_ast/src/lib.rs`, `svelte_parser/src/lib.rs`, `svelte_codegen_client/src/template/`

### ~~`{@html expr}`~~ — Raw HTML insertion ✅
- **Phases**: P, A, T
- **AST**: `Node::HtmlTag { id, span, expression_span }`
- **Parser**: Handle `{@html ...}` in tag scanner (similar to `{@render}`)
- **Analyze**: Register expression in `parse_js`. Mark dynamic in `reactivity`. Handle in `content_types`
- **Codegen**: `$.html(anchor, () => expr)`
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js` (~60 lines)
- **Not yet**: `is_controlled` optimization (single child → innerHTML), `is_svg`/`is_mathml` namespace flags

### ~~`{#key expr}` — Keyed re-render block~~ ✅
- **Phases**: P, A, T
- **AST**: `Node::KeyBlock { id, span, expression_span, fragment }`
- **Parser**: Parse `{#key expr}...{/key}`
- **Analyze**: Add `FragmentKey::KeyBody(NodeId)`. Process in `lower`, `reactivity`, `content_types`
- **Codegen**: `$.key(anchor, () => expr, ($$anchor) => { ... })`
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/KeyBlock.js` (~45 lines)

### ~~`{@const x = expr}` — Block-scoped constant~~ ✅
- **Phases**: P, A, T
- **AST**: `Node::ConstTag { id, span, declaration_span }`
- **Parser**: Parse `{@const ...}` extracting variable declaration
- **Analyze**: Scope integration — const binding visible in subsequent template nodes within same block
- **Transform**: Destructuring → generates tmp var (`$$const_0`), rewrites aliases to `$.get(tmp).prop`
- **Codegen**: Simple: `const x = $.derived(() => expr)`. Destructured: `const $$const_0 = $.derived(() => expr)` + aliases
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/ConstTag.js`
- **Deferred**: dev mode `$.tag()`, placement validation

### ~~`style:prop={value}` — Style directive~~ ✅
- **Phases**: P, A, T
- **AST**: `Attribute::StyleDirective { name, value: StyleDirectiveValue, important: bool }` with `StyleDirectiveValue` enum (Shorthand, Expression, String, Concatenation)
- **Parser**: Parse `style:color={expr}`, `style:color` (shorthand), `style:color="red"` (string), `style:color="red-{x}"` (concat), `|important` modifier
- **Codegen**: `$.set_style(el, staticStyle, prev, { directives })` — same pattern as `$.set_class()`. `|important` produces `[{ normal }, { important }]` array format.
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js`

### ~~`class` attribute — Object/array syntax (Svelte 5)~~ ✅
- **Phases**: P, A, T
- **Syntax**: `class={{ active: isActive, bold }}`, `class={[base, isActive && "active", variant]}`
- **Parser**: Detect object/array expression in `class` attribute value
- **Codegen**: `$.set_class(el, ...)` — merges object keys where value is truthy, filters falsy array items
- **Notes**: Preferred over `class:name` directive in Svelte 5. Coexists with static `class="..."` and `class:` directives
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js`

### `{@debug vars}` — Dev-mode debugger
- **Phases**: P, T
- **AST**: `Node::DebugTag { id, span, identifiers: Vec<Span> }`
- **Parser**: Parse `{@debug x, y}`
- **Codegen**: `debugger` statement + `console.log` of variables (dev only). In prod: emit nothing
- **Dependency**: Same `dev` flag as `$inspect` (Tier 1)
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/DebugTag.js`

---

## Tier 3 — Bind Directive Completeness ✅

Theme: parser/AST already supports `bind:name={expr}`. Need element-aware codegen dispatch per binding type.

**Status**: Codegen complete for all regular-element bindings. Window/document bindings deferred to Tier 4 (requires `<svelte:window>` / `<svelte:document>` parser support).

Key file: `crates/svelte_codegen_client/src/template/attributes.rs`
Ref: `reference/compiler/phases/3-transform/client/visitors/BindDirective.js`

### Element reference

| Binding | Elements | Runtime | Phases |
|---------|----------|---------|--------|
| `bind:this` | any element or component | `$.bind_this(el, ($$value) => ref = $$value, () => ref)` | T, A |

Note: `bind:this` uses a different pattern — NOT getter/setter, uses `build_bind_this` utility.

### Function bindings (Svelte 5)

`bind:property={get, set}` — accepts a getter/setter pair for custom validation/transformation during binding updates.
- **Syntax**: `bind:value={getValue, setValue}`
- **Codegen**: Same runtime calls, but getter/setter are user-provided functions instead of generated ones
- **Notes**: Works with any bindable property. Enables custom logic (clamping, formatting) on binding updates

### Input/Form bindings

| Binding | Elements | Runtime | Phases |
|---------|----------|---------|--------|
| `bind:value` | `<input>`, `<textarea>` | `$.bind_value(el, get, set)` | T |
| `bind:value` | `<select>` | `$.bind_select_value(el, get, set)` | T |
| `bind:value` | `<select multiple>` | `$.bind_select_value(el, get, set)` (array) | T |
| `bind:checked` | `<input type="checkbox">` | `$.bind_checked(el, get, set)` | T |
| `bind:checked` | `<input type="radio">` | `$.bind_checked(el, get, set)` | T |
| `bind:indeterminate` | `<input type="checkbox">` | `$.bind_property(el, "indeterminate", "change", get, set)` | T |
| `bind:group` | `<input type="checkbox">` | `$.bind_group(group_arr, el, get, set)` | T |
| `bind:group` | `<input type="radio">` | `$.bind_group(group_arr, el, get, set)` | T |
| `bind:files` | `<input type="file">` | `$.bind_files(el, get, set)` | T |

### Details element

| Binding | Elements | Runtime | Phases |
|---------|----------|---------|--------|
| `bind:open` | `<details>` | `$.bind_property(el, "open", "toggle", get, set)` | T |

### Contenteditable bindings

| Binding | Elements | Runtime | Phases |
|---------|----------|---------|--------|
| `bind:innerHTML` | `[contenteditable]` | `$.bind_content_editable(el, get, set, "innerHTML")` | T |
| `bind:innerText` | `[contenteditable]` | `$.bind_content_editable(el, get, set, "innerText")` | T |
| `bind:textContent` | `[contenteditable]` | `$.bind_content_editable(el, get, set, "textContent")` | T |

### Dimension bindings (all readonly, all visible elements)

| Binding | Runtime | Phases |
|---------|---------|--------|
| `bind:clientWidth` | `$.bind_resize_observer(el, "client", set)` | T |
| `bind:clientHeight` | `$.bind_resize_observer(el, "client", set)` | T |
| `bind:offsetWidth` | `$.bind_element_size(el, "offset", set)` | T |
| `bind:offsetHeight` | `$.bind_element_size(el, "offset", set)` | T |
| `bind:contentRect` | `$.bind_resize_observer(el, "contentRect", set)` | T |
| `bind:contentBoxSize` | `$.bind_resize_observer(el, "contentBoxSize", set)` | T |
| `bind:borderBoxSize` | `$.bind_resize_observer(el, "borderBoxSize", set)` | T |
| `bind:devicePixelContentBoxSize` | `$.bind_resize_observer(el, "devicePixelContentBoxSize", set)` | T |

### Media bindings (`<audio>`, `<video>`)

| Binding | R/W | Runtime | Phases |
|---------|-----|---------|--------|
| `bind:currentTime` | R/W | `$.bind_current_time(el, get, set)` | T |
| `bind:playbackRate` | R/W | `$.bind_playback_rate(el, get, set)` | T |
| `bind:paused` | R/W | `$.bind_paused(el, get, set)` | T |
| `bind:volume` | R/W | `$.bind_volume(el, get, set)` | T |
| `bind:muted` | R/W | `$.bind_muted(el, get, set)` | T |
| `bind:duration` | RO | `$.bind_property(el, "duration", "durationchange", set)` | T |
| `bind:buffered` | RO | `$.bind_buffered(el, set)` | T |
| `bind:seekable` | RO | `$.bind_seekable(el, set)` | T |
| `bind:seeking` | RO | `$.bind_seeking(el, set)` | T |
| `bind:ended` | RO | `$.bind_ended(el, set)` | T |
| `bind:readyState` | RO | `$.bind_ready_state(el, set)` | T |
| `bind:played` | RO | `$.bind_played(el, set)` | T |
| `bind:videoWidth` | RO | `$.bind_property(el, "videoWidth", "resize", set)` | T |
| `bind:videoHeight` | RO | `$.bind_property(el, "videoHeight", "resize", set)` | T |

### Image bindings (readonly)

| Binding | Elements | Runtime | Phases |
|---------|----------|---------|--------|
| `bind:naturalWidth` | `<img>` | `$.bind_property(el, "naturalWidth", "load", set)` | T |
| `bind:naturalHeight` | `<img>` | `$.bind_property(el, "naturalHeight", "load", set)` | T |

---

## Tier 4 — Directives & Interactivity

Theme: action, transition, animation directives. New AST attribute variants + parser + codegen.

### ~~`use:action={params}` — Action directive~~ ✅
- **Phases**: P, A, T
- **AST**: `Attribute::UseDirective { name, expression_span: Option<Span> }`
- **Codegen**: `$.action(el, () => actionFn, () => params)`
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/UseDirective.js` (~30 lines)
- [ ] `use:action` with `await` expression (requires `run_after_blockers`) *(discovered during port, deferred)*

### `transition:name={params}` / `in:` / `out:` — Transitions [x]
- **Phases**: P, A, T
- **AST**: `Attribute::TransitionDirective { name, expression_span: Option<Span>, modifiers: Vec<String>, direction: TransitionDirection }`
  - `TransitionDirection`: `Both` | `In` | `Out`
- **Modifiers**: `|local` (scoped to block), `|global` (default)
- **Codegen**:
  - `transition:fade` → `$.transition(flags, el, () => fade, () => params)`
  - `in:fly` → `$.transition(TRANSITION_IN, el, () => fly)`
  - `out:slide` → `$.transition(TRANSITION_OUT, el, () => slide)`
- **Events**: Elements get `introstart`, `introend`, `outrostart`, `outroend` events
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/TransitionDirective.js`
- [ ] Validation: duplicate transition directives *(discovered during port, deferred)*
- [ ] Validation: conflicting transition: + in:/out: on same element *(discovered during port, deferred)*

### `animate:name={params}` — FLIP animations [x]
- **Phases**: P, A, T
- **AST**: `Attribute::AnimateDirective { name, expression_span: Option<Span> }`
- **Constraint**: Only valid inside keyed `{#each}` blocks (validation needed)
- **Codegen**: `$.animation(el, animateFn, () => params)`
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/AnimateDirective.js`
- [ ] Validation: `animate:` only valid inside keyed `{#each}` blocks *(discovered during port, deferred)*
- [ ] Validation: duplicate `animate:` directives on same element *(discovered during port, deferred)*

### ~~`{@attach fn}` — Element attachment (Svelte 5.29+)~~ ✅
- **Phases**: P, A, T
- **AST**: `Attribute::AttachTag { expression_span }` (in element attributes array)
- **Codegen**: `$.attach(el, () => fn)`
- **Notes**: Modern alternative to `use:action`. Re-runs on reactive dependency changes. Conditional with falsy values.
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/AttachTag.js`
- [ ] `{@attach}` on component nodes — generates `$.attachment()` property in props *(deferred)*
- [ ] `{@attach}` with async/blockers — `$.run_after_blockers()` wrapping *(deferred)*

---

## Tier 5 — Special Elements

Theme: `<svelte:*>` elements for global bindings, dynamic elements, head management, error boundaries.

### ~~`<svelte:options>` — Compiler options tag~~ ✅
- **Phases**: P only (no codegen)
- **Attributes**: `runes={true|false}`, `namespace="html"|"svg"|"mathml"`, `customElement="tag-name"`, `css="injected"`, `preserveWhitespace`, `immutable`, `accessors`
- **Notes**: Parse early, store on component metadata. Scanner extended for `svelte:*` tag names. Validation for unknown attrs, invalid values, no children, tag names.
- [ ] `customElement` object form: full parsing of `tag`, `shadow`, `props`, `extend` properties *(deferred — expression span stored, analysis-phase parsing needed)*
- [ ] `namespace` affecting codegen: `$.from_svg()` / `$.from_mathml()` instead of `$.from_html()` *(deferred — requires codegen changes)*

### `<svelte:head>` — Document head
- **Phases**: P, A, T
- **AST**: `Node::SvelteHead { id, span, fragment }`
- **Codegen**: `$.head(($$anchor) => { ... })`
- **Constraint**: Top-level only
- **Ref**: `reference/compiler/phases/3-transform/client/visitors/SvelteHead.js`

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
- **Deps**: `use:action` (Tier 4)
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
| ~~Unmutated rune optimization~~ | ~~S~~ | ~~Done — moved to Done section~~ |
| CSS hash injection | T | Add scoped class to elements (requires Tier 6) |

---

## Tier 8 — Legacy Svelte 4 (Lowest Priority)

Theme: deprecated syntax superseded by Svelte 5 features. Only needed for migrating codebases.

| Feature | Svelte 5 replacement | Transform | Phases |
|---------|----------------------|-----------|--------|
| ~~`on:event={handler}` + modifiers~~ | ~~`onclick={handler}` (already works)~~ | ~~`$.event(el, "click", handler, modifiers)`~~ | ~~P, A, T~~ | ✅ Done |
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

## Edge Cases / Tech Debt

Deferred items from completed features. Each links back to the originating tier.

### on:directive (Tier 8)
- [ ] Call memoization: `on:click={getHandler()}` → `$.derived(() => getHandler())` + `$.get()`. Needs `ExpressionMetadata.has_call` in analysis. Rare in Svelte 4 code.
- [ ] SvelteDocument/SvelteWindow/SvelteBody routing: events on these special elements should go to `init` not `after_update`. Blocked on Tier 5 (special elements).
- [ ] Dev-mode `$.apply()` wrapping for imported identifier handlers. Blocked on `dev` compiler option.
