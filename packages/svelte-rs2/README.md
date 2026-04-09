# @mrwaip/svelte-rs2 (canary facade)

## Compiler entrypoint

Use `@mrwaip/svelte-rs2/compiler`.

```js
import { compile, compileModule } from '@mrwaip/svelte-rs2/compiler';
```

## Canary compatibility policy

This package currently exposes only `compile` and `compileModule` through a Node native addon.

### Native loading policy

- In local development, `compiler/index.js` first checks `compiler/native/svelte-rs2.node`.
- In packaged installs, it loads a platform package via optional dependencies:
  - `@mrwaip/svelte-rs2-darwin-arm64`
  - `@mrwaip/svelte-rs2-darwin-x64`
  - `@mrwaip/svelte-rs2-linux-x64-gnu`
- Unsupported targets throw an explicit platform error during import.
- For canary packaging, run `npm run prepare-platform-package` in `packages/svelte-rs2` after `cargo build -p napi_compiler --release` to copy the current platform artifact into the matching platform package.

### Result shape

Both `compile` and `compileModule` return a stable canary shape:

- `js: null | { code, map }`
- `css: null | { code, map, hasGlobal }`
- `warnings: Warning[]`
- `metadata: { canary, hasCss, unsupported }`
- `ast: null`

### Source map policy

- `js.map` and `css.map` are always `null` in the current canary.
- There is no source-map generation yet.

### AST policy

- `ast` is always `null` in the current canary.
- AST output is intentionally not exposed until the public shape is finalized.

### Unsupported options policy

- `ast`, `sourcemap`, `outputFilename` **throw** immediately.
- `modernAst` is accepted but produces a warning with code `unsupported_option_ignored`.

### Diagnostics policy

- Rust diagnostics with severity `Error` are rethrown as JS exceptions.
- Non-error diagnostics are returned through `warnings`.
