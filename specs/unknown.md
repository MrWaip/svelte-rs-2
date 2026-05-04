# Unknown problems

## Current state
- **Working**: 3/11 use cases
- **Tests**: 1/5 green
- Last updated: 2026-05-04

## Source

- User request: create a durable triage spec for problems that do not yet map to one owning feature spec
- `/diagnose` benchmark component (dev=true, runes=true, customElement=true) — broad repro `diagnose_runes_dev_ce_benchmark`
- `/diagnose` benchmark component (dev=true only, no runes/customElement overrides) — narrower repro `diagnose_dev_benchmark` exercising the same dev-codegen mismatches without CE/runes noise

## Use cases

- [ ] dev-mode `==` and `===` comparisons in template/snippet expressions are not wrapped with `$.equals` / `$.strict_equals`; layer: transform; repro/test: diagnose_runes_dev_ce_benchmark, diagnose_dev_benchmark; candidate specs: text-expression-tag.md, if-block.md; suggested spec: none
- [ ] `$props()` source-line argument passed to `$.prop($$props, ..., flags, default)` and the location array passed to `$.add_locations(..., [[line, col], ...])` are off (props lines off by 4, `<svelte:head>` array contains a phantom head-root entry, named-slot inner element location dropped to `[]`); layer: codegen; repro/test: diagnose_runes_dev_ce_benchmark, diagnose_dev_benchmark; candidate specs: source-maps.md, props-bindable.md, element.md, legacy-slots.md; suggested spec: none
- [ ] Dev-mode named-slot child arrow on a static component is incorrectly wrapped with `$.wrap_snippet(App, ($$anchor, $$slotProps) => { ... })`; reference emits the bare arrow for `$$slots: { footer: ($$anchor, $$slotProps) => { ... } }` and only wraps the synthesized default-children entry. Inverse of `component_dev_default_children_wrap_snippet`; layer: transform; repro/test: diagnose_dev_benchmark, diagnose_runes_dev_ce_benchmark; candidate specs: legacy-slots.md, component-node.md; suggested spec: legacy-slots.md
- [ ] `$state.raw({...})` declarator in a script that combines `$props()` rest, dev mode, and `customElement: true` is emitted as a plain object literal instead of `$.tag($.state({...}), "name")`, and the corresponding `$state.snapshot(rawData)` reads `rawData` directly instead of `$.get(rawData)`; not reproducible in isolation, only in the combined benchmark; layer: transform; repro/test: diagnose_runes_dev_ce_benchmark; candidate specs: state-rune.md, custom-elements.md; suggested spec: state-rune.md
- [x] Dev-mode console method calls referencing reactive state are wrapped via `$.log_if_contains_state(method, ...args)` (e.g. `console.log("count:", count)` → `console.log(...$.log_if_contains_state("log", "count:", $.get(count)))`); currently not emitted on the `.svelte.js` / `.svelte.ts` standalone module path — layer: codegen + transform; repro/test: `module_dev_console_log_wrap`; candidate specs: `inspect-runes.md` (related but only covers `$inspect`), none cover console-method auto-instrumentation; suggested spec: new `dev-console-instrumentation.md` covering `console.{log,debug,info,warn,error,trace,dir,group,groupCollapsed}` dev wrapping for both component scripts and `.svelte.js` modules — closed 2026-05-04: `module_dev_console_log_wrap` passes; module-side console wrapping is emitted via the threaded `dev` flag from use case below
- [x] `compile_module` (`.svelte.js` / `.svelte.ts`) does not thread `dev` flag into the codegen-side transform pipeline — `svelte_codegen_client::generate_module` discards `dev`, and `script::pipeline::transform_module_program` hardcodes `dev: false` into `run_transform`. Cross-cutting: this is the shared root cause for `module_dev_state_tag` (owned by `state-rune.md`), `module_dev_derived_tag` (owned by `derived-state.md`), and `module_dev_console_log_wrap` (above) — layer: codegen; repro/test: any of the three above; candidate specs: `state-rune.md` + `derived-state.md` already track their slice, this entry tracks the shared infrastructure fix — closed 2026-05-04: `compile_module` (`crates/svelte_compiler/src/lib.rs:274`) threads `dev` into `generate_module` (`crates/svelte_codegen_client/src/lib.rs:610`) and `transform_module_program` (`crates/svelte_codegen_client/src/script/pipeline.rs:70`); all three dependent tests pass
- [ ] CSS pipeline emits stylesheet content (the value of `$$css.code`) collapsed onto a single line; reference compiler preserves original source whitespace and comment markers; layer: css-pipeline; repro/test: `diagnose_runes_dev_ce_benchmark`; candidate specs: `css-pipeline.md`; suggested spec: `css-pipeline.md`
- [ ] Instance-script leading JSDoc / line comments on simple declarations (e.g. `/** @type {Function | undefined} */ let show;`) are stripped during script lowering; reference retains them; layer: codegen/script; repro/test: `diagnose_runes_dev_ce_benchmark`; candidate specs: none specifically for comment retention; suggested spec: none — needs new comment-retention spec or extend script lowering doc
- [x] `validate_options_custom_element_warns_without_compiler_flag` diagnostic emits span 0..0 instead of spanning the `customElement` option attribute as reference does; layer: analyze (validate); repro/test: `validate_options_custom_element_warns_without_compiler_flag`; candidate specs: `diagnostics-infrastructure.md`, `custom-elements.md`; suggested spec: `diagnostics-infrastructure.md` — closed 2026-05-04: `crates/svelte_analyze/src/validate/mod.rs:410-423` builds the warning via `attr.span()`, diagnostic now produces span `16..37` matching reference
- [ ] `script-tag-with-script-wrapper` — template-level `<script>` element is not wrapped via `$.with_script(...)` and the closing `<!>` anchor is absent from our `from_html` template. Reference: `phases/3-transform/client/transform-template/index.js:47-55` wraps the builder call when `state.template.contains_script_tag` is true; `phases/3-transform/client/transform-template/template.js:12` documents `create_fragment_with_script_from_html` rationale. Repro/test: `preserve_whitespace_script_element` (currently `#[ignore]`); layer: codegen template-builder; suggested spec: new `script-template-emission.md` once audited.
- [ ] In non-runes mode (`runes:false`), `let x = $derived(expr)` raises `rune_invalid_usage` and aborts JS emission, while reference compiler tolerates `$derived(...)` (and other rune-shaped calls) by falling back to legacy store-subscription codegen — `$derived(expr)` lowers to `$derived()(expr)` where `$derived` is treated as a `$store` getter. Whole component cannot be compiled in legacy+dev mode because the analyzer hard-errors on the first declarator. First diverging diagnostic: `RuneInvalidUsage { rune: "$derived" }` from `crates/svelte_analyze/src/validate/runes.rs:744`. Other runes used in the repro (`$state`, `$state.raw`, `$props`, `$effect`, `$inspect`, `$bindable`) currently do not trigger this analyzer branch in legacy mode but would still need legacy lowering once the hard error is removed. Layer: analyze (validate/runes); repro/test: `diagnose_legacy_dev_benchmark`; candidate specs: `derived-state.md` (claims `[x] rune_invalid_usage in non-runes mode` but reference does not emit it), `legacy-reactivity-system.md`, `store-subscriptions.md`; suggested spec: `legacy-reactivity-system.md` (cross-cutting rune-vs-store fallback in legacy mode).

