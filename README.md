<div align="center">

# svelte-rs

**The Svelte 5 compiler, rewritten in Rust.**

Drop-in replacement for `svelte/compiler` (pinned to **svelte@5.56.4**) — same JS output, ~13× faster across the full corpus (client + server, dev + prod).

> ⚠️ **WIP / canary.** Built by a human with heavy AI assistance. Expect bugs, missing edge cases, and breaking changes. Not production-ready — please report what breaks.

[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/MrWaip/svelte-rs-2)
[![npm](https://img.shields.io/badge/npm-%40mrwaip%2Fsvelte--rs2-cb3837)](https://www.npmjs.com/package/@mrwaip/svelte-rs2)
[![vite-plugin](https://img.shields.io/npm/v/@mrwaip/vite-plugin-svelte?label=vite-plugin-svelte&color=646cff)](https://www.npmjs.com/package/@mrwaip/vite-plugin-svelte)
[![tests](https://img.shields.io/badge/e2e_tests-10k%2B-success)](./tasks/compiler_tests/cases2)

[Playground](https://mrwaip.github.io/svelte-rs-2/) · [Roadmap](./ROADMAP.md)

</div>

---

## Why

- **Byte-exact JS output** — **~1,200 cases** (1,199 compiler + 26 diagnostics) expand to **10,000+ passing e2e tests**, each diffed against the reference compiler across client + server × dev + prod.
- **Drop-in** — same `compile()` / `compileModule()` API as `svelte/compiler`.
- **~13× faster** across the full 2,825-file corpus (client 12.9×, server 13.5×, dev variants ~12.7×).
- Wired into a fork of `vite-plugin-svelte`, so you can try it in a real Vite app today.

### Requirements

| | |
| --- | --- |
| Node | `^20.19 \|\| ^22.12 \|\| >=24` |
| Platforms | macOS arm64/x64, Linux x64 glibc. Windows and Linux musl (Alpine) not yet. |
| Peer | `svelte@5.56.4` — Vite plugin falls back to `svelte/compiler` for unsupported options. |

## What Works So Far?

This is still a work in progress and is not yet at full feature parity with `svelte/compiler`. Parity target: **svelte@5.56.4**. Bugs may exist. Please check this list carefully before logging a new issue or assuming an intentional change.

| Feature | Status | Notes |
| --- | --- | --- |
| Svelte 5 syntax | done | Runes, template, bindings, directives, events, special elements, diagnostics, a11y. Byte-exact diff against `svelte/compiler`. |
| Svelte 4 legacy | done | `export let`, `$:`, `beforeUpdate`/`afterUpdate`, `<slot>`, `<svelte:self>`, `<svelte:component>`, auto-mode detection. Byte-exact diff against `svelte/compiler`. |
| CSS pipeline | done | analyze + transform + codegen |
| TypeScript | done | Script stripping only — no type checking. |
| `.svelte.js` / `.svelte.ts` modules | done | — |
| Custom elements | in progress | Basic path works; some option combinations not covered. |
| Compiler options | in progress | Most common options honored (incl. `discloseVersion`, `warningFilter`); long tail still landing. |
| Dev mode (`dev: true`) | done | Byte-exact diff across ~1,200 client-dev cases; server-dev too. Some ownership/hydration diagnostics still land case-by-case. |
| SSR (`generate: 'server'`) | done | Server transform + codegen (prod & dev). Byte-exact diff across ~1,200 server cases. |
| Source maps | done | `js.map` / `css.map` emitted; `sourcemap` option honored. |
| HMR | done | Hot-module-replacement output supported. |
| Preprocessors | not ready | Not started. |

Definitions:

- **done** aka "believed done": we're not currently aware of any deficits or major left work to do. OK to log bugs.
- **in progress**: currently being worked on; some things may work, some may not. OK to log panics, but nothing else please.
- **prototype**: proof-of-concept only; do not log bugs.
- **not ready**: either haven't even started yet, or far enough from ready that you shouldn't bother messing with it yet.

Full breakdown: [ROADMAP.md](./ROADMAP.md).

### Known limitations

- `ast` option is not supported — passing it throws; the returned `ast` is always `null`.
- `outputFilename` option is not supported — passing it throws.
- `modernAst: true` is accepted but ignored (emits `unsupported_option_ignored` warning).
- `dev: true` runtime checks land case-by-case — not all ownership / hydration diagnostics are emitted yet.

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

API mirrors `svelte/compiler`. Unsupported options (`ast`, `outputFilename`) throw; see `packages/svelte-rs2/compiler/index.d.ts`.

Beyond the reference API, a few opt-in extras are available:

- **`warningFilter: (warning) => boolean`** — matches Svelte 5's option; drops warnings the predicate rejects.
- **`suppress: WarningCode[]`** — a typed list of warning codes dropped at the source (the `WarningCode` union covers every emittable warning). Cheaper than filtering after the fact: suppressed warnings are never built, framed, or serialized.
- **`withDiagnostics: true`** — returns `{ js, css, diagnostics }` where each diagnostic carries `severity: 'error' | 'warning'`, and **never throws** on error (instead of the default throw-first-error behavior). Handy for editors, linters, and batch tooling that want every error in one pass.

Want to see real input → output? Browse [`tasks/compiler_tests/cases2/`](./tasks/compiler_tests/cases2) — each folder has `case.svelte` (input) plus our output and the reference for every mode: `case-rust.js` / `case-svelte.js` (client), `.dev.js`, `.server.js`, `.server.dev.js`. ~1,200 cases, byte-diffed.

### In a Vite app

Use the fork of `vite-plugin-svelte` — it routes `compile` / `compileModule` through `@mrwaip/svelte-rs2` automatically, with fallback to `svelte/compiler` where the Rust side doesn't support an option yet.

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

## Benchmarks

<img src="./assets/benchmark.svg" alt="svelte-rs compiler benchmark — ~13× faster than svelte/compiler, ~6× faster than rsvelte" width="720">

Whole-corpus throughput: **2,825 real `.svelte`/`.svelte.js` files**, each compiled in every mode, single-threaded, one file at a time. Median of 5 runs, measured 2026-07-18 via `just bench-compare` on a 12-core Linux x64. Per-run instruction-count benchmarks (64 total) also run on every commit via [CodSpeed](https://codspeed.io/MrWaip/svelte-rs-2).

| mode | svelte | rsvelte | ours | vs svelte | vs rsvelte |
| --- | ---: | ---: | ---: | ---: | ---: |
| client | 1403.8 ms | 676.6 ms | 110.9 ms | **12.7×** | **6.1×** |
| client-dev | 1490.9 ms | 913.1 ms | 127.1 ms | **11.7×** | **7.2×** |
| ssr | 1222.6 ms | 533.0 ms | 89.9 ms | **13.6×** | **5.9×** |
| ssr-dev | 1321.6 ms | 667.2 ms | 101.7 ms | **13.0×** | **6.6×** |

Reproduce it with `just bench-compare`. The [rsvelte](#alternatives) column appears when the optional `@rsvelte/vite-plugin-svelte-native` is installed; single-threaded and per-file, rsvelte lands around ~2× vs `svelte/compiler` (it also ships a multi-threaded `compileBatch` API — a different axis). Speedup climbs further on large single components, where fixed per-file overhead amortizes.

## Alternatives

- [**rsvelte**](https://github.com/baseballyama/rsvelte) — another Rust port of the Svelte 5 compiler, also built on OXC. Ships a WASM build (`@rsvelte/compiler`), a native NAPI binding, and a multi-threaded `compileBatch` API. `just bench-compare` compares against it directly whenever its native binding is installed.

## Try it locally

Requires Rust, Node, and [`just`](https://github.com/casey/just) (`cargo install just` or `brew install just`).

```sh
just playground               # build wasm + serve playground
just quick-check App.svelte   # diff one component against svelte/compiler
just test-compiler            # run the 10,000+ test e2e suite (all modes)
```

## Contributing

Standard PRs welcome — no AI required. Day-to-day work uses [Claude Code](https://claude.com/claude-code); slash commands live in `.claude/commands/` (`/status`, `/port`, `/audit`). Before opening a PR: `just test-compiler && just test-diagnostics && just lint` must be green.

Bugs and questions: <https://github.com/MrWaip/svelte-rs-2/issues>.

## Acknowledgements

- [Svelte](https://github.com/sveltejs/svelte) — the compiler this project mirrors; reference output is the source of truth.
- [OXC](https://github.com/oxc-project/oxc) — JS parser, AST, and codegen.
- [vite-plugin-svelte](https://github.com/sveltejs/vite-plugin-svelte) — base of the Vite integration fork.

## License

[MIT](./LICENSE) © Lobkov Constantine
