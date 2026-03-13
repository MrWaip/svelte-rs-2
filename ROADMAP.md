# Roadmap: Svelte 5 Client Compiler in Rust

Scope: client-side compilation only (no SSR, no legacy mode).
Current work items: see `TODO.md`.

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
- [x] `$props` rune (destructure, defaults, `$bindable`, rest, mutated)
- [x] Import hoisting
- [x] Strip TypeScript
- [x] Exports (`$$.exports`)

### Template codegen
- [x] Element (with all attribute types)
- [x] Component (props + children-as-snippet)
- [x] IfBlock, EachBlock
- [x] SnippetBlock, RenderTag
- [x] Text node, ExpressionTag

### Optimizations
- [x] Whitespace trimming
- [x] Merge adjacent text/interpolation
- [x] First-node-is-text optimization
- [x] Single-element optimization
- [x] Text-and-interpolation-only optimization
- [x] Non-reactive attribute optimization

### WASM
- [x] Compiler compiled to WASM for browser use

---

## Priority 1 — Rune codegen

Essential runes beyond `$state` and `$props`. Purely script codegen changes.

- [x] **`$derived` / `$derived.by`** — `$.derived(() => expr)` / `$.derived(fn)`
  - Need: `detect_rune()` member expression support, `enter_variable_declarator` State/Derived distinction
  - Ref: `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`
  - Runtime: `$.derived()`

- [ ] **`$effect` / `$effect.pre`** — `$.user_effect(fn)` / `$.user_pre_effect(fn)`
  - Need: `detect_rune()` member expressions, new handler for `$effect()` expression statements
  - Ref: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`, `ExpressionStatement.js`
  - Runtime: `$.user_effect()`, `$.user_pre_effect()`

- [ ] **`$inspect`** — `$.inspect(fn)` (dev mode only)
  - Need: expression statement handler, dev-mode conditional
  - Ref: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`
  - Runtime: `$.inspect()`

- [ ] **`$state.raw`** — non-proxied state (skip `$.proxy()`)
  - Need: `detect_rune()` member expression for `$state.raw`
  - Ref: `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`
  - Runtime: `$.state()` (no proxy wrapping)

---

## Priority 2 — Essential template features

Most-used template features after the core set.

- [ ] **`{@html expr}`** — raw HTML insertion
  - Need: AST (`Node::HtmlTag`), parser (`{@html ...}`), template codegen
  - Ref: `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js`
  - Runtime: `$.html()`

- [ ] **`{#key expr}`** — keyed re-render block
  - Need: AST (`Node::KeyBlock`), parser (`{#key ...}{/key}`), analyze, template codegen
  - Ref: `reference/compiler/phases/3-transform/client/visitors/KeyBlock.js`
  - Runtime: `$.key()`

- [ ] **`style:prop={value}`** — style directive
  - Need: AST (`StyleDirective`), parser, attributes codegen
  - Ref: `reference/compiler/phases/3-transform/client/visitors/shared/element.js`
  - Runtime: `$.set_style()`

- [ ] **`use:action={params}`** — action directive
  - Need: AST (`UseDirective`), parser, template codegen
  - Ref: `reference/compiler/phases/3-transform/client/visitors/UseDirective.js`
  - Runtime: `$.action()`

- [ ] **`transition:` / `in:` / `out:`** — transitions
  - Need: AST (`TransitionDirective`), parser, template codegen
  - Ref: `reference/compiler/phases/3-transform/client/visitors/TransitionDirective.js`
  - Runtime: `$.transition()`

- [ ] **`animate:`** — FLIP animations
  - Need: AST (`AnimateDirective`), parser, template codegen (EachBlock integration)
  - Ref: `reference/compiler/phases/3-transform/client/visitors/AnimateDirective.js`
  - Runtime: `$.animation()`

---

## Priority 3 — Special elements

Svelte special elements for dynamic rendering.

- [ ] **`<svelte:component this={X}>`** — dynamic component
  - Ref: `reference/compiler/phases/3-transform/client/visitors/SvelteComponent.js`
  - Runtime: `$.component()`

