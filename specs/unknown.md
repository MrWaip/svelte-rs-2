# Unknown / unowned diagnose findings

## Current state
- Working: 4 unchecked unknown items
- Tests: 16/19 passing
- Last updated: 2026-05-22

## Probes with no reproduction

- 2026-05-12 / re-confirmed 2026-05-13 (twice) — runes component combining `$props`/`$state`/`$derived`/`$effect`, `Tween.of` from `svelte/motion`, `{@attach}`, `bind:clientHeight`, multi-arm `{#if}`/`{:else if}` with union-type discriminant (`typeof`/`'key' in obj`), `use:`-actions with reactive object args, `transition:`/`in:`/`out:` (including `slideAndFade` with arg object), snippet children + `{@render snippet?.()}` inside conditional branches, `style:` directive bound to derived value, scss with custom-media + var fallbacks, ts script. Mode default (runes auto), `generate=client`, no `--dev`. Result: full JS+CSS parity (207 lines js). No follow-up.

- 2026-05-17 — legacy-mode component (`<script lang="ts">` with `export let`, `$:` reactive `bgImage = \`url(\${benefit.background})\``, `<style lang="scss">` containing malformed `color: var(textPrimary);` — bare-ident inside `var(...)` instead of `--ident`). Sweep reported `format-error: Unexpected token Ident("textPrimary")` from terser parsing one of the compiled JS outputs (sweep does not distinguish which side); options were `{ generate: 'client', dev: false, modernAst: true, discloseVersion: false }`. Narrowed to `<script lang="ts">` + `$:` + element with `class:`/`style="--bgImage: {bgImage};"`/`on:keydown`/`on:click` + two scss rules with `var(textPrimary)`; ran via `just quick-check` — full JS+CSS parity (38 lines js, 103 bytes css), no panic, no format-error. Did not reproduce with: dropping `lang="ts"`, dropping `lang="scss"`, swapping `var(textPrimary)` → `var(--textPrimary)`. Original full component depends on missing aliases (`$helpers/tracker`, `$lib/ds/Checkbox`, `BenefitItem` from a sibling path), so end-to-end repro was not exercised against terser inside the sweep harness. No follow-up.

## Use cases

- [x] Element with class:directive bound to state and a child text expression that needs memo deps now merge into a single `$.template_effect` — codegen routes memoized text concat through `EmitState.shared_memo` instead of a separate `emit_effect_call_extern` to `after_update`; `emit_template_effect_with_memo` folds shared_memo's deps with `memo_attrs` and `regular_updates` into one effect (test: template_effect_merge_class_state_with_memo_text)

- [x] Full `typescript_invalid_feature` diagnostic parity with reference `remove_typescript_nodes.js`: parameter properties (`private readonly x` in constructor), decorators, accessor fields, enums, namespaces with non-type nodes. Single validator in `crates/svelte_analyze/src/validate/typescript.rs` wired from `validate_program` (`<script lang="ts">`), module-script branch (`<script module lang="ts">`), and `validate_standalone_module` (`.svelte.ts`). Tests: `typescript::parameter_property_accessibility`, `typescript::decorator`, `typescript::accessor_field`, `typescript::enum_declaration`, `typescript::namespace_with_value`; unit: `analyze_module_reports_typescript_parameter_property`.

- [ ] `generate.mjs` and `cli.mjs` (sweep) use `remove_typescript_nodes` to pre-strip TypeScript from `.svelte.ts` files before passing to reference `compileModule`; this fails for TypeScript parameter properties because `remove_typescript_nodes` throws `typescript_invalid_feature` instead of expanding them — layer: tooling (`tasks/generate_test_cases/generate.mjs`, `packages/svelte-rs2-sweep/cli.mjs`); repro/test: none; candidate specs: none; suggested spec: none (tooling-only fix)

- [ ] Element with both a `bind:` directive and a delegated event handler emits `$.bind_*` after `$.delegated`; reference compiler emits `$.delegated` after `$.bind_*` — layer: codegen; repro/test: none yet (observed in narrowed `store_runes_prop_assign_bind` original draft `<input bind:value={$x} onchange={reset}>`); candidate specs: `bind-directives.md`, `events.md`; suggested spec: `events.md`

- [x] Component invocation with a `{#snippet}` child whose body contains nested template descendants now allocates the host's anchor `node` ident before entering the snippet body, matching reference. `emit_component` pre-allocates the host anchor through `direct_anchor_expr` (static path) or `comment_anchor_node_name` (dynamic path) before calling `build_component_snippet_children`; `emit_dynamic_component` accepts the pre-allocated anchor name instead of allocating its own. Shared `IdentGen` counter for `node` now consumes the host's id first, then descends into snippet bodies. (test: `component_snippet_node_ident_ordering`)

- [x] `fragment_N` template-id counter aligned with reference for fragment-level `SingleExpr` / `SingleConcat` strategies. Implemented in `emit_fragment` in `crates/svelte_codegen_client/src/codegen/fragment/mod.rs`: consumes one phantom `gen_ident("fragment")` before strategy emission, mirroring reference `Fragment.js` `trimmed.length > 0` branch where the outer `fragment_N` ident is allocated and then shadowed by the inner `text` id when `use_space_template` holds. (test: `diagnose_fragment_id_after_component_with_snippet`)

- [x] Component invocation `bind:<prop>={localState}` setter emits `$.set(local, $$value, true)` for runes-mode `$state` local. Codegen `bind_prop.rs::emit_bind_identifier` for `ComponentBindTarget::Rune` passes `Arg::Bool(true)` as third argument (test: `component_bind_ref_state_flag`)

