# @mrwaip/svelte-rs

**The Svelte 5 compiler, rewritten in Rust.**

Drop-in replacement for `svelte/compiler` (parity target: **svelte@5.56.4**) — same JS output, ~9× faster.

[![npm](https://img.shields.io/npm/v/@mrwaip/svelte-rs?color=cb3837)](https://www.npmjs.com/package/@mrwaip/svelte-rs)
[![license](https://img.shields.io/npm/l/@mrwaip/svelte-rs)](https://github.com/MrWaip/svelte-rs/blob/master/LICENSE)

[Playground](https://mrwaip.github.io/svelte-rs/) · [Repository](https://github.com/MrWaip/svelte-rs) · [Issues](https://github.com/MrWaip/svelte-rs/issues)

> ⚠️ **WIP / canary.** Expect bugs, missing edge cases, and breaking changes. Not production-ready — please report what breaks.

## Install

```sh
npm i -D @mrwaip/svelte-rs
```

A Node native addon (NAPI). Prebuilt binaries come in as optional dependencies — nothing is compiled on install.

## Usage

```js
import { compile } from '@mrwaip/svelte-rs/compiler';

const { js, css, warnings } = compile(
  `<script>let { name } = $props();</script><h1>hello {name}</h1>`,
  { filename: 'Hello.svelte', generate: 'client' }
);

console.log(js.code);
```

`compileModule` handles `.svelte.js` / `.svelte.ts` modules, `preprocess` runs markup / script / style hooks:

```js
import { compileModule, preprocess } from '@mrwaip/svelte-rs/compiler';

const mod = compileModule(`let count = $state(0);`, { filename: 'store.svelte.js' });
const { code } = await preprocess(source, [myPreprocessor], { filename: 'App.svelte' });
```

The API mirrors `svelte/compiler`; full typings live in `compiler/index.d.ts`.

### In a Vite app

A fork of `vite-plugin-svelte` routes `compile` / `compileModule` through this package, falling back to `svelte/compiler` for options the Rust side doesn't support yet.

```sh
npm i -D @mrwaip/vite-plugin-svelte
```

```js
import { defineConfig } from 'vite';
import { svelte } from '@mrwaip/vite-plugin-svelte';

export default defineConfig({ plugins: [svelte()] });
```

## Beyond the reference API

Opt-in extras that `svelte/compiler` does not have:

- **`warningFilter: (warning) => boolean`** — matches Svelte 5's option; drops warnings the predicate rejects.
- **`suppress: WarningCode[]`** — a typed list of warning codes dropped at the source. Cheaper than filtering afterwards: suppressed warnings are never built, framed, or serialized.
- **`withDiagnostics: true`** — returns `{ js, css, diagnostics }` where each diagnostic carries `severity: 'error' | 'warning'`, and never throws on error. Handy for editors, linters, and batch tooling.
- **`transformTypescript: true`** — transpiles TypeScript features that emit runtime code (`enum`, `namespace`, parameter properties, decorators) instead of reporting `typescript_invalid_feature`. Behaviourally equivalent to `vitePreprocess({ script: true })`, not byte-equal.
- **`transformStyle: true`** (+ **`loadPaths`**) — compiles `lang="scss"` / `lang="sass"` with `grass` before the CSS pipeline. Semantically equal to dart-sass on our corpus, but `grass` does not promise full dart-sass compatibility — keep a JS preprocessor fallback for unsupported constructs.
- **`cssTargets: string[]`** — browserslist queries; lowers modern CSS and adds vendor prefixes via `lightningcss`, replacing `postcss` + `autoprefixer`. Not byte-equal to autoprefixer: the prefix databases disagree.

These run inside the compiler with no Node round trip, and sit outside the byte-parity contract with `svelte/compiler`.

## Requirements

| | |
| --- | --- |
| Node | `^20.19 \|\| ^22.12 \|\| >=24` |
| Platforms | macOS (arm64/x64), Linux glibc & musl/Alpine (x64/arm64), Windows (x64/arm64) |
| Svelte | `svelte@5.56.4` — the parity target (not a declared peer dependency) |

Unsupported targets throw an explicit platform error on import.

## Result shape

Both `compile` and `compileModule` return:

- `js: null | { code, map }`
- `css: null | { code, map, hasGlobal }`
- `warnings: Warning[]` — or `diagnostics: Diagnostic[]` with `withDiagnostics: true`
- `metadata: { canary, hasCss, unsupported }`
- `ast: null`

Source maps are emitted for both JS and CSS; the `sourcemap` option (an input map to chain) is honored.

Rust diagnostics with severity `Error` are rethrown as JS exceptions unless `withDiagnostics: true` is set. Everything else comes back through `warnings`.

## Canary limitations

- `ast` and `outputFilename` options **throw**; the returned `ast` is always `null`.
- `modernAst: true` is accepted but ignored — emits an `unsupported_option_ignored` warning.
- `dev: true` runtime checks land case-by-case: not every ownership / hydration diagnostic is emitted yet.
- Custom elements and the long tail of compiler options are still in progress.
- `transformTypescript`, `transformStyle`, `loadPaths` and `cssTargets` work at runtime but are not yet declared in the `CompileOptions` typings.

The [feature matrix](https://github.com/MrWaip/svelte-rs#status) says which areas accept bug reports today.

## Why

- **Byte-exact JS output** — ~3,000 cases diffed against the reference compiler in all four modes (client/server × dev/prod): 12,000+ passing comparisons.
- **Diagnostics parity** — 823 cases across 27 categories (a11y, runes, CSS, TypeScript, …), matched by code, severity, and span.
- **~9× faster** on two real-world production codebases (8.6–10.2× across every mode). [Numbers and methodology](https://github.com/MrWaip/svelte-rs#benchmarks).

## License

[MIT](https://github.com/MrWaip/svelte-rs/blob/master/LICENSE) © Lobkov Constantine
