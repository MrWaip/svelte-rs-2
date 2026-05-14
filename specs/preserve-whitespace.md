# preserveWhitespace

## Current state
- **Working**: 25/31 use cases
- **Tests**: 18/18 green
- Last updated: 2026-05-22

## Source

- User request: `/audit preserveWhitespace`
- Reference option: `compilerOptions.preserveWhitespace` (default `false`)
- Reference attribute: `<svelte:options preserveWhitespace={…} />`

## How It Works

`preserveWhitespace` is a compile-time option, settable via either `compilerOptions.preserveWhitespace` or `<svelte:options preserveWhitespace={…} />`. Resolution: `<svelte:options>` value wins; otherwise the compile option; default `false`.

When `false` (default), `clean_nodes` (reference) / `prepare.rs` (ours) trims template fragments:

1. Drop leading and trailing whitespace-only `Text` nodes from each fragment.
2. Strip leading whitespace from the first remaining `Text` node and trailing whitespace from the last.
3. For each remaining `Text` node, collapse leading whitespace to either `''` (if previous text already ended in whitespace) or `' '`, and trailing whitespace to `' '`. Skip the collapse on the side adjacent to an `ExpressionTag`, because text+expression+text fuses into one logical text run.
4. Drop a single-space `Text` entirely when the parent is a "whitespace-removable" container (`select`, `tr`, `table`, `tbody`, `thead`, `tfoot`, `colgroup`, `datalist`) or any SVG element other than `<text>` (and `foreignObject` resets back to HTML).

When `true`, none of the above runs — text is forwarded verbatim. Two implicit overrides force `true` regardless of the option:

- `<pre>` and `<textarea>` always preserve whitespace inside their fragment.
- Client codegen also forces `preserve_whitespace=true` for `<script>` element children.

There is one extra rule that runs even with preservation: if `<pre>`'s very first child `Text` is exactly `\n` or `\r\n`, drop it — the browser eats that leading newline anyway, so keeping it would diverge between SSR and hydration.

Trailing source whitespace is stripped at parser entry via `source.trim_end()` in `parse_with_js`, mirroring reference's `template.trimEnd()` in `phases/1-parse/index.js:97`. This means the AST never carries trailing whitespace-only `Text` nodes at the source-EOF boundary, so codegen does not need any root-vs-inner-fragment distinction when preservation is on.

## Syntax variants

```svelte
<svelte:options preserveWhitespace />
<svelte:options preserveWhitespace={true} />
<svelte:options preserveWhitespace={false} />
<svelte:options preserveWhitespace="true" />   // invalid: string value
```

```js
compile(source, { preserveWhitespace: true });
compile(source, { preserveWhitespace: false });
```

## Use cases

