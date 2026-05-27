# `<svelte:head>` / `<title>`

## Current state
- **Working**: 16/16 use cases
- **Tests**: 21/21 green
- Last updated: 2026-05-17

## Source

- ROADMAP Special Elements: `<svelte:head>` / `<title>`
- Audit request: `$audit <svelte:head> / <title>`

## Syntax variants

- `<svelte:head></svelte:head>`
- `<svelte:head><meta ... /></svelte:head>`
- `<svelte:head><link ... /></svelte:head>`
- `<svelte:head><title>static text</title></svelte:head>`
- `<svelte:head><title>{expr}</title></svelte:head>`
- `<svelte:head><title>text {expr} text</title></svelte:head>`
- `<svelte:head><title>&amp; {expr} &lt;</title></svelte:head>`
- `<svelte:head ...>...</svelte:head>`
- `<svelte:head>` only at component top level
- one `<svelte:head>` per component
- `<title ...>...</title>` inside `<svelte:head>`
- `<title>{await expr}</title>` inside `<svelte:head>`

## Use cases

- [x] Parse a top-level `<svelte:head>` fragment and preserve its child fragment as a dedicated `SvelteHead` node.
- [x] Treat `<title>` inside `<svelte:head>` as a special title element for lowering/codegen while leaving other head children as normal elements.
- [x] Generate `$.head(hash(filename), ($$anchor) => { ... })` for `<svelte:head>` content.
- [x] Lower `<title>` content into `$.document.title = ...` with static, reactive, mixed-text, entity-decoded, and async expression variants.
- [x] Allow other regular head children like `<meta>` and `<link>` alongside `<title>`.
- [x] Reject duplicate `<svelte:head>` tags with `svelte_meta_duplicate`.
- [x] Reject `<svelte:head>` outside the component top level with `svelte_meta_invalid_placement`.
- [x] Reject attributes or directives on `<svelte:head>` with `svelte_head_illegal_attribute`.
- [x] Reject attributes or directives on `<title>` inside `<svelte:head>` with `title_illegal_attribute`.
- [x] Reject non-text / non-expression children inside `<title>` with `title_invalid_content`.
- [x] Emit body template-fragment declaration (`var fragment = $.comment();` and analogues) BEFORE the `$.head(...)` call when component body root resolves to a `$.comment()` fragment (multi-root body, body that starts with `{#if}`/`{#each}`/`{#await}`/`{#key}`/`{#snippet}` consumer, etc.).
- [x] Do not emit a stray `var fragment = $.comment();` after `var fragment = root();` when the component body mixes a `from_html` root template with a top-level `{@render ...}` snippet call and `<svelte:head>` is present — only the single `root()` fragment declaration should appear.
- [x] `<svelte:head>` containing a `<script>` element inside a control-flow branch (e.g. `{#if}`) — fragment prepare injects a synthetic trailing `Comment` child when the only fragment child is a `<script>` element, so classification picks `Multi` and emit produces `var fragment_N = root_M(); var script = $.first_child(fragment_N); ... $.append($$anchor, fragment_N);` with `$.from_html(..., 1)` under `$.with_script`.
- [x] Fragment / node id allocation order — when `<svelte:head>` body needs a `$.comment()` fragment AND the outer body also needs one, the `$.head(...)` callback must claim the unsuffixed `fragment` / `node` names first, with the outer body taking `fragment_1` / `node_1`. Currently we allocate outer first, swapping the suffixes. layer: codegen; repro/test: head_nested_if_with_body_if_id_order; candidate specs: svelte-head-title; suggested spec: svelte-head-title.
- [x] Effect emit order inside `<svelte:head>` — `$.deferred_template_effect(() => { $.document.title = ... })` for `<title>` must come AFTER sibling `$.template_effect(...)` calls for reactive head children (e.g. `<meta content={x}/>`). Currently we emit the deferred title effect in source order, which puts it before sibling element effects. layer: codegen; repro/test: head_title_then_meta_effect_order; candidate specs: svelte-head-title; suggested spec: svelte-head-title.
- [x] Inline `<script>` body containing a JS template literal — backticks and `${` inside the script source are embedded raw into the generated `$.from_html(\`...\`)` template literal, producing syntactically invalid JS. `template_str_expr` (`crates/svelte_ast_builder/src/builder/templates.rs:14`) calls `self.ast.atom(value)` without escaping ``\` `` or `${`. layer: codegen; repro/test: diagnose_head_inline_script_template_literal; candidate specs: svelte-head-title; suggested spec: svelte-head-title.

## Out of scope

- SSR `head` string emission and server-transform parity
- Browser runtime verification beyond generated client output parity
- `<title>` outside `<svelte:head>` semantics beyond existing regular-element parsing

## Reference

- Reference docs: `original/docs/05-special-elements/05-svelte-head.md`
- Reference parser: `original/compiler/phases/1-parse/state/element.js`
- Reference analyze: `original/compiler/phases/2-analyze/visitors/SvelteHead.js`
- Reference analyze: `original/compiler/phases/2-analyze/visitors/TitleElement.js`
- Reference client transform: `original/compiler/phases/3-transform/client/visitors/SvelteHead.js`
- Reference client transform: `original/compiler/phases/3-transform/client/visitors/TitleElement.js`
- Reference diagnostics: `original/compiler/errors.js`
- Rust parser conversion: `crates/svelte_parser/src/svelte_elements.rs`
- Rust parser entry: `crates/svelte_parser/src/lib.rs`
- Rust analyze lowering: `crates/svelte_analyze/src/passes/lower.rs`
- Rust analyze fragment traversal: `crates/svelte_analyze/src/walker/traverse.rs`
- Rust client codegen: `crates/svelte_codegen_client/src/template/svelte_head.rs`
- Rust client codegen: `crates/svelte_codegen_client/src/template/title_element.rs`
- Existing compiler cases: `tasks/compiler_tests/cases2/svelte_head_basic`, `tasks/compiler_tests/cases2/svelte_head_reactive`, `tasks/compiler_tests/cases2/svelte_head_with_content`, `tasks/compiler_tests/cases2/title_variants`, `tasks/compiler_tests/cases2/async_title_basic`, `tasks/compiler_tests/cases2/svelte_head_title_meta`, `tasks/compiler_tests/cases2/title_entity_decoding`, `tasks/compiler_tests/cases2/head_with_special_elements`, `tasks/compiler_tests/cases2/head_with_snippets`, `tasks/compiler_tests/cases2/head_position_with_body`, `tasks/compiler_tests/cases2/head_with_if_body`

## Test cases

- [x] `svelte_head_basic`
- [x] `svelte_head_reactive`
- [x] `svelte_head_with_content`
- [x] `title_variants`
- [x] `async_title_basic`
- [x] `svelte_head_title_meta`
- [x] `title_entity_decoding`
- [x] `head_with_special_elements`
- [x] `head_with_snippets`
- [x] `head_position_with_body`
- [x] Parser coverage for duplicate `<svelte:head>` and invalid `<svelte:head>` placement
- [x] Analyzer coverage for illegal `<svelte:head>` attributes, illegal `<title>` attributes, and invalid `<title>` content
- [x] `svelte_head_illegal_attribute`
- [x] `title_illegal_attribute`
- [x] `title_invalid_content`
- [x] `svelte_meta_duplicate_head`
- [x] `svelte_meta_invalid_placement_head`
- [x] `head_with_if_body`
- [x] `head_with_render_and_component`
- [x] `diagnose_head_script_in_if`
- [x] `head_nested_if_with_body_if_id_order`
- [x] `head_title_then_meta_effect_order`
- [x] `diagnose_head_inline_script_template_literal`
