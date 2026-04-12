# $: reactive assignments

## Current state
- **Working**: 0/3 use cases
- **Next**: audit the missing legacy-mode `$:` parser/analyze/codegen flow, then add focused compiler coverage before implementation
- **Moved (2026-04-12)**:
  - Deferred legacy `$:` `$.deep_read_state()` ownership moved here from `specs/state-rune.md` so `$state` can stay scoped to client-side rune parity
- **Notes**:
  - The repo already rejects `$:` in runes mode via `legacy_reactive_statement_invalid`
  - Existing `$.deep_read_state(...)` coverage in `svelte_options_immutable_legacy` only proves coarse legacy immutable template reads, not full legacy `$:` support
- Last updated: 2026-04-12

## Source

- ROADMAP item: `Legacy Svelte 4 -> $: reactive assignments`
- Moved out of `specs/state-rune.md` during spec normalization on 2026-04-12

## Syntax variants

- `$: doubled = count * 2;`
- `$: console.log(items.length);`
- `$: ({ value } = source);`

## Use cases

- [ ] Legacy `$:` statements in non-runes components are discovered, ordered, and emitted through the legacy client runtime path.
- [ ] Legacy `$:` dependency tracking uses coarse-grained reads where the reference compiler does, including `$.deep_read_state(...)` for bindable props, template bindings, imports, `$$props`, and `$$restProps`.
- [ ] Legacy reactive assignments propagate updates through downstream `$:` declarations in topological order instead of running as isolated statements.

## Out of scope

- SSR behavior for legacy reactive statements
- Extending `$state` rune semantics beyond their existing client-side behavior
- Unowned legacy features outside `$:` reactive assignments

## Reference

- Reference compiler:
  - `reference/compiler/phases/3-transform/client/visitors/LabeledStatement.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/utils.js`
- Existing Rust behavior:
  - `crates/svelte_codegen_client/src/template/expression.rs`
  - `crates/svelte_diagnostics/src/lib.rs`
  - `specs/state-rune.md`
  - `specs/svelte-options.md`

## Test cases

- [ ] `legacy_reactive_assignment_basic`
- [ ] `legacy_reactive_assignment_props_deep_read`
- [ ] `legacy_reactive_assignment_topological_order`
