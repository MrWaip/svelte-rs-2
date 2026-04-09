# `<svelte:window>` / `<svelte:document>` / `<svelte:body>`

## Current state
- **Working**: 9/13 use cases
- **Missing**: 4 use cases
- **Next**: implement parser root-only duplicate/placement validation for all three tags, analyzer validation for children and illegal attributes, then port `{@attach}` codegen for `<svelte:document>`
- Last updated: 2026-04-07

## Source

- ROADMAP Special Elements: `<svelte:window>` / `<svelte:document>` / `<svelte:body>`
- Audit request: `$audit <svelte:window> / <svelte:document> / <svelte:body>`

## Syntax variants

- `<svelte:window on:event={handler} />`
- `<svelte:window onevent={handler} />`
- `<svelte:window bind:innerWidth={w} />`
- `<svelte:window bind:innerHeight={h} />`
- `<svelte:window bind:outerWidth={w} />`
- `<svelte:window bind:outerHeight={h} />`
- `<svelte:window bind:scrollX={x} />`
- `<svelte:window bind:scrollY={y} />`
- `<svelte:window bind:online={online} />`
- `<svelte:window bind:devicePixelRatio={dpr} />`
- `<svelte:document on:event={handler} />`
- `<svelte:document onevent={handler} />`
- `<svelte:document bind:activeElement={node} />`
- `<svelte:document bind:fullscreenElement={node} />`
- `<svelte:document bind:pointerLockElement={node} />`
- `<svelte:document bind:visibilityState={state} />`
- `<svelte:document {@attach attachment} />`
- `<svelte:body on:event={handler} />`
- `<svelte:body onevent={handler} />`
- `<svelte:body use:action />`
- `<svelte:window>...</svelte:window>`
- `<svelte:document>...</svelte:document>`
- `<svelte:body>...</svelte:body>`
- `<svelte:window>` only at component top level
- `<svelte:document>` only at component top level
- `<svelte:body>` only at component top level
- one `<svelte:window>` per component
- one `<svelte:document>` per component
- one `<svelte:body>` per component

## Use cases

- [x] Parse top-level `<svelte:window>`, `<svelte:document>`, and `<svelte:body>` into dedicated AST nodes instead of leaving them as regular elements.
- [ ] Reject duplicate `<svelte:window>`, `<svelte:document>`, and `<svelte:body>` tags with `svelte_meta_duplicate`.
- [ ] Reject `<svelte:window>`, `<svelte:document>`, and `<svelte:body>` outside the component top level with `svelte_meta_invalid_placement`.
- [ ] Reject children inside `<svelte:window>`, `<svelte:document>`, and `<svelte:body>` with `svelte_meta_invalid_content`.
- [ ] Reject non-event attributes and spread attributes on `<svelte:window>` / `<svelte:document>` with `illegal_element_attribute`, and on `<svelte:body>` with `svelte_body_illegal_attribute`.
- [x] Generate `<svelte:window>` event listeners for both Svelte 5 event attributes and legacy `on:` directives, including shared event modifier handling.
- [x] Generate `<svelte:window>` bindings for `innerWidth`, `innerHeight`, `outerWidth`, `outerHeight`, `scrollX`, `scrollY`, `online`, and `devicePixelRatio`.
- [x] Generate `<svelte:document>` event listeners for both Svelte 5 event attributes and legacy `on:` directives, including modifier/bubbling behavior shared with other event targets.
- [x] Generate `<svelte:document>` bindings for `activeElement`, `fullscreenElement`, `pointerLockElement`, and `visibilityState`.
- [ ] Generate `{@attach}` on `<svelte:document>`.
- [x] Generate `<svelte:body>` event listeners for both Svelte 5 event attributes and legacy `on:` directives.
- [x] Generate `use:` actions on `<svelte:body>`.
- [x] Preserve mixed special-element output when `<svelte:head>`, `<svelte:window>`, `<svelte:document>`, and `<svelte:body>` coexist in one component.

## Out of scope

- SSR behavior for `window`, `document`, or `document.body`
- Runtime semantics beyond client output parity and compiler diagnostics
- Event system features already tracked in `specs/events.md`
- Binding-system rules already tracked in `specs/bind-directives.md`
- Action semantics already tracked in `specs/use-action.md`

