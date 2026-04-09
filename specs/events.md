# Events

## Current state
- **Working**: feature complete for client-side event handling in this spec's scope
- **Done this session**: normalized the cross-spec count drift, implemented dev-mode `$.apply(...)` wrapping for non-inline event handlers, and matched reference dev handler naming for Svelte 5 event attributes plus legacy `on:` directives
- **Scope note**: component `$$events` forwarding is tracked in `specs/component-node.md` and is already complete there; it is not remaining work for this spec
- **Remaining**: no open event-owned client-side use cases in this spec; compiler negative snapshots are not part of this feature
- Last updated: 2026-04-09

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
- [x] Event work that targets components is split across specs: DOM events are covered here, while `<Component on:done={...} />` -> `$$events` is implemented in [component-node.md](/Users/klobkov/personal-code/svelte-rs-2/specs/component-node.md)
- [x] Dev-mode `$.apply()` + event handler naming (tests: `event_attr_dev_apply`, `on_directive_dev_apply`)

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
- [x] `svelte_body_event_attr`
- [x] `svelte_body_event_legacy`
- [x] `svelte_document_bubble`
- [x] `svelte_document_events`
- [x] `svelte_window_event_attr`
- [x] `svelte_window_event_legacy`
- [x] `on_directive_nonpassive`
- [x] `component_events` (covered in `specs/component-node.md`)
