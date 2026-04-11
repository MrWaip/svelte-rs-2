# Unknown problems

## Current state

- **Working**: 1 recorded unknown item
- **Next**: TS script comment leaks into client output and perturbs template cursor state
- Last updated: 2026-04-11

## Source

- User request: create a durable triage spec for problems that do not yet map to one owning feature spec

## Use cases

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

- [ ] diagnose_svg_city_icon
