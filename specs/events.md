# Events

## Current state
- **Working**: 20/20 use cases
- **Tests**: 35/35 green
- Last updated: 2026-05-14

## Source

- User request: `/audit Events`
- Roadmap section: `## Events`

## Syntax variants

- `<button onclick={handler} />`
- `<button onclickcapture={handler} />`
- `<div ontouchstart={handler} />`
- `<button on:click={handler} />`
- `<button on:click|preventDefault|capture={handler} />`
- `<div on:touchmove|nonpassive={handler} />`
- `<svelte:window onresize={handler} />`
- `<svelte:document on:keydown />`
- `<svelte:body onclick={handler} />`

## Use cases

- [x] Svelte 5 event attributes on regular elements lower through `$.event(...)` or `$.delegated(...)` with exact event names (tests: `event_attr_import_handler`, `event_attr_member_handler`, `event_attr_has_call`)
- [x] Delegatable DOM events use per-node `$.delegated(...)` plus a module-level `$.delegate([...])` registration (test: `event_mixed_delegation`)
- [x] Capture suffix handling matches the reference compiler, including the `gotpointercapture`/`lostpointercapture` exception (tests: `event_attr_capture`, `event_attr_capture_non_deleg`, `event_attr_gotpointercapture`)
- [x] Passive auto-detection for touch events matches the reference compiler on DOM nodes and special elements (tests: `event_attr_passive`, `event_attr_passive_window`)
- [x] Legacy `on:` directives on DOM elements lower through `$.event(...)` and bubble forms work on special elements (tests: `on_directive`, `svelte_document_bubble`, `svelte_body_event_legacy`, `svelte_window_event_legacy`)
- [x] Legacy modifier wrappers that are already covered match reference output (`preventDefault`, `capture`, `once`) (tests: `on_directive_modifiers`, `svelte_document_events`)
- [x] `<svelte:window>`, `<svelte:document>`, and `<svelte:body>` accept both Svelte 5 event attributes and legacy `on:` syntax in the same special-element code paths (tests: `svelte_window_event_attr`, `svelte_document_events`, `svelte_body_event_attr`)
- [x] Legacy `nonpassive` modifier preserves an undefined capture slot and passes explicit passive `false` (`on_directive_nonpassive`)
- [x] Analyze emits DOM-event diagnostics and warnings: invalid modifiers, invalid passive/nonpassive combinations, mixed legacy/new syntax, and runes-mode `on:` deprecation warnings
- [x] Non-runes mode must not emit `event_directive_deprecated` for legacy `on:` directives on `<svelte:window>` (diagnostic test: `on_directive_not_deprecated_in_non_runes_mode_svelte_window`)
- [x] Legacy `on:` directive errors (`event_handler_invalid_modifier`, `event_handler_invalid_modifier_combination`, `mixed_event_handler_syntaxes`) report the full directive span, and diagnostic-parity harness collapses output to the first error to mirror reference's throw-on-first-error semantics (tests: `on_directive_invalid_modifier`, `on_directive_passive_nonpassive_conflict`, `on_directive_mixed_syntax`, `on_directive_mixed_syntax_svelte_element`)
- [x] Event work that targets components is split across specs: DOM events are covered here, while `<Component on:done={...} />` -> `$$events` is implemented in [component-node.md](/Users/klobkov/personal-code/svelte-rs-2/specs/component-node.md)
- [x] Dev-mode `$.apply()` + event handler naming (tests: `event_attr_dev_apply`, `on_directive_dev_apply`)
- [x] Delegated handler identifiers that resolve to non-direct post-transform bindings (e.g. `{@const}` destructure lifted into `$.derived` → `$.get(computed_const).<name>`) must be wrapped in `function(...$$args) { handler?.apply(this, $$args); }`, not passed raw to `$.delegated` (test: `event_attr_const_tag_destructure`)
- [x] Event-attribute handler whose identifier resolves to a `$props()` binding (`BindingSemantics::Prop` / `LegacyBindableProp`) is classified `HandlerEmit::WrappedInert` in `derive_handler_emit`, producing `function(...$$args) { $$props.<name>?.apply(this, $$args); }` instead of raw `$.delegated("click", el, $$props.onChange)` (test: `event_attr_props_handler`)
- [x] Element-level emit ordering: Svelte 5 event-attributes (`onclick={...}`) emit first in source order, then a separate bucket holds `on:`-directives, `transition:`, `use:`, `animate:`, `@attach`, `bind:` in their source order — matches reference's `state.after_update` vs `element_state.after_update` split (tests: `event_attr_delegated_after_non_delegated_order`, `transition_after_delegated`, `transition_before_lifecycle_events`)
- [x] Load-error elements (`<img>`, `<iframe>`, `<link>`, `<script>`, `<source>`, `<style>`, `<track>`, `<body>`) emit `$.replay_events(<node>)` into `after_update` when they carry spread, `use:`, `onload`, or `onerror` (test: `img_spread_replay_events`)
- [x] Legacy `on:` directive on an element that also carries `use:` action is wrapped as `$.effect(() => $.event(...))` and placed in `init` (reference `RegularElement.js` `has_use` branch). Without `use:` the unwrapped `$.event(...)` stays in `after_update` (test: `on_directive_with_use`)
- [x] When a legacy `on:` directive is wrapped as `$.effect(() => $.event(...))` due to a sibling `use:` action, the wrapped effect must remain in the source-order element bucket alongside `bind:this` / `use:` / other directives (test: `on_directive_with_use_bind_this_order`)
- [x] Nested elements that each carry a delegated event-attribute and a `transition:` directive emit all `$.delegated(...)` calls in preorder before any `$.transition(...)` call, which then emit in postorder. Codegen routes per-element event pushes into `EmitState.after_update` directly and per-element scoped directives (transition/use/animate/bind/attach/on-legacy) into `EmitState.element_after_update`; at the end of each element frame the scoped slice is split off and appended to `after_update`, mirroring reference's `context.state.after_update` vs `element_state.after_update` split (test: `diagnose_nested_delegated_transition_order`)

