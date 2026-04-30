# Unknown problems

## Current state
- **Working**: 0/5 use cases
- **Tests**: 0/2 green
- Last updated: 2026-04-30

## Source

- User request: create a durable triage spec for problems that do not yet map to one owning feature spec
- `/diagnose` benchmark component (dev=true, runes=true, customElement=true) — broad repro `diagnose_runes_dev_ce_benchmark`

## Use cases

- [ ] dev-mode `==` and `===` comparisons in template/snippet expressions are not wrapped with `$.equals` / `$.strict_equals`; layer: transform; repro/test: diagnose_runes_dev_ce_benchmark; candidate specs: text-expression-tag.md, if-block.md; suggested spec: none
- [ ] `$props()` source-line argument passed to `$.prop($$props, ..., flags, default)` and the location array passed to `$.add_locations(..., [[line, col], ...])` are off (props lines off by 4, `<svelte:head>` array contains a phantom head-root entry); layer: codegen; repro/test: diagnose_runes_dev_ce_benchmark; candidate specs: source-maps.md, props-bindable.md, element.md; suggested spec: none
- [ ] `$state.raw({...})` declarator in a script that combines `$props()` rest, dev mode, and `customElement: true` is emitted as a plain object literal instead of `$.tag($.state({...}), "name")`, and the corresponding `$state.snapshot(rawData)` reads `rawData` directly instead of `$.get(rawData)`; not reproducible in isolation, only in the combined benchmark; layer: transform; repro/test: diagnose_runes_dev_ce_benchmark; candidate specs: state-rune.md, custom-elements.md; suggested spec: state-rune.md
- [ ] Dev-mode console method calls referencing reactive state are wrapped via `$.log_if_contains_state(method, ...args)` (e.g. `console.log("count:", count)` → `console.log(...$.log_if_contains_state("log", "count:", $.get(count)))`); currently not emitted on the `.svelte.js` / `.svelte.ts` standalone module path — layer: codegen + transform; repro/test: `module_dev_console_log_wrap`; candidate specs: `inspect-runes.md` (related but only covers `$inspect`), none cover console-method auto-instrumentation; suggested spec: new `dev-console-instrumentation.md` covering `console.{log,debug,info,warn,error,trace,dir,group,groupCollapsed}` dev wrapping for both component scripts and `.svelte.js` modules
- [ ] `compile_module` (`.svelte.js` / `.svelte.ts`) does not thread `dev` flag into the codegen-side transform pipeline — `svelte_codegen_client::generate_module` discards `dev`, and `script::pipeline::transform_module_program` hardcodes `dev: false` into `run_transform`. Cross-cutting: this is the shared root cause for `module_dev_state_tag` (owned by `state-rune.md`), `module_dev_derived_tag` (owned by `derived-state.md`), and `module_dev_console_log_wrap` (above) — layer: codegen; repro/test: any of the three above; candidate specs: `state-rune.md` + `derived-state.md` already track their slice, this entry tracks the shared infrastructure fix

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

## Test cases
- [ ] `diagnose_runes_dev_ce_benchmark`
- [ ] `module_dev_console_log_wrap`