- [x] `CompileOptions::preserve_whitespace` field with default `false`; threaded into `AnalyzeOptions::preserve_whitespace`, stored on `AnalysisData::script.preserve_whitespace`, exposed via `CodegenView::preserve_whitespace()` and `FragmentCtx::preserve_whitespace`.
- [x] Parser accepts `<svelte:options preserveWhitespace />` (boolean shorthand), `={true}`, `={false}`; all flow through `process_svelte_option_bool` into `SvelteOptions::preserve_whitespace`.
- [x] Parser rejects string value `preserveWhitespace="true"` with `svelte_options_invalid_attribute_value` diagnostic, and unknown attribute names with `svelte_options_unknown_attribute`.
- [x] Resolution: `<svelte:options preserveWhitespace>` overrides `CompileOptions::preserve_whitespace`; otherwise falls back to compile option, default `false` (test: `svelte_options_preserve_whitespace`).
- [x] Default behavior — `preserve_whitespace=false`: drop leading and trailing whitespace-only Text nodes, strip leading/trailing whitespace from boundary Text nodes (test: many; representative: `svg_inner_whitespace_trimming`).
- [x] Default behavior — collapse internal Text whitespace adjacent to non-Text siblings to single `' '`, with `''` collapse when previous Text already ended in whitespace.
- [x] Default behavior — preserve whitespace at boundary with `ExpressionTag`: do not collapse leading ws before an expression or trailing ws after an expression, because Text+Expression+Text fuses into one logical run.
- [x] `can_remove_entirely` for parents `select`, `tr`, `table`, `tbody`, `thead`, `tfoot`, `colgroup`, `datalist` — drop single-space Text entirely.
- [x] `can_remove_entirely` for any SVG element except `<text>` (and any element that has a `<text>` ancestor); `foreignObject` switches the namespace back to HTML and disables removal (test: `svg_inner_whitespace_trimming`, `svg_text_preserves_whitespace`).
- [x] `<pre>` parent forces `preserve_whitespace=true` for its children fragment, regardless of option (`FragmentCtx::child_of_element` sets `inside_pre`).
- [x] `<textarea>` parent forces `preserve_whitespace=true` for its children fragment (test: `textarea_child_value_dynamic`).
- [x] Compile option `preserveWhitespace=true` (without `<svelte:options>`) preserves leading and trailing whitespace at the root fragment (test: `preserve_whitespace_compile_option_true`). Test runner `tasks/compiler_tests/test_v3.rs:case_input_and_options` extended during this audit to read `preserveWhitespace` from `config.json`.
- [x] `<script>` element child fragment in the template forces `preserve_whitespace=true` for its content, alongside `<pre>`/`<textarea>` (matches `phases/3-transform/client/visitors/RegularElement.js:317`). Implemented via the propagating `FragmentCtx::inside_script` flag set in `child_of_element("script", ...)`.
- [x] `<pre>` first-child Text equal to exactly `\n` or `\r\n` is dropped, regardless of `preserve_whitespace` value (matches `phases/3-transform/utils.js:257-262`). Checked in `prepare.rs` against `FragmentCtx::parent_element_name == Some("pre")` after the trim filter (test: `preserve_whitespace_pre_first_newline`).
- [x] `preserve_whitespace=true` keeps trailing whitespace-only Text nodes inside any non-root fragment. Achieved systemically via `source.trim_end()` at parse entry in `crates/svelte_parser/src/lib.rs:parse_with_js`, mirroring `reference/compiler/phases/1-parse/index.js:97`. Codegen does not need to distinguish root from inner fragments (test: `preserve_whitespace_inner_trailing_text`).
- [x] Default-mode (`preserve_whitespace=false`) leading-whitespace collapse on a Text node whose decoded content contains a non-breaking space (`&nbsp;` → U+00A0) sandwiched between a non-Text element sibling and a following block-tag sibling (`{#if}` / `{#each}` / `{#await}`): rule #3 collapses the leading whitespace to a single `' '`. Test: `diagnose_text_entity_leading_ws_before_if_block`.
- [x] Default-mode (`preserve_whitespace=false`, `preserve_comments=false`) merging of whitespace-only `Text` runs that become adjacent after consecutive `Comment` nodes are stripped between two component (or non-Text) siblings. After `clean_nodes` collapse, two static `Text(" ")` siblings remain; reference inlines both as static template HTML (`<!>  <!>`) and navigates past via `sibling(node, 2)`. Fix: `prepare.rs` `merge_static_parts` fuses adjacent `BufItem::Text` at push site, so `flush_buf` emits a single `Child::Text` (static template inline) instead of `Child::Concat` (which always falls into runtime `text.nodeValue = ...`). Test: `diagnose_consecutive_comments_between_components`.
- [x] `can_remove_entirely` fires for a root fragment whose children are all SVG elements but whose own `fragment_namespace` is `Html` (template root with sibling `<svg>`/`<g>`). `FragmentCtx::root` now infers SVG via `fragment_children_are_svg` (per-child `creation_namespace`), mirroring reference `infer_namespace` in `Fragment.js:30`. Test: `diagnose_svg_fragment_root_ws_between_siblings`.
- [x] `can_remove_entirely` fires for a block-body fragment (`{#if}`/`{#each}`/…) whose children are bare SVG tags while the block sits inside a non-SVG ancestor. `FragmentCtx::child_of_block` runs the same SVG inference and gates it on a new `inside_svg_text` flag so SVG `<text>` subtrees keep their literal whitespace. Test: `diagnose_svg_block_fragment_ws_between_siblings`.
- [x] Component default slot (`children` snippet) whose body fragment is SVG-only: same SVG inference path closes whitespace between bare SVG siblings inside the slot. Test: `diagnose_svg_component_slot_ws_between_siblings`.
- [x] `inside_svg_text` guard: `{#if}`/`{#each}` placed directly inside `<text>` keeps the whitespace between its anchors verbatim (reference emits `<!> <!>`). Test: `diagnose_svg_text_block_ws_preserved`.
- [x] `{#snippet}` body fragment with bare SVG children: closed via existing `FragmentCtx::root` SVG inference path that snippet emission already uses. Test: `diagnose_svg_snippet_body_ws_between_siblings`.
- [x] Legacy `<svelte:fragment slot="…">` body fragment with bare SVG children: `FragmentCtx::child_of_named_slot` now accepts the slot body `FragmentId` and runs `fragment_children_are_svg` to set `can_remove_entirely`. Test: `diagnose_svg_legacy_slot_ws_between_siblings`.
- [x] Analyze-side `fragment_namespace_for` aligned with reference `infer_namespace` for `Root`/`ComponentChildren`/`NamedSlot` roles: walks direct element children via `creation_namespace`/`namespace` and returns the consistent namespace (SVG/MathML), otherwise falls back. This propagates to `block_semantics/builder/html_tag.rs` so `{@html x}` among SVG-only siblings now emits with the SVG flag set on `$.html(...)`. Test: `diagnose_svg_root_html_tag_strategy`.
- [x] `infer_namespace_from_children` recurses through `{#if}`/`{#each}`/`{#await}`/`{#key}`/`<svelte:boundary>` block fragments (mirrors reference `check_nodes_for_namespace` walker allow-list: EachBlock/IfBlock/AwaitBlock/KeyBlock/Fragment/Element/SvelteElement/Text). Opaque containers (`ComponentNode`, `SnippetBlock`, `SlotElementLegacy`, `SvelteFragmentLegacy`, `SvelteComponentLegacy`, `SvelteSelf`) are deliberately not traversed, matching reference. Test: `diagnose_svg_root_block_only_html_tag`.
- [x] `<td>` (or `<th>`) parent fragment containing two-or-more sibling `{#if}` blocks each shaped `{expr} <br/>`: reference keeps the single-space `Text(" ")` between block anchors and emits `set_text(text, \`${expr} ?? ""\` + " ")`. Root cause was `FragmentCtx::child_of_element` inheriting `can_remove_entirely` from the parent context, so the `<tr>` `WHITESPACE_REMOVABLE_ELEMENTS` membership leaked into the `<td>` body. Fix: `child_of_element` now recomputes `can_remove_entirely` strictly from the new `el_name` (and the SVG/foreignObject branch) instead of inheriting `self.can_remove_entirely`. Layer: codegen — `crates/svelte_codegen_client/src/codegen/data_structures/fragment_ctx.rs`. Test: `diagnose_td_sibling_if_blocks_whitespace`.
- [ ] `infer_namespace_from_children` does not yet descend into the child fragment of `Element`/`SvelteElement`, while reference `check_nodes_for_namespace` walks the full subtree. Visible only when a SVG element sub-tree contains a `<foreignObject>` with HTML descendants AND another sibling at the same enclosing fragment, e.g. `<svg>…<foreignObject><div/></foreignObject>…</svg> <g/>` at root. Reference walks into the foreignObject `<div>`, flips the fragment namespace to `html`, and preserves the inter-sibling whitespace; we stop at the direct `<svg>`/`<g>` (both SVG) and remove the whitespace. Fix scope: recurse into `Element`/`SvelteElement` `fragment` in `infer_namespace_from_children` and stop at the first non-SVG/non-MathML descendant (reference's `RegularElement` visitor returns `'html'` + `stop()`). Layer: analyze — `crates/svelte_analyze/src/passes/template_side_tables.rs`. No test yet.
- [ ] `fragment_namespace_for` does not run `infer_namespace_from_children` for `FragmentRole::SnippetBody`. Reference `infer_namespace` routes `SnippetBlock` parents through `check_nodes_for_namespace`. Observable when a snippet body contains SVG siblings plus `{@html x}`: `block_semantics/builder/html_tag.rs:7` reads analyze-side `fragment_namespace`, picks `Html`, and the emitted `$.html(...)` call lacks the trailing `, void 0, true` SVG flag. Codegen-side `FragmentCtx::root` (used for snippet body) still trims direct-child whitespace via `fragment_children_are_svg`, so this is purely an `{@html}`-strategy parity gap. Layer: analyze. No test yet.
- [ ] `fragment_namespace_for` does not run `infer_namespace_from_children` for `SlotElementLegacy` fallback (legacy `<slot>` element body) and may miss other Component-shaped parents reference enumerates (`Component`, `SvelteComponent`, `SvelteSelf` — legacy `<svelte:component>` / `<svelte:self>` children). Reference's `infer_namespace` list: `Fragment | Root | Component | SvelteComponent | SvelteFragment | SnippetBlock | SlotElement`. Our analyze covers `Root`/`ComponentChildren`/`NamedSlot`; missing roles correspond to slot-fallback content and the two legacy `svelte:` wrappers. Layer: analyze. No test yet.
- [ ] Codegen-side `fragment_children_are_svg` in `FragmentCtx::root` and `FragmentCtx::child_of_block` does not recurse through `{#if}/{#each}/{#await}/{#key}/<svelte:boundary>` bodies the way analyze-side `infer_namespace_from_children` already does. Observable when a fragment's only direct children are block nodes (e.g. snippet body or root being `{#if cond}<g/>{/if} {#if cond}<g/>{/if}`): direct-element scan returns none, `can_remove_entirely` stays false, and the whitespace between blocks survives in the template, inflating `$.next(N)`. Fix scope: either replace `fragment_children_are_svg` with a shared visitor mirroring `visit_node_for_namespace`, or compute `can_remove_entirely` purely from analyze-side `fragment_namespace` (single source of truth). Layer: codegen — `crates/svelte_codegen_client/src/codegen/data_structures/fragment_ctx.rs`. No test yet.
- [ ] `role_needs_text_first_next` (`crates/svelte_codegen_client/src/codegen/fragment/mod.rs:58`) covers `Root | EachBody | EachFallback | SnippetBody | ComponentChildren | SvelteBoundaryBody`. Reference `is_text_first` additionally fires for `SvelteComponent` and `SvelteSelf` parents (legacy `<svelte:component>` / `<svelte:self>` children). Our equivalent fragment roles for those parents need explicit inclusion so a Text/ExpressionTag-first child still gets an anchor comment. Layer: codegen. No test yet.
- [ ] Reference `check_nodes_for_namespace` returns `'maybe_html'` when a non-whitespace `Text` is found among SVG/MathML siblings, after which `infer_namespace` falls back to the direct-`RegularElement` second pass (skipping blocks). Our `infer_namespace_from_children` never demotes to a "maybe" state — once it has an SVG `acc`, the presence of a non-whitespace `Text` does not force the second-pass fallback. In all examined practical cases the result coincides (direct elements decide); a hardened parity audit needs a fuzz probe to confirm no observable behavior diverges. Layer: analyze. No test yet.

## Out of scope

- SSR / server codegen for `preserveWhitespace` — `svelte_codegen_server` does not exist yet. Reference handling lives in `phases/3-transform/server/visitors/RegularElement.js` and `Fragment.js`.
- `regex_whitespaces_strict` collapse on attribute values — that runs from a `trim_whitespace` flag inside `build_attribute_value`, not from `preserveWhitespace`; orthogonal feature.
- Lone `<script>` fragment empty-comment append (`clean_nodes` line 267) — separate runtime concern, tracked under script-related work.
- `<script>` element template emission via `$.with_script(...)` runtime wrapper — tracked under `specs/unknown.md` (`script-tag-with-script-wrapper`); orthogonal to whitespace.

## Reference

### Svelte
- `reference/compiler/types/index.d.ts` (option type)
- `reference/compiler/validate-options.js:108` (compile option default)
- `reference/compiler/phases/1-parse/read/options.js:183-186` (`<svelte:options preserveWhitespace>` parser)
- `reference/compiler/phases/1-parse/index.js:97` (`template.trimEnd()` at parse entry — root trim trick)
- `reference/compiler/phases/3-transform/utils.js:111-305` `clean_nodes` (full whitespace pipeline + `<pre>` first-newline + `can_remove_entirely`)
- `reference/compiler/phases/3-transform/client/visitors/Fragment.js:33-41` (passes `preserve_whitespace` to `clean_nodes`)
- `reference/compiler/phases/3-transform/client/visitors/RegularElement.js:303-319` (forces `preserve_whitespace` for `pre`/`textarea`/`script`)
- `reference/compiler/phases/3-transform/server/visitors/Fragment.js:21` and `RegularElement.js:25-26,79` (server side; out of scope)
- `reference/compiler/phases/patterns.js:5-11` (`regex_starts_with_whitespaces`, `regex_ends_with_whitespaces`, `regex_not_whitespace`, `regex_whitespaces_strict`)

### Our code
- `crates/svelte_compiler/src/options.rs:23,48` (`CompileOptions::preserve_whitespace`)
- `crates/svelte_compiler/src/lib.rs:71-80,109` (`resolved_preserve_whitespace_option`)
- `crates/napi_compiler/src/lib.rs:38,106-107` (NAPI option pass-through)
- `crates/svelte_ast/src/lib.rs:1038` (`SvelteOptions::preserve_whitespace`)
- `crates/svelte_ast/src/lib.rs:146-152` (`WHITESPACE_REMOVABLE_ELEMENTS`, `is_whitespace_removable_parent`)
- `crates/svelte_parser/src/svelte_elements.rs:138,193,259` (parse `<svelte:options>` attribute)
- `crates/svelte_analyze/src/lib.rs:82,98,126` (`AnalyzeOptions::preserve_whitespace` plumbing)
- `crates/svelte_analyze/src/types/data/analysis.rs:17,41` (storage)
- `crates/svelte_analyze/src/types/data/codegen_view.rs:26-28` (accessor)
- `crates/svelte_parser/src/lib.rs:parse_with_js` (`source.trim_end()` at parse entry)
- `crates/svelte_codegen_client/src/codegen/data_structures/fragment_ctx.rs` (`inside_pre`, `inside_textarea`, `inside_script`, `parent_element_name`, `can_remove_entirely`)
- `crates/svelte_codegen_client/src/codegen/fragment/prepare.rs` (whitespace trim pipeline + `<pre>` first-newline rule)
- `tasks/compiler_tests/test_v3.rs:case_input_and_options` (reads `preserveWhitespace` from `config.json`)

## Test cases

- [x] `svelte_options_preserve_whitespace`
- [x] `textarea_child_value_dynamic`
- [x] `svg_inner_whitespace_trimming`
- [x] `svg_text_preserves_whitespace`
- [x] `preserve_whitespace_compile_option_true`
- [x] `preserve_whitespace_pre_first_newline`
- [x] `preserve_whitespace_inner_trailing_text`
- [x] `diagnose_text_entity_leading_ws_before_if_block`
- [x] `diagnose_consecutive_comments_between_components`
- [x] `diagnose_svg_fragment_root_ws_between_siblings`
- [x] `diagnose_svg_block_fragment_ws_between_siblings`
- [x] `diagnose_svg_component_slot_ws_between_siblings`
- [x] `diagnose_svg_text_block_ws_preserved`
- [x] `diagnose_svg_snippet_body_ws_between_siblings`
- [x] `diagnose_svg_legacy_slot_ws_between_siblings`
- [x] `diagnose_svg_root_html_tag_strategy`
- [x] `diagnose_svg_root_block_only_html_tag`
- [x] `diagnose_td_sibling_if_blocks_whitespace`
