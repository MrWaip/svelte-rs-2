# Unknown problems

## Current state

- **Working**: 2 recorded unknown items
- **Next**: TS `$props` + `$bindable` checkbox binding emits non-reference codegen for `bind:checked` and `{disabled}` shorthand
- **Moved (2026-04-11)**: Inspecting rune values in `$inspect(...)` incorrectly warning `state_referenced_locally` now belongs to `specs/inspect-runes.md`
- **Moved (2026-04-11)**: Input shorthand `{value}` / `{disabled}` on `<input>` from `$props()` now belongs to `specs/attributes-spreads.md`

## Source

- User request: create a durable triage spec for problems that do not yet map to one owning feature spec

## Use cases

- [ ] TS `$props` + `$bindable` checkbox binding emits non-reference codegen for `bind:checked` and `{disabled}` shorthand — layer: codegen; repro/test: props_bindable_checkbox_disabled_shorthand_ts; candidate specs: specs/props-bindable.md, specs/bind-directives.md; suggested spec: none
- [ ] TS script comment leaks into client output and perturbs template cursor state — layer: codegen; repro/test: diagnose_svg_city_icon; candidate specs: none; suggested spec: typescript-script-stripping

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
- [ ] props_bindable_checkbox_disabled_shorthand_ts
- [ ] diagnose_svg_city_icon
