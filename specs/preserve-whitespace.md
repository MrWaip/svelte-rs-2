# preserveWhitespace

## Current state
- **Working**: 13/15 use cases
- **Tests**: 5/8 green
- Last updated: 2026-05-03

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

There is one extra rule that runs even with preservation: if `<pre>`'s very first child `Text` is exactly `\n` or `\r\n`, drop it — the browser eats that leading newline anyway, so keeping it would diverge between SSR and hydration. This applies after the trim step (or unconditionally inside `<pre>` when preservation is on).

Reference's parser also calls `template.trimEnd()` before parsing, so trailing source whitespace never reaches `clean_nodes`. Our parser does not. As a workaround, `prepare.rs` always trims trailing whitespace-only `Text` nodes from each fragment, even when `preserve_whitespace=true`. That is correct at the root fragment but wrong for inner fragments where preservation is on — see use cases.

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
- [x] `<pre>` parent forces `preserve_whitespace=true` for its children fragment, regardless of option (`FragmentCtx::child_of_element` sets `is_pre`).
- [x] `<textarea>` parent forces `preserve_whitespace=true` for its children fragment (test: `textarea_child_value_dynamic`).
- [x] Compile option `preserveWhitespace=true` (without `<svelte:options>`) preserves leading and trailing whitespace at the root fragment (test: `preserve_whitespace_compile_option_true`). Test runner `tasks/compiler_tests/test_v3.rs:case_input_and_options` extended during this audit to read `preserveWhitespace` from `config.json`.
- [ ] `<script>` element child fragment (template-level `<script>`, not module/instance) should also force `preserve_whitespace=true` for its children. Reference: `phases/3-transform/client/visitors/RegularElement.js:317` passes `name === 'script' || state.preserve_whitespace` to `clean_nodes`. Our `FragmentCtx::child_of_element` only special-cases `pre`/`textarea`; add `script` next to them. Quick fix. Note: the `preserve_whitespace_script_element` test fixture also exercises an unrelated `$.with_script(...)` wrapper that our codegen does not yet emit, so the test will need a second fix to fully match (test: `preserve_whitespace_script_element`, `#[ignore]`).
- [ ] `<pre>` first-child Text equal to exactly `\n` or `\r\n` must be dropped, regardless of `preserve_whitespace` value. Reference: `phases/3-transform/utils.js:257-262`. Triggered when `<pre>` opens with a newline followed by an inner element or `ExpressionTag`. Our `prepare.rs` has no such branch. Quick fix (test: `preserve_whitespace_pre_first_newline`, `#[ignore]`).
- [ ] `preserve_whitespace=true` must keep trailing whitespace-only Text nodes inside *inner* fragments, not just at the root. Our `prepare.rs:70-79` unconditionally trims trailing ws-only nodes regardless of `preserve_whitespace`, as a workaround for the missing top-level `template.trimEnd()` that reference applies before parsing. Fix: trim trailing ws-only nodes only at the root fragment (or apply `trim_end` once on `Component::source` like reference does), not inside element fragments. Moderate (test: `preserve_whitespace_inner_trailing_text`, `#[ignore]`).

## Out of scope

- SSR / server codegen for `preserveWhitespace` — `svelte_codegen_server` does not exist yet. Reference handling lives in `phases/3-transform/server/visitors/RegularElement.js` and `Fragment.js`.
- `regex_whitespaces_strict` collapse on attribute values — that runs from a `trim_whitespace` flag inside `build_attribute_value`, not from `preserveWhitespace`; orthogonal feature.
- Lone `<script>` fragment empty-comment append (`clean_nodes` line 267) — separate runtime concern, tracked under script-related work.

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
- `crates/svelte_codegen_client/src/codegen/data_structures/fragment_ctx.rs:10-12,27,50-61` (`is_pre`, `is_textarea`, `can_remove_entirely`)
- `crates/svelte_codegen_client/src/codegen/fragment/prepare.rs:68-103,127-147,418-442` (whitespace trim pipeline)
- `tasks/compiler_tests/test_v3.rs:38-72` (config.json reader missing `preserveWhitespace`)

## Test cases

- [x] `svelte_options_preserve_whitespace`
- [x] `textarea_child_value_dynamic`
- [x] `svg_inner_whitespace_trimming`
- [x] `svg_text_preserves_whitespace`
- [x] `preserve_whitespace_compile_option_true`
- [ ] `preserve_whitespace_pre_first_newline`
- [ ] `preserve_whitespace_inner_trailing_text`
- [ ] `preserve_whitespace_script_element`
