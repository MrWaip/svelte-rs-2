<div align="center">

# svelte-rs

**The Svelte 5 compiler, rewritten in Rust.**

Drop-in replacement for `svelte/compiler` (pinned to **svelte@5.56.4**) — same JS output, ~30× faster.

> ⚠️ **WIP / canary.** Built by a human with heavy AI assistance. Expect bugs, missing edge cases, and breaking changes. Not production-ready — please report what breaks.

[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/MrWaip/svelte-rs-2)
[![npm](https://img.shields.io/badge/npm-%40mrwaip%2Fsvelte--rs2-cb3837)](https://www.npmjs.com/package/@mrwaip/svelte-rs2)
[![tests](https://img.shields.io/badge/e2e_tests-1500%2B-success)](./tasks/compiler_tests/cases2)

[Playground](https://mrwaip.github.io/svelte-rs-2/) · [Roadmap](./ROADMAP.md)

</div>

---

## Why

- **Byte-exact JS output** — diffed against the reference compiler on **1500+ end-to-end cases** (compiler + diagnostics).
- **Drop-in** — same `compile()` / `compileModule()` API as `svelte/compiler`.
- **30× faster** (geomean across 24 benches, up to **52×** on large components).
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
| Svelte 4 legacy | in progress | `export let`, `$:`, `beforeUpdate`/`afterUpdate`, `<svelte:component>`, auto-mode detection done. `<slot>` / `<svelte:self>` still pending. |
| CSS pipeline | done | analyze + transform + codegen |
| TypeScript | done | Script stripping only — no type checking. |
| `.svelte.js` / `.svelte.ts` modules | done | — |
| Custom elements | in progress | Basic path works; some option combinations not covered. |
| Compiler options | in progress | Most common options honored; long tail (`discloseVersion`, etc.) still landing. |
| Dev mode (`dev: true`) | in progress | Runtime checks and ownership tracking land case-by-case. |
| Source maps | in progress | Partial implementation, not yet verified against reference. |
| Preprocessors | not ready | Not started. |
| SSR (`generate: 'server'`) | not ready | Not started — large chunk of work. |
| HMR | not ready | Not started. |

Definitions:

- **done** aka "believed done": we're not currently aware of any deficits or major left work to do. OK to log bugs.
- **in progress**: currently being worked on; some things may work, some may not. OK to log panics, but nothing else please.
- **prototype**: proof-of-concept only; do not log bugs.
- **not ready**: either haven't even started yet, or far enough from ready that you shouldn't bother messing with it yet.

Full breakdown: [ROADMAP.md](./ROADMAP.md).

### Known limitations

- `ast` option is not supported — passing it throws; the returned `ast` is always `null`.
- `sourcemap` and `outputFilename` options are not supported — passing them throws.
- `modernAst: true` is accepted but ignored (emits `unsupported_option_ignored` warning).
- `js.map` / `css.map` are always `null` (source maps are WIP).
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

API mirrors `svelte/compiler`. Unsupported options (`ast`, `sourcemap`, `outputFilename`) throw; see `packages/svelte-rs2/compiler/index.d.ts`.

Want to see real input → output? Browse [`tasks/compiler_tests/cases2/`](./tasks/compiler_tests/cases2) — each folder has `case.svelte` (input), `case-rust.js` (our output) and `case-svelte.js` (reference). 1100+ cases, byte-diffed.

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

**Geomean: 30.18×** across 24 cases (min 20.14×, max 51.97×). Walltime vs `svelte/compiler`, measured 2026-05-04 via `just bench-compare-walltime`. `big_v6.svelte` is a synthetic stress component (~5.8k LOC); the rest are real-world Svelte files.

<details>
<summary>Per-case table</summary>

| case | rust med | js med | speedup |
| --- | ---: | ---: | ---: |
| `big_v6.svelte` (compile, synthetic stress) | 18.39 ms | 955.9 ms | **51.97×** |
| `big_v6.svelte` (compile_dev, synthetic stress) | 19.56 ms | 944.7 ms | **48.30×** |
| `module/case_02.svelte.js` (real-world) | 0.131 ms | 5.283 ms | 40.37× |
| `template/case_01.svelte` (real-world) | 0.745 ms | 21.65 ms | 29.08× |
| `snippets/case_01.svelte` (real-world) | 1.558 ms | 35.62 ms | 22.86× |
| `css/case_01.svelte` (real-world) | 0.014 ms | 0.290 ms | 20.14× |

</details>

## Try it locally

Requires Rust, Node, and [`just`](https://github.com/casey/just) (`cargo install just` or `brew install just`).

```sh
just playground               # build wasm + serve playground
just quick-check App.svelte   # diff one component against svelte/compiler
just test-compiler            # run the 1500+ e2e suite
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
