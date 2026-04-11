# Unknown problems

## Current state

- **Working**: 1 recorded unknown item
- **Next**: TS `$props` + `$bindable` checkbox binding emits non-reference codegen for `bind:checked` and `{disabled}` shorthand
- Last updated: 2026-04-11

## Source

- User request: create a durable triage spec for problems that do not yet map to one owning feature spec

## Use cases

- [ ] TS `$props` + `$bindable` checkbox binding emits non-reference codegen for `bind:checked` and `{disabled}` shorthand — layer: codegen; repro/test: props_bindable_checkbox_disabled_shorthand_ts; candidate specs: specs/props-bindable.md, specs/bind-directives.md; suggested spec: none

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
