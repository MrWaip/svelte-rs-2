# Unknown problems

## Current state
- **Working**: 0/1 use cases
- **Tests**: 0/1 green
- Last updated: 2026-04-30

## Source

- User request: create a durable triage spec for problems that do not yet map to one owning feature spec

## Use cases

- [ ] JSDoc `/** @type ... */` annotation on a script-level `let` declaration leaks into emitted client JS — layer: codegen; repro/test: only reproduces inside the large `/diagnose` benchmark component (script with stores + runes + `bind:group` + `let show;` annotated). Could not reduce to a focused isolated case during diagnose; symptom may resolve once `bind_group_order_with_stores` is fixed since both originate from the same instance-body splice region. Candidate specs: typescript-script-stripping, bind-directives; suggested spec: typescript-script-stripping

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
- [ ] JSDoc `@type` leak — broad-repro only; revisit after `bind_group_order_with_stores` lands
