# svelte-rs — Rust implementation of the Svelte compiler (WIP)

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/MrWaip/svelte-rs-2)

## Demo

https://mrwaip.github.io/svelte-rs-2/

## Architecture overview

https://excalidraw.com/#json=tPR4IJ3ZQmfRfF0xW1fif,Qw3c1g41YuyCLz1XmRcujw

---

## Feature checklist

See [ROADMAP.md](./ROADMAP.md) for the full feature checklist.

---

## Workflow

This project uses Claude Code with a set of specialized commands and agents.

### Session Start
`/status` — project overview: active specs, ignored tests, next ROADMAP item, known debt

### Feature Porting
1. `/audit <feature>` — gap analysis, create a spec and tests
2. `/port specs/<file>.md` — implement the next slice from the spec
3. `/qa` — review for material quality issues
4. `/sync-docs` — sync ROADMAP and CODEBASE_MAP

### Test Triage
1. `/explain-test <name>` (optional — understand what the test covers)
2. `/triage-test <name>` — classify the work as `local-fix`, `slice-gap`, or `spec-gap`
3. `/qa` (optional)

### Tech Debt / Refactoring
1. `/improve <description>` — diagnosis, fix, and tests
2. `/qa`

### Investigation
- `/diagnose <component>` — run the repro through the pipeline, isolate the root cause, add focused tests, and record follow-up work in a spec or `ROADMAP.md`
- `/audit <feature>` — gap analysis vs the reference compiler
- `/explain-test <name>` — what the test does and why it fails
- `/bench` — Rust vs JS performance

### Maintenance
- `/sync-docs` — synchronize documentation with the code
- `/add-test <name>` — test-first: create a test before implementation

---

## Building the WASM package

```sh
wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
```
