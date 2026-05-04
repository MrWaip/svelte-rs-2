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

## Benchmarks

Walltime comparison against `svelte/compiler` on `tasks/benchmark/benches/compiler/**`. Measured 2026-05-04 via `just bench-compare-walltime` (3 s budget per side, 0.5 s warmup, min 5 iters, `NeverGrowInPlaceAllocator`). Times are per single `compile()` / `compileModule()` call.

| case | rust med | rust mean | rust n | js med | js mean | js n | speedup (med) |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| compile[benches/compiler/big_v6.svelte] | 18.39 | 18.83 | 160 | 955.9 | 976.7 | 5 | 51.97x |
| compile_dev[benches/compiler/big_v6.svelte] | 19.56 | 19.96 | 151 | 944.7 | 960.2 | 5 | 48.30x |
| compile_module[benches/compiler/module/case_02.svelte.js] | 0.1309 | 0.1360 | 21998 | 5.283 | 5.603 | 536 | 40.37x |
| compile_module_dev[benches/compiler/module/case_02.svelte.js] | 0.1337 | 0.1352 | 22134 | 5.300 | 5.694 | 527 | 39.64x |
| compile_module[benches/compiler/module/case_04.svelte.js] | 0.2580 | 0.2603 | 11350 | 10.05 | 10.09 | 298 | 38.95x |
| compile_module[benches/compiler/module/case_03.svelte.js] | 0.2529 | 0.2552 | 11578 | 9.631 | 9.821 | 306 | 38.09x |
| compile_module_dev[benches/compiler/module/case_03.svelte.js] | 0.2577 | 0.2602 | 11340 | 9.741 | 9.821 | 306 | 37.80x |
| compile_module_dev[benches/compiler/module/case_04.svelte.js] | 0.2691 | 0.2720 | 10850 | 10.15 | 10.14 | 296 | 37.72x |
| compile_module[benches/compiler/module/case_01.svelte.js] | 0.1398 | 0.1426 | 20987 | 5.034 | 5.319 | 565 | 36.00x |
| compile_module_dev[benches/compiler/module/case_01.svelte.js] | 0.1436 | 0.1484 | 20168 | 4.919 | 5.247 | 572 | 34.25x |
| compile_dev[benches/compiler/template/case_02.svelte] | 0.6525 | 0.6553 | 4575 | 20.01 | 20.45 | 147 | 30.66x |
| compile_dev[benches/compiler/template/case_01.svelte] | 0.7388 | 0.7414 | 4043 | 22.33 | 22.47 | 134 | 30.22x |
| compile[benches/compiler/template/case_02.svelte] | 0.6415 | 0.6453 | 4612 | 18.73 | 19.08 | 158 | 29.20x |
| compile[benches/compiler/template/case_01.svelte] | 0.7445 | 0.7474 | 4011 | 21.65 | 22.11 | 136 | 29.08x |
| compile_dev[benches/compiler/mixed/case_01.svelte] | 0.4584 | 0.4632 | 6456 | 11.57 | 11.54 | 260 | 25.23x |
| compile[benches/compiler/mixed/case_01.svelte] | 0.4577 | 0.4606 | 6484 | 11.50 | 11.59 | 259 | 25.11x |
| compile[benches/compiler/reactivity/case_01.svelte] | 0.4019 | 0.4124 | 7185 | 9.539 | 9.497 | 316 | 23.73x |
| compile[benches/compiler/legacy/case_01.svelte] | 0.4398 | 0.4517 | 6623 | 10.43 | 10.64 | 282 | 23.71x |
| compile_dev[benches/compiler/reactivity/case_01.svelte] | 0.4229 | 0.4255 | 6951 | 9.854 | 9.901 | 303 | 23.30x |
| compile_dev[benches/compiler/legacy/case_01.svelte] | 0.4397 | 0.4429 | 6758 | 10.06 | 10.09 | 298 | 22.88x |
| compile[benches/compiler/snippets/case_01.svelte] | 1.558 | 1.569 | 1900 | 35.62 | 36.36 | 83 | 22.86x |
| compile_dev[benches/compiler/snippets/case_01.svelte] | 1.628 | 1.663 | 1803 | 36.34 | 36.63 | 82 | 22.32x |
| compile_dev[benches/compiler/css/case_01.svelte] | 0.0147 | 0.0156 | 189504 | 0.2988 | 0.3243 | 9244 | 20.32x |
| compile[benches/compiler/css/case_01.svelte] | 0.0144 | 0.0147 | 202196 | 0.2903 | 0.3338 | 8980 | 20.14x |

All times in ms. **geomean speedup: 30.18x** (n=24), min 20.14x, max 51.97x.

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

### Diagnostic Parity
1. `/add-diagnostic-test <name>` — create a focused diagnostic parity case under `tasks/diagnostic_tests/`
2. `/diagnose-diagnostics <component|case>` — isolate a false positive, false negative, or span mismatch against npm `svelte/compiler`
3. `/port specs/<file>.md` — implement the owning fix after the mismatch is reduced to one durable case

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
- `/add-diagnostic-test <name>` — test-first: create a diagnostic parity case before implementation

### Snapshot Generation
- `just generate` — regenerate reference snapshots for both `tasks/compiler_tests/` and `tasks/diagnostic_tests/`

---

## Building the WASM package

```sh
wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
```

Or `just playground` to build WASM and serve the playground locally.

## Native Node.js bindings (NAPI)

A native addon for Node.js consumers lives in `crates/napi_compiler` and is published as `svelte-rs2` from `packages/`.

- `just npm-smoke` — debug build + local link + smoke test
- `just npm-build` — release build + tarballs for platform packages

## Quick check against reference compiler

`just quick-check path/to/component.svelte` — compile a single component and diff against `svelte/compiler` output.

## Useful dev commands

- `just test-compiler` — run all compiler integration tests
- `just test-case <name>` / `just test-case-verbose <name>` — single test case
- `just test-diagnostics` / `just test-diagnostic-case <name>` — diagnostic parity tests
- `just clippy-strict` — clippy with `-D warnings`
- `just generate` — regenerate reference snapshots
- `just dump-ast '<expr>'` — dump OXC ESTree JSON for a JS expression
