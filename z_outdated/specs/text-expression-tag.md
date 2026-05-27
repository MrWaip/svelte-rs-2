# Text / ExpressionTag

## Current state
- **Working**: 15/15 use cases
- **Tests**: 26/26 green
- Last updated: 2026-05-21

## Source

- `ROADMAP.md` Template item: `Text / ExpressionTag`
- Audit request: `Template -> Text / ExpressionTag`

## Syntax variants

- Plain text nodes in fragments and element children
- Standalone expression tags: `{expression}`
- Mixed text and expression sequences: `Hello {name}!`
- Title text sequences in `<svelte:head><title>...</title></svelte:head>`
- Async expression tags inside text content
- Text containing HTML character references such as `&amp;` and `&lt;`

## Use cases

- [x] Standalone static text nodes compile to static DOM text
- [x] Standalone expression tags compile at root and inside elements
- [x] Mixed text and expression sequences compile for root, regular elements, and `<title>`
- [x] SVG whitespace handling works for ignorable inter-element whitespace and `<text>` content
- [x] Text entities decode correctly for mixed text/expression concatenation in root fragments, regular elements, and `<title>`
- [x] Template validation rejects invalid text / expression placement with `node_invalid_placement`
- [x] Bidirectional control character warnings in text nodes are implemented, including `svelte-ignore` handling
- [x] Concat-context expression interpolations whose value is statically known to be non-nullish skip the `?? ""` fallback. `is_node_expr_definitely_defined` in `crates/svelte_codegen_client/src/codegen/mod.rs` returns `true` for `BinaryExpression`, so `${a() + b()}` inside a `set_text` template literal emits without the fallback (test: `text_expression_binary_no_nullish_fallback`).
- [x] HTML named entity decoder is safe when the entity is followed by a multi-byte UTF-8 character. `decode_named_entity` in `crates/svelte_parser/src/html.rs` limits its search to the leading ASCII byte run of the remainder (named entity names are ASCII-only), so `&rest[..end]` never lands inside a multi-byte char boundary (regression: `&nbsp;всегда`).
- [x] Pure-static element text fed into the `$.from_html(`...`)` template literal preserves named HTML entities verbatim. Codegen `ConcatPart::StaticEntities { html, text }` carries both forms when the parser-level `Text::decoded` is `Some` (entities present). Template-HTML consumers (`Template::push_text` via `FragmentAnchor::Child`) read the raw `html` form through `FragmentCtx::static_html_of`; JS-string consumers (`$.text(...)`, `${…}` template-literal interpolation, `document.title = …`) keep using the decoded `text` form through `FragmentCtx::static_text_of`. Trim is applied to both forms with the same flags — leading/trailing ASCII-whitespace runs are identical in raw and decoded so positions align. (test: `diagnose_html_template_preserves_nbsp`)
- [x] Sibling-text concatenation that interpolates an imported binding (e.g. `<div>Hello {NAME} world<Other/></div>`) compiles to `$.template_effect(() => $.set_text(text, …))`, not `text.nodeValue = …`. Owning layer: **analyze**. Imports cross a module boundary that the compiler cannot evaluate (a `.svelte.js` re-export may carry `$state`), so they are classified as `BindingSemantics::MaybeReactive` — a domain answer to "we cannot prove non-reactivity". `is_symbol_dynamic` in `crates/svelte_analyze/src/expression_semantics/builder/derive.rs` (and the mirror in `passes/dynamism.rs`) returns `true` for `MaybeReactive`, feeding `ExprKind::SimpleRead { reactive: true }` so concat-context emits `template_effect`. Component / snippet binding classifiers (`is_reactive_component_binding`, `is_reactive_symbol` in `block_semantics/builder/render.rs`) treat `MaybeReactive` as static — imported `<Other />` and `{@render snip()}` keep their direct call form. (test: `diagnose_text_concat_import_uses_template_effect`)

- [x] Concat-context interpolations lifted into a `template_effect` memo param (`$0`, sync or async) emit `${$0 ?? ""}` unconditionally. `build_concatenation_parts` in `crates/svelte_codegen_client/src/codegen/concatenation.rs` tracks `was_memoized` per part: when `data.kind` triggers `sync_values_push` / `async_values_push`, `defined` is forced to `false` regardless of the source expression's `ExpressionSemantics::Evaluation` because the substituted identifier `$0` carries no defined-ness. Same correction applied to `TitlePart::SyncMemo` / `TitlePart::AsyncMemo` in `crates/svelte_codegen_client/src/codegen/hoisted/title.rs` (mirror site, no failing case yet but kept consistent). Test: `text_expression_conditional_memoized_needs_nullish_fallback`.

- [x] Concat-context interpolation whose codegen form is a comma `SequenceExpression` (multi-source auto-tracking from `apply_legacy_wrap`: `(deep_read_state(a), deep_read_state(b), untrack(() => expr))`) emits as `${(seq) ?? ""}` inside the `set_text` template literal, matching reference. `build_concatenation_parts` in `crates/svelte_codegen_client/src/codegen/concatenation.rs` re-checks the post-wrap `effective_expr`: when it is `Expression::SequenceExpression`, the per-part `defined` flag is forced to `false` regardless of the source expression's `Evaluation`, so the builder wraps the sequence in `LogicalExpression(Coalesce, seq, "")`. The OXC printer adds the parentheses around the comma sequence on the lhs of `??` automatically by operator precedence. (test: `diagnose_text_concat_sequence_expr_nullish_fallback`)

