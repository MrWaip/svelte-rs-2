<div align="center">

# svelte-rs

**The Svelte 5 compiler, rewritten in Rust.**

Drop-in replacement for `svelte/compiler` (pinned to **svelte@5.56.4**) — same JS output, ~9× faster.

[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/MrWaip/svelte-rs)
[![npm](https://img.shields.io/badge/npm-%40mrwaip%2Fsvelte--rs2-cb3837)](https://www.npmjs.com/package/@mrwaip/svelte-rs)
[![vite-plugin](https://img.shields.io/npm/v/@mrwaip/vite-plugin-svelte?label=vite-plugin-svelte&color=646cff)](https://www.npmjs.com/package/@mrwaip/vite-plugin-svelte)
[![tests](https://img.shields.io/badge/e2e_tests-10k%2B-success)](./tasks/compiler_tests/cluster_cases)

[Playground](https://mrwaip.github.io/svelte-rs/) · [Issues](https://github.com/MrWaip/svelte-rs/issues)

> ⚠️ **WIP / canary.** Built by a human with heavy AI assistance. Expect bugs, missing edge cases, and breaking changes. Not production-ready — please report what breaks.

</div>

---

## Why

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./assets/benchmark.svg">
  <source media="(prefers-color-scheme: light)" srcset="./assets/benchmark-light.svg">
  <img src="./assets/benchmark.svg" alt="svelte-rs compiler benchmark — ~9× faster than svelte/compiler, ~3.7× faster than rsvelte" width="720">
</picture>

- **Byte-exact JS output** — **~2,600 cases** (1,404 cluster + 1,199 legacy), each diffed against the reference compiler in all four modes (client/server × dev/prod) — **10,000+ passing comparisons**.
- **Diagnostics parity** — **731 cases** across 26 categories (a11y, runes, CSS, TypeScript, …), matched against `svelte/compiler`'s own warnings and errors by code, severity, and span.
- **Drop-in** — same `compile()` / `compileModule()` API as `svelte/compiler`.
- **~9× faster** on two anonymized real-world production codebases (8.6–10.2× across every mode, 3.7–4.9× over rsvelte's native binding).
- **Ready to try** — wired into a fork of `vite-plugin-svelte`, so it runs in a real Vite app today.

## Install

### As a compiler

```sh
npm i -D @mrwaip/svelte-rs
```

```js
import { compile } from '@mrwaip/svelte-rs/compiler';

const { js } = compile(
  `<script>let { name } = $props();</script><h1>hello {name}</h1>`,
  { filename: 'Hello.svelte', generate: 'client' }
);

console.log(js.code);
```

The API mirrors `svelte/compiler`; see `packages/svelte-rs/compiler/index.d.ts`. A few opt-in extras beyond the reference API:

- **`warningFilter: (warning) => boolean`** — matches Svelte 5's option; drops warnings the predicate rejects.
- **`suppress: WarningCode[]`** — a typed list of warning codes dropped at the source. Cheaper than filtering after the fact: suppressed warnings are never built, framed, or serialized.
- **`withDiagnostics: true`** — returns `{ js, css, diagnostics }` where each diagnostic carries `severity: 'error' | 'warning'`, and never throws on error. Handy for editors, linters, and batch tooling.

Real input → output for every mode lives in [`tasks/compiler_tests/cluster_cases/`](./tasks/compiler_tests/cluster_cases): each leaf has `case.svelte` plus our output and the reference (`case-rust.js` / `case-svelte.js`, `.dev.js`, `.server.js`, `.server.dev.js`). The older flat [`cases2/`](./tasks/compiler_tests/cases2) suite is legacy but still checked.

### In a Vite app

The fork of `vite-plugin-svelte` routes `compile` / `compileModule` through `@mrwaip/svelte-rs` automatically, falling back to `svelte/compiler` for options the Rust side doesn't support yet.

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

Whole-corpus throughput on **two anonymized real-world production codebases** — every `.svelte` file compiled in every mode, single-threaded, one file at a time. Median of 5 runs, measured 2026-07-27 via `just bench-compare <dir>` on Apple Silicon (macOS arm64), against **svelte@5.56.4**, **@rsvelte/vite-plugin-svelte-native 0.3.1** (native binding) and **@rsvelte/compiler 0.9.4** (WASM).

| project | mode | svelte | ours | rsv-native | rsv-wasm | vs svelte | vs rsv-native | vs rsv-wasm |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| **A** — 7,910 files, 12.0 MB | client | 1995.2 ms | 221.7 ms | 820.2 ms | 1334.3 ms | **9.00×** | **3.70×** | **6.02×** |
| | client-dev | 2039.3 ms | 237.2 ms | 877.7 ms | n/a | **8.60×** | **3.70×** | n/a |
| | ssr | 1741.1 ms | 182.9 ms | 683.8 ms | 1020.2 ms | **9.52×** | **3.74×** | **5.58×** |
| | ssr-dev | 1914.8 ms | 201.2 ms | 749.7 ms | n/a | **9.51×** | **3.73×** | n/a |
| **B** — 10,093 files, 14.4 MB | client | 2831.3 ms | 313.3 ms | 1507.7 ms | 2395.2 ms | **9.04×** | **4.81×** | **7.64×** |
| | client-dev | 3050.4 ms | 335.8 ms | 1628.4 ms | n/a | **9.08×** | **4.85×** | n/a |
| | ssr | 2677.5 ms | 263.3 ms | 1068.4 ms | 1603.5 ms | **10.17×** | **4.06×** | **6.09×** |
| | ssr-dev | 2810.1 ms | 283.5 ms | 1144.4 ms | n/a | **9.91×** | **4.04×** | n/a |

Both projects are closed-source, so these runs are not reproducible from this repository — `just bench-compare` against the repo's own fixture corpus is. An explicit directory is scanned as-is, so the file counts include whatever `.svelte` files the checkout carries, vendored dependencies included.

On project A all four compilers compiled the identical file set (zero skips). On project B `rsvelte` rejects one file the other three accept (`svelte 24 / ours 24 / rsvelte 25` skipped), so its columns there are measured on a set smaller by one file.

Reproduce with `just bench-compare`. Per-run instruction-count benchmarks (64 total) also run on every commit via [CodSpeed](https://codspeed.io/MrWaip/svelte-rs). The [rsvelte](#alternatives) columns appear when its packages are installed. These are rsvelte's serial `compile` numbers, matching our single-file harness — its parallel `compileBatch` (rayon) is not measured here.

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

Bugs and questions: <https://github.com/MrWaip/svelte-rs/issues>.

## Acknowledgements

- [Svelte](https://github.com/sveltejs/svelte) — the compiler this project mirrors; reference output is the source of truth.
- [OXC](https://github.com/oxc-project/oxc) — JS parser, AST, and codegen.
- [vite-plugin-svelte](https://github.com/sveltejs/vite-plugin-svelte) — base of the Vite integration fork.

## License

[MIT](./LICENSE) © Lobkov Constantine
