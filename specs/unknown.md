# Unknown problems

## Current state

- **Working**: 2 recorded unknown items
- **Next**: Input shorthand `{value}` / `{disabled}` on `<input>` from `$props()` lowers as generic attrs instead of input-special runtime updates
- Last updated: 2026-04-11

## Source

- User request: create a durable triage spec for problems that do not yet map to one owning feature spec

## Use cases

- [ ] Input shorthand `{value}` / `{disabled}` on `<input>` from `$props()` should lower through input-special value/boolean paths (`$.remove_input_defaults`, `$.set_value`, `input.disabled = ...`) but currently compiles as generic attributes — layer: analysis; repro/test: diagnose_props_bindable_icon_component; candidate specs: bind-directives.md, element.md; suggested spec: bind-directives.md
- [ ] Inspecting `$derived` in `$inspect(...)` incorrectly warns `state_referenced_locally` — layer: analysis; repro/test: validate_inspect_derived_no_state_referenced_locally_warning; candidate specs: inspect-runes.md, derived-state.md; suggested spec: inspect-runes.md
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
- [ ] diagnose_props_bindable_icon_component
- [ ] validate_inspect_derived_no_state_referenced_locally_warning