- [x] Element whose dynamic text is reached through `$.sibling(...)` (i.e. preceded by static text and/or other nodes) emits the base anchor as `$.child(parent)` without the text hint; the `true` hint belongs only on the outer `$.sibling(..., n, true)`. Layer: **codegen** — `make_sibling_expr` in `crates/svelte_codegen_client/src/codegen/fragment/process_children.rs` (the `ChildAnchor::ElementChild` branch) gates `Arg::Bool(true)` on `$.child` by `is_text && skipped == 0`, so when the dynamic target is reached through `$.sibling`, the inner `$.child` stays unflagged. Test: `diagnose_static_text_before_dynamic_in_element`.

- [x] Element with a single dynamic expression child whose source is reactive must compile to `$.child(el, true)` + `$.template_effect(() => $.set_text(text, ...))`, never to `el.textContent = $.get(...)`. The HTML template carries a single space placeholder (`<button> </button>`) so `$.child(el, true)` can attach during hydration. The codegen single-expression-child fast path already toggles between `textContent =` and the reactive form via `is_dynamic_template`; the gap was upstream in analyze. Owning layer: **analyze** — `collect_derived_init_refs` in `crates/svelte_analyze/src/reactivity_semantics/builder_v2/mod.rs` only walks the `$derived.by` argument when it's an `ArrowFunctionExpression` with `.expression == true`. Block-body arrows, function expressions, and non-arrow arguments are opaque: no refs collected, so `Derived.reactive` stays at the initial `true`, and the read-site `is_dynamic_template` correctly forces `template_effect`. Mirrors reference's `scope.evaluate` boundary — it does not peer into block bodies. (test: `diagnose_button_single_dynamic_text`)

## Out of scope

- Dev-only equality rewrites via `$.strict_equals(...)` / `$.equals(...)`; if this parity work is revived, track it in a separate dev-codegen spec instead of the roadmap-closed text/expression feature

## Reference

- Reference docs:
- `original/docs/03-template-syntax/01-basic-markup.md`
- Reference compiler:
- `original/compiler/phases/1-parse/state/text.js`
- `original/compiler/phases/2-analyze/visitors/Text.js`
- `original/compiler/phases/2-analyze/visitors/ExpressionTag.js`
- `original/compiler/phases/3-transform/client/visitors/shared/fragment.js`
- `original/compiler/phases/3-transform/client/visitors/RegularElement.js`
- `original/compiler/phases/3-transform/client/visitors/BinaryExpression.js`
- Rust implementation:
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_parser/src/scanner/mod.rs`
- `crates/svelte_analyze/src/passes/lower.rs`
- `crates/svelte_analyze/src/passes/content_types.rs`
- `crates/svelte_codegen_client/src/lib.rs`
- `crates/svelte_codegen_client/src/template/expression.rs`
- `crates/svelte_codegen_client/src/template/element.rs`
- `crates/svelte_codegen_client/src/template/title_element.rs`
- `tasks/compiler_tests/test_v3.rs`

## Test cases

- [x] `single_text_node`
- [x] `single_interpolation`
- [x] `static_interpolation`
- [x] `inline_await_text_concat`
- [x] `title_variants`
- [x] `svg_inner_whitespace_trimming`
- [x] `svg_text_preserves_whitespace`
- [x] `ts_strip_expression_tag`
- [x] `text_entity_decoding`
- [x] `text_entity_decoding_root`
- [x] `title_entity_decoding`
- [x] `invalid_text_parent_uses_topology_ancestor_lookup`
- [x] `validate_text_invalid_placement` (analyzer)
- [x] `validate_expression_tag_invalid_placement` (analyzer)
- [x] `validate_text_bidirectional_control_warning` (analyzer)
- [x] `validate_text_bidirectional_control_warning_ignored` (analyzer)
- [x] `text_expression_binary_no_nullish_fallback`
- [x] `decodes_named_entity_followed_by_multibyte_char` (parser unit)
- [x] `leaves_ampersand_when_multibyte_char_follows_directly` (parser unit)
- [x] `decodes_named_entity_in_mixed_multibyte_text` (parser unit)
- [x] `diagnose_html_template_preserves_nbsp`
- [x] `diagnose_button_single_dynamic_text`
- [x] `derived_by_block_body_is_opaque_to_inert_deps_optimizer` (analyzer unit)
- [x] `derived_by_expression_body_tracks_inert_deps` (analyzer unit)
- [x] `diagnose_text_concat_import_uses_template_effect`
- [x] `text_expression_conditional_memoized_needs_nullish_fallback`
- [x] `diagnose_text_concat_sequence_expr_nullish_fallback`
- [x] `diagnose_static_text_before_dynamic_in_element`
