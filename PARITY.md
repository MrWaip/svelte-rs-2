# Svelte Compiler Feature Parity Report

Last updated: 2026-03-17

This document tracks feature parity between our Rust compiler and the Svelte reference compiler.
For implementation priorities, see [ROADMAP.md](ROADMAP.md).

## Summary

- **123** compiler integration test cases
- **13** of ~25 AST node types implemented
- **12** of 13 attribute/directive types implemented
- **11** analysis passes operational
- **15** codegen modules

## Legend

| Status | Meaning |
|--------|---------|
| Done | Fully implemented and tested |
| Partial | Core path works, edge cases or sub-features missing |
| Missing | Not implemented |
| N/A | Not applicable or intentionally deferred |

---

## AST Node Types

| Node Type | Reference | Ours | Status | Notes |
|-----------|-----------|------|--------|-------|
| Text | `Text` | `Node::Text` | Done | |
| RegularElement | `RegularElement` | `Node::Element` | Done | |
| Component | `Component` | `Node::ComponentNode` | Done | |
| Comment | `Comment` | `Node::Comment` | Partial | Parsed, no `preserveComments` codegen |
| ExpressionTag | `ExpressionTag` | `Node::ExpressionTag` | Done | |
| IfBlock | `IfBlock` | `Node::IfBlock` | Done | |
| EachBlock | `EachBlock` | `Node::EachBlock` | Done | |
| SnippetBlock | `SnippetBlock` | `Node::SnippetBlock` | Done | |
| RenderTag | `RenderTag` | `Node::RenderTag` | Partial | Missing optional chaining `fn?.()` |
| HtmlTag | `HtmlTag` | `Node::HtmlTag` | Done | |
| ConstTag | `ConstTag` | `Node::ConstTag` | Done | |
| KeyBlock | `KeyBlock` | `Node::KeyBlock` | Done | |
| SvelteHead | `SvelteHead` | `Node::SvelteHead` | Done | |
| AwaitBlock | `AwaitBlock` | — | Missing | `{#await}` not in AST or codegen |
| DebugTag | `DebugTag` | — | Missing | `{@debug}` not in AST or codegen |
| TitleElement | `TitleElement` | — | Missing | `<title>` special handling |
| SlotElement | `SlotElement` | — | Missing | Legacy `<slot>` element |
| SvelteWindow | `SvelteWindow` | — | Missing | `<svelte:window>` bindings/events |
| SvelteDocument | `SvelteDocument` | — | Missing | `<svelte:document>` bindings/events |
| SvelteBody | `SvelteBody` | — | Missing | `<svelte:body>` events |
| SvelteBoundary | `SvelteBoundary` | — | Missing | `<svelte:boundary>` error boundaries |
| SvelteComponent | `SvelteComponent` | — | Missing | Legacy `<svelte:component this={X}>` |
| SvelteElement | `SvelteElement` | — | Missing | `<svelte:element this={tag}>` |
| SvelteSelf | `SvelteSelf` | — | Missing | Legacy `<svelte:self>` |
| SvelteFragment | `SvelteFragment` | — | Missing | Legacy `<svelte:fragment>` |
| Error | — | `Node::Error` | N/A | Error recovery (ours only) |

---

## Attributes & Directives

| Type | Reference | Ours | Status | Notes |
|------|-----------|------|--------|-------|
| StringAttribute | `Attribute` | `StringAttribute` | Done | |
| ExpressionAttribute | `Attribute` | `ExpressionAttribute` | Done | |
| BooleanAttribute | `Attribute` | `BooleanAttribute` | Done | |
| ConcatenationAttribute | `Attribute` | `ConcatenationAttribute` | Done | |
| SpreadAttribute | `SpreadAttribute` | `ShorthandOrSpread` | Done | |
| ClassDirective | `ClassDirective` | `ClassDirective` | Done | |
| StyleDirective | `StyleDirective` | `StyleDirective` | Done | |
| BindDirective | `BindDirective` | `BindDirective` | Done | 15+ bind types |
| UseDirective | `UseDirective` | `UseDirective` | Done | |
| TransitionDirective | `TransitionDirective` | `TransitionDirective` | Done | in/out/both, local/global |
| AnimateDirective | `AnimateDirective` | `AnimateDirective` | Done | |
| AttachTag | `AttachTag` | `AttachTag` | Done | Svelte 5.29+ |
| OnDirective (legacy) | `OnDirective` | `OnDirectiveLegacy` | Done | Svelte 4, with modifiers |
| LetDirective | `LetDirective` | — | Missing | `let:` for slot context |

---

## Event Handling

| Feature | Status | Notes |
|---------|--------|-------|
| Delegatable event attributes (`onclick`) | Done | `$.delegated()` codegen |
| Event delegation setup (`$.delegate([...])`) | Done | Component-level |
| Non-delegatable event attributes (`onscroll`) | **Bug** | Uses `$.set_attribute()` instead of `$.event()` |
| Event capture suffix (`onclickcapture`) | Missing | |
| Passive event auto-detection | Missing | `touchstart`, `wheel`, etc. |
| Handler wrapping (non-inline) | Missing | `(...$$args) => handler.apply(this, $$args)` |
| Handler memoization (`has_call`) | Missing | Depends on Memoizer infrastructure |
| Legacy `on:` directive | Done | With all modifiers |

---

## Runes & Reactivity

| Feature | Status | Notes |
|---------|--------|-------|
| `$state()` | Done | Mutation tracking, $.get/$.set |
| `$state.raw()` | Done | |
| `$derived` | Done | |
| `$derived.by` | Done | |
| `$props()` | Done | Destructuring, defaults, bindability |
| `$effect` | Done | |
| `$effect.pre` | Done | |
| `$effect.tracking` | Done | |
| `$effect.root` | Done | |
| `$snapshot` | Done | |
| Store subscriptions (`$store`) | Done | |
| Legacy `$:` reactive declarations | Missing | Svelte 4 |

