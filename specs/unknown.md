# Unknown problems

## Current state

- **Working**: 0 recorded unknown items
- **Next**: complete; add new items here only until they are mapped to an owning feature spec
- **Moved (2026-04-11)**: Inspecting rune values in `$inspect(...)` incorrectly warning `state_referenced_locally` now belongs to `specs/inspect-runes.md`
- **Moved (2026-04-11)**: Input shorthand `{value}` / `{disabled}` on `<input>` from `$props()` now belongs to `specs/attributes-spreads.md`
- **Moved (2026-04-11)**: `props_bindable_checkbox_disabled_shorthand_ts` now belongs to `specs/bind-directives.md`; the live gap is bindable-prop `bind:checked` lowering, while `{disabled}` shorthand was already closed by `specs/attributes-spreads.md`
- **Moved (2026-04-11)**: TS script comment leakage from `diagnose_svg_city_icon` now belongs to `specs/typescript-script-stripping.md`

## Source

- User request: create a durable triage spec for problems that do not yet map to one owning feature spec

## Use cases

- None currently recorded

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

## Test cases
- None currently recorded
