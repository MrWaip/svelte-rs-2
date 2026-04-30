# Unknown problems

## Current state
- **Working**: 0/4 use cases
- **Tests**: 0/2 green
- Last updated: 2026-04-30

## Source

- User request: create a durable triage spec for problems that do not yet map to one owning feature spec
- `/diagnose` benchmark component (dev=true, runes=true, customElement=true) — broad repro `diagnose_runes_dev_ce_benchmark`

## Use cases

- [ ] dev-mode `==` and `===` comparisons in template/snippet expressions are not wrapped with `$.equals` / `$.strict_equals`; layer: transform; repro/test: diagnose_runes_dev_ce_benchmark; candidate specs: text-expression-tag.md, if-block.md; suggested spec: none
- [ ] `$props()` source-line argument passed to `$.prop($$props, ..., flags, default)` and the location array passed to `$.add_locations(..., [[line, col], ...])` are off (props lines off by 4, `<svelte:head>` array contains a phantom head-root entry); layer: codegen; repro/test: diagnose_runes_dev_ce_benchmark; candidate specs: source-maps.md, props-bindable.md, element.md; suggested spec: none
- [ ] `$state.raw({...})` declarator in a script that combines `$props()` rest, dev mode, and `customElement: true` is emitted as a plain object literal instead of `$.tag($.state({...}), "name")`, and the corresponding `$state.snapshot(rawData)` reads `rawData` directly instead of `$.get(rawData)`; not reproducible in isolation, only in the combined benchmark; layer: transform; repro/test: diagnose_runes_dev_ce_benchmark; candidate specs: state-rune.md, custom-elements.md; suggested spec: state-rune.md
- [ ] `.svelte.js` standalone module compiled with `dev: true` drops dev-only wrappers — `$state`/`$derived` declarations are not wrapped in `$.tag(value, "name")`, and `console.log` calls referencing module-level state are not wrapped in `$.log_if_contains_state(...)` — layer: codegen (`svelte_codegen_client::generate_module` discards `dev`; `transform_module_program` hardcodes `dev: false` into `run_transform`, so transform-side `$.tag` and `transform_console_log` paths in `svelte_transform` never run for module path); repro/test: `module_compilation_dev`; candidate specs: `script-module.md` (explicitly excludes `.svelte.js`), `state-rune.md`, `derived-state.md`, `inspect-runes.md`; suggested spec: new `module-js-dev.md` covering `.svelte.js` / `.svelte.ts` dev-mode transforms

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
- `crates/svelte_compiler/src/lib.rs` — `compile_module` passes `dev` only to analyze
- `crates/svelte_codegen_client/src/lib.rs` — `generate_module` discards `dev`
- `crates/svelte_codegen_client/src/script/pipeline.rs` — `transform_module_program` hardcodes `dev: false`
- `crates/svelte_transform/src/transformer/inspect.rs` — `transform_console_log` (gated on `self.dev`)
- `crates/svelte_transform/src/transformer/state.rs` / `derived.rs` — `$.tag` wrapping for dev

## Test cases
- [ ] `diagnose_runes_dev_ce_benchmark`
- [ ] `module_compilation_dev`
