<div align="center">

# svelte-rs

**The Svelte 5 compiler, rewritten in Rust.**

Drop-in replacement for `svelte/compiler` (pinned to **svelte@5.56.4**) — same JS output, ~13× faster.

[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/MrWaip/svelte-rs-2)
[![npm](https://img.shields.io/badge/npm-%40mrwaip%2Fsvelte--rs2-cb3837)](https://www.npmjs.com/package/@mrwaip/svelte-rs2)
[![vite-plugin](https://img.shields.io/npm/v/@mrwaip/vite-plugin-svelte?label=vite-plugin-svelte&color=646cff)](https://www.npmjs.com/package/@mrwaip/vite-plugin-svelte)
[![tests](https://img.shields.io/badge/e2e_tests-10k%2B-success)](./tasks/compiler_tests/cluster_cases)

[Playground](https://mrwaip.github.io/svelte-rs-2/) · [Issues](https://github.com/MrWaip/svelte-rs-2/issues)

> ⚠️ **WIP / canary.** Built by a human with heavy AI assistance. Expect bugs, missing edge cases, and breaking changes. Not production-ready — please report what breaks.

</div>

---

## Why

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./assets/benchmark.svg">
  <source media="(prefers-color-scheme: light)" srcset="./assets/benchmark-light.svg">
  <img src="./assets/benchmark.svg" alt="svelte-rs compiler benchmark — ~13× faster than svelte/compiler, ~13× faster than rsvelte" width="720">
</picture>

- **Byte-exact JS output** — **~2,600 cases** (1,404 cluster + 1,199 legacy), each diffed against the reference compiler in all four modes (client/server × dev/prod) — **10,000+ passing comparisons**.
- **Diagnostics parity** — **731 cases** across 26 categories (a11y, runes, CSS, TypeScript, …), matched against `svelte/compiler`'s own warnings and errors by code, severity, and span.
- **Drop-in** — same `compile()` / `compileModule()` API as `svelte/compiler`.
- **~13× faster** across the ~2,950-file corpus (client 13.0×, server 14.1×, dev variants 12.6–14.2×).
- **Ready to try** — wired into a fork of `vite-plugin-svelte`, so it runs in a real Vite app today.

## Install

### As a compiler

```sh
npm i -D @mrwaip/svelte-rs2
```

```js
import { compile } from '@mrwaip/svelte-rs2/compiler';

const { js } = compile(
  `<script>let { name } = $props();</script><h1>hello {name}</h1>`,
  { filename: 'Hello.svelte', generate: 'client' }
);

console.log(js.code);
```

The API mirrors `svelte/compiler`; see `packages/svelte-rs2/compiler/index.d.ts`. A few opt-in extras beyond the reference API:

- **`warningFilter: (warning) => boolean`** — matches Svelte 5's option; drops warnings the predicate rejects.
- **`suppress: WarningCode[]`** — a typed list of warning codes dropped at the source. Cheaper than filtering after the fact: suppressed warnings are never built, framed, or serialized.
- **`withDiagnostics: true`** — returns `{ js, css, diagnostics }` where each diagnostic carries `severity: 'error' | 'warning'`, and never throws on error. Handy for editors, linters, and batch tooling.

Real input → output for every mode lives in [`tasks/compiler_tests/cluster_cases/`](./tasks/compiler_tests/cluster_cases): each leaf has `case.svelte` plus our output and the reference (`case-rust.js` / `case-svelte.js`, `.dev.js`, `.server.js`, `.server.dev.js`). The older flat [`cases2/`](./tasks/compiler_tests/cases2) suite is legacy but still checked.

### In a Vite app

The fork of `vite-plugin-svelte` routes `compile` / `compileModule` through `@mrwaip/svelte-rs2` automatically, falling back to `svelte/compiler` for options the Rust side doesn't support yet.

```sh
npm i -D @mrwaip/vite-plugin-svelte
```

```js
import { defineConfig } from 'vite';
import { svelte } from '@mrwaip/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
});
```

Source: <https://github.com/MrWaip/vite-plugin-svelte>.

### Requirements

| | |
| --- | --- |
| Node | `^20.19 \|\| ^22.12 \|\| >=24` |
| Platforms | Prebuilt binaries for macOS (arm64/x64), Linux glibc & musl/Alpine (x64/arm64), Windows (x64/arm64). |
| Peer | `svelte@5.56.4` — Vite plugin falls back to `svelte/compiler` for unsupported options. |

## Status

Parity target: **svelte@5.56.4**. Check this before logging an issue.

<details>
<summary><b>Feature matrix &amp; known limitations</b></summary>

| Feature | Status | Notes |
| --- | --- | --- |
| Svelte 5 syntax | done | Runes, template, bindings, directives, events, special elements, diagnostics, a11y. |
| Svelte 4 legacy | done | `export let`, `$:`, `beforeUpdate`/`afterUpdate`, `<slot>`, `<svelte:self>`, `<svelte:component>`, auto-mode detection. |
| CSS pipeline | done | analyze + transform + codegen. |
| TypeScript | done | Script stripping only — no type checking. |
| `.svelte.js` / `.svelte.ts` modules | done | — |
| Dev mode (`dev: true`) | done | Byte-exact client-dev and server-dev; some ownership/hydration diagnostics land case-by-case. |
| SSR (`generate: 'server'`) | done | Server transform + codegen (prod & dev). |
| Source maps | done | `js.map` / `css.map` emitted; `sourcemap` option honored. |
| HMR | done | Hot-module-replacement output supported. |
| Custom elements | in progress | Basic path works; some option combinations not covered. |
| Compiler options | in progress | Most common options honored (incl. `discloseVersion`, `warningFilter`); long tail still landing. |
| Preprocessors | not ready | Not started. |

**done** — no known deficits; OK to log bugs. **in progress** — partially working; log panics only. **not ready** — don't file bugs yet.

**Known limitations**

- `ast` option throws; the returned `ast` is always `null`.
- `outputFilename` option throws.
- `modernAst: true` is accepted but ignored (emits `unsupported_option_ignored` warning).
- `dev: true` runtime checks land case-by-case — not all ownership / hydration diagnostics are emitted yet.

</details>

## Benchmarks

Whole-corpus throughput: **~2,950 real `.svelte` files**, each compiled in every mode, single-threaded, one file at a time. Median of 5 runs, measured 2026-07-19 via `just bench-compare` on Apple Silicon (macOS arm64), against **svelte@5.56.4** and **rsvelte 0.2.6**.

| mode | svelte | rsvelte | ours | vs svelte | vs rsvelte |
| --- | ---: | ---: | ---: | ---: | ---: |
| client | 1416.8 ms | 1387.4 ms | 109.1 ms | **13.0×** | **12.7×** |
| client-dev | 1456.6 ms | 1688.7 ms | 115.9 ms | **12.6×** | **14.6×** |
| ssr | 1254.1 ms | 1137.2 ms | 88.8 ms | **14.1×** | **12.8×** |
| ssr-dev | 1334.7 ms | 1225.5 ms | 93.7 ms | **14.2×** | **13.1×** |

Reproduce with `just bench-compare`. Per-run instruction-count benchmarks (64 total) also run on every commit via [CodSpeed](https://codspeed.io/MrWaip/svelte-rs-2). The [rsvelte](#alternatives) column appears when its native binding is installed. These are rsvelte's serial `compile` numbers, matching our single-file harness; its parallel `compileBatch` (rayon) reaches ~3× over `svelte/compiler` on this machine, yet our single-threaded compiler is still faster than rsvelte's 12-core batch. rsvelte's serial path is also markedly slower on arm64 than on x64 — on x64 it lands closer to ~2× over `svelte/compiler`.

Our lead widens as components grow; rsvelte's serial edge over `svelte/compiler` is ~2× only on sub-1 KB fixtures and collapses on real components — which dominate the byte weight:

| component size | share of bytes | ours vs svelte | rsvelte vs svelte |
| --- | ---: | ---: | ---: |
| < 1 KB | 26% | 8.6× | 2.3× |
| 1–10 KB | 17% | 14.2× | 0.7× |
| > 10 KB | 57% | 22.0× | 1.2× |

## Alternatives

- [**rsvelte**](https://github.com/baseballyama/rsvelte) — another Rust port of the Svelte 5 compiler, also built on OXC. Ships a WASM build (`@rsvelte/compiler`), a native NAPI binding, and a multi-threaded `compileBatch` API.

## Try it locally

Requires Rust, Node, and [`just`](https://github.com/casey/just) (`cargo install just` or `brew install just`).

```sh
just playground               # build wasm + serve playground
just quick-check App.svelte   # diff one component against svelte/compiler
just test-compiler            # run the 10,000+ comparison e2e suite (all modes)
just test-diagnostics         # run the 731-case diagnostics parity suite
```

## Contributing

Standard PRs welcome — no AI required. Day-to-day work uses [Claude Code](https://claude.com/claude-code) with repo-specific skills under `.claude/skills/`. Before opening a PR: `just test-compiler && just test-diagnostics && just lint` must be green.

Bugs and questions: <https://github.com/MrWaip/svelte-rs-2/issues>.

## Acknowledgements

- [Svelte](https://github.com/sveltejs/svelte) — the compiler this project mirrors; reference output is the source of truth.
- [OXC](https://github.com/oxc-project/oxc) — JS parser, AST, and codegen.
- [vite-plugin-svelte](https://github.com/sveltejs/vite-plugin-svelte) — base of the Vite integration fork.

## License

[MIT](./LICENSE) © Lobkov Constantine