## Out of scope

- Implementing compiler fixes directly in this spec
- Keeping items here after they have been mapped to an owning feature spec

## Reference
### Svelte

- None. This spec is a project triage queue, not a language feature spec.

### Our code

- `ROADMAP.md`
- `.codex/skills/diagnose/SKILL.md`
- `.codex/skills/port/SKILL.md`
- `tasks/compiler_tests/test_v3.rs`
- `tasks/compiler_tests/cases2/`
- `crates/svelte_compiler/src/lib.rs:274` — `compile_module` threads `dev` to analyze and codegen
- `crates/svelte_codegen_client/src/lib.rs:610` — `generate_module` accepts and forwards `dev`
- `crates/svelte_codegen_client/src/script/pipeline.rs:70` — `transform_module_program` accepts `dev` and forwards to `run_transform`
- `crates/svelte_transform/src/transformer/inspect.rs` — `transform_console_log` (gated on `self.dev`)
- `crates/svelte_analyze/src/validate/mod.rs:410-423` — `OptionsMissingCustomElement` warning span via `attr.span()`

## Test cases
- [ ] `diagnose_runes_dev_ce_benchmark`
- [ ] `diagnose_dev_benchmark`
- [x] `module_dev_console_log_wrap`
- [ ] `preserve_whitespace_script_element`
- [ ] `diagnose_legacy_dev_benchmark`
