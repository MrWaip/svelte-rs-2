# @mrwaip/svelte-rs2 (canary facade)

## Compiler entrypoint

Use `@mrwaip/svelte-rs2/compiler`.

```js
import { compile, compileModule } from '@mrwaip/svelte-rs2/compiler';
```

## Canary compatibility policy

This package currently exposes only `compile` and `compileModule` through a Node native addon.

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