---

## Analysis Passes

| Pass | Status | Notes |
|------|--------|-------|
| JS expression parsing | Done | OXC-based |
| Scope building | Done | Unified scope tree |
| Reference resolution | Done | Template → symbol linking |
| Store subscription detection | Done | |
| Known values (const eval) | Done | |
| Props analysis | Done | |
| Fragment lowering (whitespace) | Done | |
| Reactivity marking | Done | Dynamic node detection |
| Elseif detection | Done | |
| Element flags | Done | Spread, class, style, etc. |
| Hoistable snippets | Done | |
| Content type classification | Done | |
| Needs-var computation | Done | |
| Expression memoization (`has_call`) | Missing | No `Memoizer` infrastructure |
| A11y validation | Missing | 0 of 39 rules |
| Full semantic validation | Missing | Placeholder only |

---

## Codegen — Client

| Feature | Status | Notes |
|---------|--------|-------|
| Root fragment generation | Done | |
| Element creation & hydration | Done | |
| Component instantiation | Done | Props, children |
| If/else/elseif blocks | Done | |
| Each blocks (item, index, key) | Done | |
| Snippet blocks (instance + hoistable) | Done | |
| Render tags | Partial | Missing optional chaining |
| HTML tags (`{@html}`) | Done | |
| Const tags | Done | Single + destructured |
| Key blocks | Done | |
| Svelte:head | Done | |
| Expression/text interpolation | Done | |
| Template HTML generation | Done | |
| Bind directives (15+ types) | Done | |
| Class directives | Done | |
| Style directives | Done | |
| Spread attributes | Done | |
| Use (action) directives | Done | |
| Transition directives | Done | |
| Animate directives | Done | |
| Attach tags | Done | |
| Legacy on: directives | Done | |
| Script rune transforms | Done | $.get/$.set/$.update |
| Module compilation (.svelte.js) | Done | |

---

## Codegen — Missing Visitors

These reference compiler visitors have no counterpart in our codegen:

| Visitor | Feature | Priority |
|---------|---------|----------|
| `AwaitBlock` | `{#await}` rendering | High |
| `DebugTag` | `{@debug}` dev output | Low |
| `TitleElement` | `<title>` text updates | Medium |
| `SlotElement` | Legacy `<slot>` | Low (legacy) |
| `SvelteWindow` | Window bindings/events | Medium |
| `SvelteDocument` | Document bindings/events | Medium |
| `SvelteBody` | Body events | Medium |
| `SvelteBoundary` | Error boundaries | Medium |
| `SvelteComponent` | Dynamic `this={X}` | Low (legacy) |
| `SvelteElement` | Dynamic `this={tag}` | Medium |
| `SvelteSelf` | Recursive components | Low (legacy) |
| `SvelteFragment` | Named fragments | Low (legacy) |
| `LetDirective` | Slot context vars | Low (legacy) |
| `Comment` | `preserveComments` | Low |

---

## Compiler Options

| Option | Reference | Ours | Status |
|--------|-----------|------|--------|
| `generate: 'client'` | Yes | Implicit | Done (client only) |
| `generate: 'server'` | Yes | — | Missing |
| `dev` mode | Yes | — | Missing |
| `css: 'injected' \| 'external'` | Yes | — | Missing |
| `cssHash` | Yes | — | Missing |
| `runes` | Yes | Always true | Partial |
| `namespace` | Yes | — | Missing |
| `preserveComments` | Yes | — | Missing |
| `preserveWhitespace` | Yes | Partial | Partial (lowering handles some) |
| `customElement` | Yes | Parsed | Missing (no codegen) |
| `hmr` | Yes | — | Missing |
| `filename` | Yes | — | Missing |
| `sourcemap` | Yes | — | Missing |
| `fragments: 'html' \| 'tree'` | Yes | html only | Partial |
| `discloseVersion` | Yes | — | Missing |
| `warningFilter` | Yes | — | Missing |

---

## CSS Processing

| Feature | Status | Notes |
|---------|--------|-------|
| CSS parsing | Partial | Stored as raw span, not AST |
| Scoped CSS (hash class) | Missing | |
| `:global()` selector | Missing | |
| CSS pruning (unused selectors) | Missing | |
| Keyframe name scoping | Missing | |
| CSS source maps | Missing | |

---

## Infrastructure

| Feature | Status | Notes |
|---------|--------|-------|
| WASM compilation target | Done | `wasm_compiler` crate |
| Diagnostics (errors) | Done | Parse errors |
| Diagnostics (warnings) | Missing | 0 of 82 warning codes |
| Source maps (JS) | Missing | |
| Source maps (CSS) | Missing | |
| SSR codegen | Missing | |
| HMR support | Missing | |
| Custom element codegen | Missing | |
| Preprocessor support | Missing | |

---

## Test Coverage by Feature Area

| Area | Cases | Notes |
|------|-------|-------|
| Core syntax | 7 | empty, text, element, interpolation, if, if-else |
| Components | 7 | basic, children, props, mixed |
| Bind directives | 11+ | All major bind types |
| Class & style | 9 | Directives, variables, objects |
| State & runes | 13 | $state, $derived, $effect variants |
| Props | 6 | Basic, bindable, lazy, rest, mutated |
| Each blocks | 7 | With conditions, nested |
| Snippets | 1 | Basic only |
| Transitions & animations | 14 | All direction/modifier combos |
| Directives (use/on) | 9 | Actions, legacy events |
| Special tags | 11 | const, html, attach, render, key |
| Head & metadata | 7 | svelte:head, store, options |
| **Total** | **~123** | |
