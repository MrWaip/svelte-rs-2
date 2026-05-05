# Auto-mode detection (runes vs legacy)

## Current state

- **Working**: explicit `<svelte:options runes={true|false}>`, explicit `CompileOptions.runes: RunesOption::{Runes, Legacy}`, auto-detection (`RunesOption::Auto`) via rune-name in `data.scoping.root_unresolved_references()` and top-level `await`. Shadowing handled by scope graph.
- **Tri-state runtime mode**: `RunesMode { Runes, SoftLegacy, HardLegacy }` lives in `svelte_ast`; resolved value in `data.script.runes_mode`. `Runes` = runes mode; `SoftLegacy` = legacy without legacy-signals (no `export let`, `$:`, `$$props`/`$$restProps`) and runes not explicitly disabled; `HardLegacy` = explicit legacy or legacy with legacy-signals.
- **Tests green**: 12 analyze-level (`maybe_runes_resolution`) + 5 compiler unit tests + 4 e2e cases (`auto_softlegacy_*`, `auto_hardlegacy_member_read_explicit`).
- **Pass placement**: `BuildReactivitySemantics` is the second pass, depends only on `ComponentSemantics`. Mode resolution is private to `reactivity_semantics::mode_resolution`.
- **Last updated**: 2026-05-05.

## Source

- ROADMAP item: `Legacy Svelte 4 -> Auto-mode detection (runes vs legacy)`
- Adjacent specs:
  - `specs/legacy-reactivity-system.md` — owns the HardLegacy template-expression coarse-grained sequence wrap.
  - `specs/svelte-options.md` — owns `<svelte:options runes={...}>` parsing.

## How it works

`mode_resolution::resolve(scoping, parsed, inline, compile) -> RunesMode`:

1. `inline = Some(true)` → `Runes`.
2. `inline = Some(false)` → `HardLegacy`.
3. `compile = Runes` → `Runes`. `compile = Legacy` → `HardLegacy`.
4. Auto-detect (otherwise):
   - any `is_rune_name(name)` in `scoping.root_unresolved_references()` → `Runes`.
   - top-level `await` in module or instance script → `Runes`.
   - else: legacy. Choose `HardLegacy` if any legacy signal (`$$props`/`$$restProps` in unresolved refs, top-level `export let`, `export { x }` where `x` is `let`-bound, `$:` labeled statement); else `SoftLegacy`.

`ScriptAnalysis` exposes `runes() -> bool` and `maybe_runes() -> bool` derived from `runes_mode`.

## Use cases

- [x] Inline `<svelte:options runes={true|false}>` overrides compile option.
- [x] `CompileOptions.runes: RunesOption::{Auto, Runes, Legacy}` with default `Auto`.
- [x] Auto-detect: rune-name in module-scope unresolved refs → `Runes`.
- [x] Auto-detect: top-level `await` (module or instance) → `Runes`.
- [x] Auto-detect: shadowed rune name (e.g. `let $state = ...`) does not flip mode.
- [x] `HardLegacy` chosen when any of `export let`, `$:`, `$$props`, `$$restProps`, or explicit disable.
- [x] `SoftLegacy` chosen when legacy with no legacy signals and runes not explicitly disabled.
- [ ] Downstream effect of `HardLegacy`: template-expression coarse-grained sequence wrap `($.get(x), $.untrack(() => …))` for member/call/assignment expressions reading legacy-state. Test: `auto_hardlegacy_member_read_explicit` (currently `#[ignore]`). Owning area: `specs/legacy-reactivity-system.md`.

## Test cases

- `tasks/compiler_tests/cases2/auto_softlegacy_member_read` — Auto + member-read with mutation, no legacy signals → SoftLegacy.
- `tasks/compiler_tests/cases2/auto_softlegacy_simple_template` — Auto + simple identifier read → SoftLegacy.
- `tasks/compiler_tests/cases2/auto_softlegacy_const_only` — Auto + only `const` declarations → SoftLegacy.
- `tasks/compiler_tests/cases2/auto_hardlegacy_member_read_explicit` — explicit `runes: false` + member-read with mutation → HardLegacy. Currently `#[ignore]` until coarse-grained wrap lands.
- `crates/svelte_analyze/src/tests.rs::maybe_runes_resolution::*` — 12 analyze-level cases covering each branch of the resolver.
- `crates/svelte_compiler/src/tests.rs::auto_mode_*` and `inline_runes_option_overrides_compile_option` — 6 compile()-level cases.

## Reference

- `reference/compiler/phases/2-analyze/index.js:451-515` — auto-detect + maybe_runes resolution.
- `reference/compiler/phases/3-transform/client/visitors/shared/utils.js:446-454` — `runes || maybe_runes` consumer in `build_expression`.
- `reference/compiler/phases/scope.js:1033-1041, 1433-1469` — top-level await as runes signal, shadowing.
