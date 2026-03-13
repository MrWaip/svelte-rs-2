# Roadmap: Svelte 5 Compiler in Rust

Full feature catalog for porting. Current work items: see `TODO.md`.

---

## Tier 1 — Core (Done)

### AST & Parser
- [x] `Text`, `Element`, `ComponentNode`, `Comment`
- [x] `ExpressionTag` — `{expr}`
- [x] `IfBlock`, `EachBlock`, `SnippetBlock`, `RenderTag`
- [x] Attributes: string, expression, boolean, concatenation, shorthand/spread, `class:`, `bind:`

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
- [x] `$state` rune (read, assign, update)
- [x] `$props` rune (destructure, defaults, $bindable, rest, mutated)
- [x] Hoist imports
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

## Tier 2 — Needed for Real Apps

### 2a. {@html expr}
- [ ] AST: `Node::HtmlTag { expression: Span }`
- [ ] Parser: `{@html ...}`
- [ ] Codegen: `$.html()`
- Ref: `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js`

### 2b. {#key expr}
- [ ] AST: `Node::KeyBlock { expression: Span, fragment: Fragment }`
- [ ] Parser: `{#key ...}...{/key}`
- [ ] Codegen: `$.key()`
- Ref: `reference/compiler/phases/3-transform/client/visitors/KeyBlock.js`

### 2c. Event handlers (`onclick={handler}`)
- [ ] Codegen for event attributes (already parsed as expression attributes)
- Ref: `reference/compiler/phases/3-transform/client/visitors/shared/events.js`

### 2d. Style directive (`style:color={value}`)
- [ ] AST: `StyleDirective` (like ClassDirective)
- [ ] Parser: `style:prop={value}`
- [ ] Codegen: `$.style()`
- Ref: `reference/compiler/phases/3-transform/client/visitors/StyleDirective.js`

### 2e. Transitions / Animations
- [ ] AST: `TransitionDirective`, `AnimateDirective`
- [ ] Parser: `transition:fade`, `in:`, `out:`, `animate:`
- [ ] Codegen: `$.transition()`, `$.animate()`
- Ref: `reference/compiler/phases/3-transform/client/visitors/TransitionDirective.js`

### 2f. Component events & spread
- [ ] Event forwarding on components
- [ ] Spread props (`$.spread_props()`)
- Ref: `reference/compiler/phases/3-transform/client/visitors/Component.js`

### 2g. use:action directive
- [ ] AST: `UseDirective`
- [ ] Parser: `use:action={params}`
- [ ] Codegen: `$.action()`
- Ref: `reference/compiler/phases/3-transform/client/visitors/UseDirective.js`

---

## Tier 3 — Validation & Correctness

### 3a. Bind directive validation
- [ ] Error on bind:value on incompatible elements
- Ref: `reference/compiler/phases/2-analyze/visitors/BindDirective.js`

### 3b. Assignment validation
- [ ] Error on mutation of const/import
- Ref: `reference/compiler/phases/2-analyze/visitors/AssignmentExpression.js`

### 3c. Directive placement validation
- [ ] Error on transition: on component
- Ref: `reference/compiler/phases/2-analyze/visitors/Component.js`

### 3d. Rune argument validation
- [ ] Error on `$state(a, b)` etc.
- Ref: `reference/compiler/phases/2-analyze/visitors/CallExpression.js`

### 3e. {@const}
- [ ] AST: `Node::ConstTag`
- [ ] Parser: `{@const x = expr}`
- [ ] Scope-like handling in analyze
- Ref: `reference/compiler/phases/3-transform/client/visitors/ConstTag.js`

---

## Tier 4 — Edge Cases, Legacy, DX

- [ ] 4a. CSS analysis (pruning, :global, unused) — `svelte_analyze/src/css.rs`
- [ ] 4b. A11y warnings — `svelte_analyze/src/a11y.rs`
- [ ] 4c. Legacy mode (`$:`, stores, `export let`) — requires scope system
- [ ] 4d. Full scope system — ~1300 lines in reference
- [ ] 4e. AwaitBlock — AST, parser, codegen
- [ ] 4f. Function/closure warnings — DX only
- [ ] 4g. SSR codegen
- [ ] 4h. Skip wrapping runes that are never mutated

---

## Architectural Notes

- **OXC** — JS expression parsing/scoping, only `Span` stored in AST
- **Side tables** (`AnalysisData`) — no AST mutations
- **Analyze**: composite visitor (tuple `TemplateVisitor`) — single tree walk for all passes
- **Codegen**: direct recursion, no visitor pattern
- **Scope system NOT needed** for Tiers 1-3 (runes mode). Current approach (OXC + side tables) is sufficient.
- Critical path was: Props → Exports → Snippets → Components
- Each feature: test case → expected output via reference compiler → `cargo test`