## Reference

- Reference analyze:
  - `reference/compiler/phases/2-analyze/visitors/Attribute.js`
  - `reference/compiler/phases/2-analyze/visitors/OnDirective.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/element.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/component.js`
  - `reference/compiler/phases/2-analyze/index.js`
- Reference client transform:
  - `reference/compiler/phases/3-transform/client/visitors/OnDirective.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/events.js`
  - `reference/compiler/phases/3-transform/client/transform-client.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
- Rust implementation:
  - `crates/svelte_parser/src/scanner/mod.rs`
  - `crates/svelte_ast/src/lib.rs`
  - `crates/svelte_analyze/src/passes/element_flags.rs`
  - `crates/svelte_analyze/src/utils/events.rs`
  - `crates/svelte_codegen_client/src/template/attributes.rs`
  - `crates/svelte_codegen_client/src/template/events/emit.rs`
  - `crates/svelte_codegen_client/src/template/events/handlers.rs`
  - `crates/svelte_codegen_client/src/template/svelte_window.rs`
  - `crates/svelte_codegen_client/src/template/svelte_document.rs`
  - `crates/svelte_codegen_client/src/template/svelte_body.rs`
  - `crates/svelte_diagnostics/src/lib.rs`

## Test cases

- [x] `event_attr_capture`
- [x] `event_attr_capture_non_deleg`
- [x] `event_attr_gotpointercapture`
- [x] `event_attr_has_call`
- [x] `event_attr_dev_apply`
- [x] `event_attr_import_handler`
- [x] `event_attr_member_handler`
- [x] `event_attr_non_delegatable`
- [x] `event_attr_passive`
- [x] `event_attr_passive_window`
- [x] `event_mixed_delegation`
- [x] `on_directive`
- [x] `on_directive_dev_apply`
- [x] `on_directive_modifiers`
- [x] `on_directive_invalid_modifier`
- [x] `on_directive_passive_nonpassive_conflict`
- [x] `on_directive_mixed_syntax`
- [x] `on_directive_mixed_syntax_svelte_element`
- [x] `on_directive_deprecated_in_runes_mode`
- [x] `svelte_body_event_attr`
- [x] `svelte_body_event_legacy`
- [x] `svelte_document_bubble`
- [x] `svelte_document_events`
- [x] `svelte_window_event_attr`
- [x] `svelte_window_event_legacy`
- [x] `on_directive_nonpassive`
- [x] `component_events` (covered in `specs/component-node.md`)
- [x] `on_directive_not_deprecated_in_non_runes_mode_svelte_window`
- [x] `event_attr_const_tag_destructure`
- [x] `event_attr_props_handler`
- [x] `event_attr_delegated_after_non_delegated_order`
- [x] `img_spread_replay_events`
- [x] `on_directive_with_use`
- [x] `on_directive_with_use_bind_this_order`
- [x] `diagnose_nested_delegated_transition_order`
