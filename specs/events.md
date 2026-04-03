# Events

## Current state
- **Working**: 9/11 event use cases
- **Done this session**: fixed `nonpassive` codegen (`void 0` capture slot), added `name_span` to `OnDirectiveLegacy` AST, implemented analyzer event diagnostics (`EventHandlerInvalidModifier`, `EventHandlerInvalidModifierCombination`, `EventDirectiveDeprecated`, `MixedEventHandlerSyntaxes`)
- **Remaining**: component `$$events` forwarding tracked in `specs/component-node.md`
- Last updated: 2026-04-03

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
- [~] Event work that targets components is split across specs: DOM events are covered here, while `<Component on:done={...} />` -> `$$events` remains open in [component-node.md](/Users/klobkov/personal-code/svelte-rs-2/specs/component-node.md)

- [ ] Dev-mode `$.apply()` + event handler naming

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

## Tasks

- [x] Analyze: add DOM-event validation/warning coverage for legacy modifiers and mixed syntax, matching the reference compiler's DOM-element-only behavior
- [x] Analyze: track whether a component/template uses event attributes versus legacy `on:` so `mixed_event_handler_syntaxes` can be emitted at the correct node
- [x] Analyze: warn on legacy `on:` directives in runes mode for DOM elements, keep component `on:` warnings suppressed
- [x] Tests: add focused analyzer unit tests for `event_handler_invalid_modifier`, passive conflict errors, mixed syntax, and `event_directive_deprecated`
- [ ] Follow-up separately in [component-node.md](/Users/klobkov/personal-code/svelte-rs-2/specs/component-node.md): component `on:` forwarding into `$$events`

## Implementation order

1. Add the missing analyze-time event validation/warning pass or extend the existing template validation path.
2. Cover the new diagnostics with bounded tests before changing any client-output behavior.
3. Keep component `$$events` work separate and continue it via `specs/component-node.md`.

## Discovered bugs

- FIXED: `crates/svelte_analyze` now emits `EventDirectiveDeprecated`, `MixedEventHandlerSyntaxes`, legacy modifier validation errors, and passive-conflict errors for DOM event handling.
- FIXED: `crates/svelte_codegen_client/src/template/events/emit.rs` now emits `$.event("touchmove", el, handler, void 0, false)` for `on:touchmove|nonpassive`.
- OPEN: `<Component on:done={handler} />` still drops `$$events` in client codegen; this is tracked in `specs/component-node.md` and reproduced by `component_events`.

## Test cases

- Existing covered compiler cases:
  - `event_attr_capture`
  - `event_attr_capture_non_deleg`
  - `event_attr_gotpointercapture`
  - `event_attr_has_call`
  - `event_attr_import_handler`
  - `event_attr_member_handler`
  - `event_attr_non_delegatable`
  - `event_attr_passive`
  - `event_attr_passive_window`
  - `event_mixed_delegation`
  - `on_directive`
  - `on_directive_modifiers`
  - `svelte_body_event_attr`
  - `svelte_body_event_legacy`
  - `svelte_document_bubble`
  - `svelte_document_events`
  - `svelte_window_event_attr`
  - `svelte_window_event_legacy`
- Added during this audit:
  - `on_directive_nonpassive`
- Related failing case in another spec:
  - `component_events`
- Recommended next command:
  - `fix-test on_directive_nonpassive`