- [x] Component invocation prop ordering: every `bind:<prop>={...}` getter/setter pair lives in `ComponentPropsOutput::deferred_items` and is appended to `items` after all regular props/spreads, mirroring reference compiler's "delay bind props to avoid spread overwrite" (test: `component_bind_prop_order`)

- [x] `{#snippet}` declared as a direct fragment child of an `{#each}` block callback (alongside `{@render}` of the same snippet) emits the `const row = …` declaration inline at the top of the each callback's body — no `{ … }` wrapper, before sibling content. Layer: codegen. Test: `diagnose_snippet_inside_each_callback`.

- [x] In legacy mode, a snippet-param destructure leaf (`{#snippet row({ value })}`) accessed as `value.x` inside a child-component prop getter wraps as `value(), $.untrack(() => value().x)`. Test: `diagnose_legacy_snippet_param_member_to_component_prop`.

- [x] Object-literal property whose value is an anonymous `FunctionExpression` is printed as method shorthand: `{ run: function (x) { … } }` → `{ run(x) { … } }`. Mutation lives in `enter_object_property` hook (`crates/svelte_transform/src/transformer/mod.rs`) via `normalize_object_property_method_shorthand` — sets `ObjectProperty.method = true` when `kind == Init` and value is `FunctionExpression`, mirroring esrap. Script mode only. (test: `diagnose_js_object_method_shorthand`)

- [ ] Free-standing `//` line comment sitting between two top-level script imports gets emitted by reference attached to the first non-import statement inside `export default function App(...)` (e.g. before `const x = ...`), but our compiler preserves it in its source position between imports at module scope. Likely an OXC comment-attachment / printer effect when imports are hoisted ahead of the rest of the script body — layer: codegen (script JS print) or transform; repro/test: `diagnose_script_line_comment_between_imports` (ignored); candidate specs: none; suggested spec: none

- [x] Each-block child scope containing an `<input>` with both a reactive `value={…}` attribute and a sibling reactive `set_text` / `set_attribute` update now emits a single `template_effect` whose body holds both the input-value if-guard and the sibling updates, with `$.reset(label)` placed before it. Fix: `emit_bind_group_value` (`crates/svelte_codegen_client/src/codegen/attributes/bind/group.rs`) pushes the guarded `if (input_value !== …) { input.value = … }` into `state.update` instead of wrapping it in its own `$.template_effect(...)`; `pack_body` → `emit_template_effect_with_memo` folds it with sibling regular updates. (test: `diagnose_legacy_each_bind_group_input_value_merges_with_text`)


- [ ] Legacy-mode component-API export — `export const f = () => …` (and `export function`) — emits `$.bind_prop($$props, "f", f)` at the script tail (between `$.init()` and `return $.pop($$exports)`), so consumers can `bind:f={…}` on the component. Currently absent; only `var $$exports = { f }` is generated. Sibling of `specs/legacy-reactivity-system.md` line 102 dev-mode item but reproduces in non-dev too — layer: transform/codegen; repro/test: `legacy_export_const_emits_bind_prop` (ignored); candidate specs: `legacy-export-let.md` (declares `export const` Out of scope), `legacy-reactivity-system.md` (tracks only the dev-mode variant), `props-bindable.md`; suggested spec: none

- [x] HTML character references inside quoted attribute values are decoded by the parser. `decode_attribute_value` in `crates/svelte_parser/src/html.rs` mirrors reference `decode_character_references(_, is_attribute_value=true)`: terminated entities (`&nbsp;`, `&amp;`, `&lt;`, numeric `&#x3c;`) always decode; unterminated forms (`&amp`, `&nbsp`, …) decode only when the following byte is not `=` and not ASCII-alphanumeric. NAMED_ENTITIES table extended with the legacy unterminated variants from the HTML5 spec. Carrier: `StringAttribute.decoded: Option<String>` + `raw_value`/`value` helpers (mirror of `Text::decoded`); `ConcatPart::Static` is decoded at fill-site in `convert_concat_parts`; `StyleDirectiveValue::String` decoded the same way. HTML-template attribute escape in `template.rs::escape_html_attr` extended to escape `<` to `&lt;` (matches reference `escape_html(value, true)`). All string-emitting consumers in codegen switched from `source_text(value_span)` to `sa.value(source)`. Tests: `diagnose_attribute_entity_decoding`, `diagnose_attribute_entity_strict_mode`, parser unit `html::tests::attr_*` (6 cases).

## Test cases

- [x] template_effect_merge_class_state_with_memo_text
- [x] typescript::parameter_property_accessibility
- [x] typescript::decorator
- [x] typescript::accessor_field
- [x] typescript::enum_declaration
- [x] typescript::namespace_with_value
- [ ] generate_mjs_ts_parameter_property_tooling
- [ ] bind_value_and_delegated_event_emit_order
- [x] component_snippet_node_ident_ordering
- [x] diagnose_fragment_id_after_component_with_snippet
- [x] component_bind_ref_state_flag
- [x] component_bind_prop_order
- [x] diagnose_snippet_inside_each_callback
- [x] diagnose_legacy_snippet_param_member_to_component_prop
- [x] diagnose_js_object_method_shorthand
- [ ] diagnose_script_line_comment_between_imports
- [x] diagnose_legacy_each_bind_group_input_value_merges_with_text
- [ ] legacy_export_const_emits_bind_prop
- [x] diagnose_attribute_entity_decoding
- [x] diagnose_attribute_entity_strict_mode