## Reference

- `reference/docs/05-special-elements/02-svelte-window.md`
- `reference/docs/05-special-elements/03-svelte-document.md`
- `reference/docs/05-special-elements/04-svelte-body.md`
- `reference/compiler/phases/1-parse/state/element.js`
- `reference/compiler/phases/2-analyze/visitors/SvelteWindow.js`
- `reference/compiler/phases/2-analyze/visitors/SvelteDocument.js`
- `reference/compiler/phases/2-analyze/visitors/SvelteBody.js`
- `reference/compiler/phases/2-analyze/visitors/shared/special-element.js`
- `reference/compiler/phases/2-analyze/visitors/BindDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/SvelteWindow.js`
- `reference/compiler/phases/3-transform/client/visitors/SvelteDocument.js`
- `reference/compiler/phases/3-transform/client/visitors/SvelteBody.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/special_element.js`
- `reference/compiler/phases/3-transform/client/visitors/BindDirective.js`
- `reference/compiler/errors.js`
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_parser/src/svelte_elements.rs`
- `crates/svelte_parser/src/walk_js.rs`
- `crates/svelte_analyze/src/passes/binding_properties.rs`
- `crates/svelte_analyze/src/passes/template_validation.rs`
- `crates/svelte_codegen_client/src/template/svelte_window.rs`
- `crates/svelte_codegen_client/src/template/svelte_document.rs`
- `crates/svelte_codegen_client/src/template/svelte_body.rs`
- `tasks/compiler_tests/cases2/svelte_window_event_legacy/`
- `tasks/compiler_tests/cases2/svelte_window_event_attr/`
- `tasks/compiler_tests/cases2/svelte_window_bind_scroll/`
- `tasks/compiler_tests/cases2/svelte_window_bind_size/`
- `tasks/compiler_tests/cases2/svelte_window_bind_online/`
- `tasks/compiler_tests/cases2/svelte_window_reactive/`
- `tasks/compiler_tests/cases2/svelte_window_combined/`
- `tasks/compiler_tests/cases2/svelte_document_events/`
- `tasks/compiler_tests/cases2/svelte_document_bindings/`
- `tasks/compiler_tests/cases2/svelte_document_bubble/`
- `tasks/compiler_tests/cases2/svelte_document_combined/`
- `tasks/compiler_tests/cases2/attach_on_document/`
- `tasks/compiler_tests/cases2/svelte_body_event_attr/`
- `tasks/compiler_tests/cases2/svelte_body_event_legacy/`
- `tasks/compiler_tests/cases2/svelte_body_action/`
- `tasks/compiler_tests/cases2/svelte_body_combined/`
- `tasks/compiler_tests/cases2/special_elements_all/`
- `tasks/compiler_tests/cases2/root_with_special_elements/`
- `tasks/compiler_tests/cases2/head_with_special_elements/`
- `tasks/compiler_tests/cases2/head_position_with_body/`
- `tasks/compiler_tests/cases2/event_attr_passive_window/`
- `specs/attach-tag.md`

## Test cases

- [x] `svelte_window_event_legacy`
- [x] `svelte_window_event_attr`
- [x] `svelte_window_bind_scroll`
- [x] `svelte_window_bind_size`
- [x] `svelte_window_bind_online`
- [x] `svelte_window_reactive`
- [x] `svelte_window_combined`
- [x] `svelte_document_events`
- [x] `svelte_document_bindings`
- [x] `svelte_document_bubble`
- [x] `svelte_document_combined`
- [x] `svelte_body_event_attr`
- [x] `svelte_body_event_legacy`
- [x] `svelte_body_action`
- [x] `svelte_body_combined`
- [x] `special_elements_all`
- [x] `root_with_special_elements`
- [x] `head_with_special_elements`
- [x] `head_position_with_body`
- [x] `event_attr_passive_window`
- [ ] `attach_on_document`
- [x] Parser coverage for duplicate and invalid-placement diagnostics for the root-only special-element family
- [x] Analyzer coverage for illegal-attribute and invalid-content diagnostics for the special-element family