- [ ] **`<svelte:element this={tag}>`** — dynamic element
  - Ref: `reference/compiler/phases/3-transform/client/visitors/SvelteElement.js`
  - Runtime: `$.element()`

- [ ] **`<svelte:head>`** — document head management
  - Ref: `reference/compiler/phases/3-transform/client/visitors/SvelteHead.js`
  - Runtime: `$.head()`

- [ ] **`<svelte:window>`** / **`<svelte:body>`** / **`<svelte:document>`** — global event binding
  - Ref: `SvelteWindow.js`, `SvelteBody.js`, `SvelteDocument.js`

---

## Priority 4 — Less common features

Features used less frequently or in advanced scenarios.

- [ ] **`{@const x = expr}`** — block-scoped constant
  - Need: AST, parser, scope handling, codegen
  - Ref: `reference/compiler/phases/3-transform/client/visitors/ConstTag.js`
  - Runtime: `$.derived()`

- [ ] **`{#await promise}`** — async block
  - Need: AST (`AwaitBlock`), parser, analyze, template codegen
  - Ref: `reference/compiler/phases/3-transform/client/visitors/AwaitBlock.js`
  - Runtime: `$.await()`

- [ ] **`<slot>`** + `let:` — slot content with variables
  - Ref: `reference/compiler/phases/3-transform/client/visitors/SlotElement.js`, `LetDirective.js`
  - Runtime: `$.slot()`

- [ ] **`on:event`** (legacy) — legacy event handler syntax
  - Ref: `reference/compiler/phases/3-transform/client/visitors/OnDirective.js`
  - Runtime: `$.event()`

- [ ] **`<svelte:self>`** — recursive component
  - Ref: `reference/compiler/phases/3-transform/client/visitors/SvelteSelf.js`

- [ ] **`<svelte:fragment>`** — fragment wrapper
  - Ref: `reference/compiler/phases/3-transform/client/visitors/SvelteFragment.js`

- [ ] **`<svelte:boundary>`** — error boundary
  - Ref: `reference/compiler/phases/3-transform/client/visitors/SvelteBoundary.js`

- [ ] **`<svelte:options>`** — compiler options tag
  - Parser only, no codegen needed

- [ ] **`{@debug vars}`** — dev-mode debugger
  - Ref: `reference/compiler/phases/3-transform/client/visitors/DebugTag.js`

- [ ] **`{@attach fn}`** — element attachment (new in Svelte 5)
  - Ref: `reference/compiler/phases/3-transform/client/visitors/AttachTag.js`

- [ ] **`<title>`** — special handling in `<svelte:head>`
  - Ref: `reference/compiler/phases/3-transform/client/visitors/TitleElement.js`

---

## Priority 5 — Validation & DX

Compile-time checks and developer experience.

- [ ] Bind directive validation (incompatible elements)
  - Ref: `reference/compiler/phases/2-analyze/visitors/BindDirective.js`
- [ ] Assignment validation (const/import mutation)
  - Ref: `reference/compiler/phases/2-analyze/visitors/AssignmentExpression.js`
- [ ] Directive placement validation (e.g., transition on component)
  - Ref: `reference/compiler/phases/2-analyze/visitors/Component.js`
- [ ] Rune argument validation (e.g., `$state(a, b)`)
  - Ref: `reference/compiler/phases/2-analyze/visitors/CallExpression.js`
- [ ] A11y warnings
- [ ] CSS scoping / pruning / `:global`

---

## Priority 6 — Optimization

Performance improvements for generated code.

- [ ] Skip wrapping unmutated runes (no `$.state()` when never assigned)
- [ ] Event delegation analysis
- [ ] CSS hash injection

---

## Architectural Notes

- **OXC** — JS expression parsing/scoping, only `Span` stored in AST
- **Side tables** (`AnalysisData`) — no AST mutations
- **Analyze**: composite visitor (tuple `TemplateVisitor`) — single tree walk for all passes
- **Codegen**: direct recursion, no visitor pattern
- **Scope system NOT needed** for Priorities 1-4 (runes mode). Current approach (OXC + side tables) is sufficient.
- Each feature: test case → expected output via reference compiler → `cargo test`
