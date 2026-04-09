# N-API Canary Publish Plan

## Goal

Publish an npm package with a Svelte-compatible compiler surface:

- package name: `@mrwaip/svelte-rs2`
- public import path: `@mrwaip/svelte-rs2/compiler`
- public functions: `compile(source, options)` and `compileModule(source, options)`
- implementation backend: Rust via N-API, not WASM

The first release target is a `canary` channel, not a production-stable release.

## Non-goals for the first canary

- Exact package name parity with upstream `svelte`
- Full behavioral parity with the reference compiler across all unsupported features
- Windows support
- `parse`, `parseCss`, `preprocess`, `migrate`, or other `svelte/compiler` exports beyond `compile` and `compileModule`
- Perfect source map parity on day one

## Key requirement

External API shape should match Svelte as closely as practical for `compile` and `compileModule`, even if some internal features are still incomplete.

This means the adapter layer should aim to return the same structural result shape as upstream:

- `js: { code, map }`
- `css: null | { code, map, hasGlobal }`
- `warnings: Warning[]`
- `metadata`
- `ast` when supported

The Rust core currently returns a thinner shape, so this requires an explicit JS/native adapter layer instead of exposing the Rust structs directly.

## Proposed package layout

### Main package

- `@mrwaip/svelte-rs2`

Responsibilities:

- expose `./compiler` export
- provide JS loader/facade
- provide TypeScript declarations
- choose and load the correct native addon for the current platform

Suggested structure:

- `package.json`
- `compiler/index.js`
- `compiler/index.d.ts`
- `scripts/` for publish/build helpers if needed

### Platform packages

Suggested initial matrix:

- `@mrwaip/svelte-rs2-darwin-arm64`
- `@mrwaip/svelte-rs2-darwin-x64`
- `@mrwaip/svelte-rs2-linux-x64-gnu`

Optional next target:

- `@mrwaip/svelte-rs2-linux-arm64-gnu`

Each platform package should contain one native `.node` artifact plus a tiny package manifest.

## Recommended technical approach

Use the standard native npm pattern used by tools like esbuild:

- one small JS meta-package
- multiple platform-specific packages
- runtime platform detection in JS
- `optionalDependencies` from the main package to the platform packages

Do not start with a single npm package containing every binary unless there is a strong reason to accept much larger package size.

## Implementation phases

## Phase 1: define the compatibility contract

Create a short internal contract doc or test fixture set that freezes the intended JS API for:

- `compile(source, options)`
- `compileModule(source, options)`

Decide explicitly:

- which upstream options are accepted in canary
- which unsupported options throw
- which unsupported options are ignored with a warning
- whether `ast` is returned in the first canary
- whether maps are returned as `null`, skeletal objects, or real maps

Exit criteria:

- one written compatibility matrix
- one smoke test per function that validates the public JS shape

## Phase 2: add a dedicated N-API crate

Create a new crate for the Node binding instead of reusing `crates/wasm_compiler`.

Suggested name:

- `crates/napi_compiler`

Responsibilities:

- expose N-API functions callable from Node
- deserialize JS options
- call `svelte_compiler::compile` / `svelte_compiler::compile_module`
- convert the result into a Node-facing intermediate shape suitable for the JS facade

Keep the Rust side focused on transport and conversion, not npm packaging concerns.

Exit criteria:

- local Node process can call the addon directly
- `compile` and `compileModule` return structured data without crashing

## Phase 3: build the JS facade

Implement `@mrwaip/svelte-rs2/compiler` as a JS adapter over the native binding.

Responsibilities:

- load the correct platform package
- export `compile` and `compileModule`
- normalize options into the Rust/native layer format
- normalize native results into Svelte-compatible result objects

This layer should own compatibility behavior that is easier to express in JS, including:

- runtime shape normalization
- clear unsupported-option errors
- future compatibility shims

Exit criteria:

- consumer code can import from `@mrwaip/svelte-rs2/compiler`
- return shape matches the documented contract

## Phase 4: type surface

Add `compiler/index.d.ts`.

Start by copying only the relevant `compile` / `compileModule` declarations from the reference compiler and then reduce or annotate unsupported pieces only if necessary.

Do not start with hand-wavy custom typings if the goal is Svelte API compatibility.

Exit criteria:

- TypeScript consumer can import the package
- `compile` and `compileModule` signatures are stable and intentional

## Phase 5: platform packaging

Add packaging for the initial target matrix:

- macOS arm64
- macOS x64
- Linux x64 glibc

Main package should use `optionalDependencies` for these platform packages.

Each platform package should export the `.node` file in the simplest possible way.

Exit criteria:

- install succeeds on supported targets
- runtime loader resolves the native addon without manual setup

## Phase 6: CI and canary publish

Add GitHub Actions workflow that:

- builds each target in a matrix
- packages each platform package
- publishes or dry-runs platform packages first
- publishes the main package last

Use a `canary` dist-tag.

Suggested release trigger:

- manual workflow dispatch first
- then optional push/tag automation after the path is stable

Exit criteria:

- one end-to-end dry run succeeds
- one real `canary` publish succeeds

## Phase 7: compatibility smoke tests

Add a small JS test suite that runs against the published surface, not just the Rust crate.

Minimum tests:

- `compile` returns object with `js`, `css`, `warnings`, `metadata`
- `compileModule` returns object with `js`, `css`, `warnings`, `metadata`
- unsupported platform gets a clear error
- unsupported option behavior is explicit and stable
- supported package import path works in Node

Prefer black-box package tests over only Rust-level tests here.

## Phase 8: integrate the package into the `vite-plugin-svelte` fork

After the package exists and local/package-level smoke tests pass, the next validation stage is to wire it into the fork at:

- `https://github.com/MrWaip/vite-plugin-svelte`

The purpose of this phase is not just to prove that `compile` works in isolation, but to prove that real bundler integration can use the package as a drop-in compiler provider.

### Known integration points in the fork

At the time of writing, the fork uses `svelte/compiler` directly in at least these places:

- `packages/vite-plugin-svelte/src/utils/compile.js`
- `packages/vite-plugin-svelte/src/plugins/compile-module.js`
- `packages/vite-plugin-svelte/src/public.d.ts`

Observed usage patterns:

- runtime import of `* as svelte from 'svelte/compiler'`
- runtime import of `* as svelteCompiler from 'svelte/compiler'`
- type imports for `CompileOptions`, `CompileResult`, `Warning`, and `ModuleCompileOptions` from `svelte/compiler`
- plugin logic expects `compiled.js`, `compiled.css`, `compiled.warnings`, and `compileModule(...).js`

This means package adoption in the fork has two separate dimensions:

- runtime compatibility
- type-level compatibility

### Integration strategy

Do this in two steps, not one.

#### Step A: runtime substitution

Modify the fork so that compile-time runtime imports can come from `@mrwaip/svelte-rs2/compiler` instead of `svelte/compiler`.

Possible approaches:

- direct source edit in the fork to replace runtime imports
- temporary shim module inside the fork that re-exports `compile` / `compileModule`
- package-manager alias only if the runtime and type surfaces are both already compatible

For the first integration pass, prefer explicit source edits or a shim module over alias magic. It makes failures easier to localize.

#### Step B: type substitution

Once runtime integration works, update the fork's type imports to consume the new package's declarations or a fork-local compatibility shim.

This is likely to require one of these options:

- `import type { ... } from '@mrwaip/svelte-rs2/compiler'`
- a local `compiler-compat.d.ts` file that re-exports the package types
- temporary split mode where runtime comes from `@mrwaip/svelte-rs2/compiler` but some types still come from `svelte/compiler`

The goal is to end up with the fork depending on the new package intentionally, not accidentally compiling because `svelte` is still present in dev dependencies.

### Additional work likely needed in the fork

1. Dependency wiring

- add `@mrwaip/svelte-rs2` to the fork's dependencies or devDependencies
- decide whether `svelte` remains only as a peer dependency for application compatibility
- avoid relying on the upstream `svelte/compiler` implementation during tests by mistake

2. Compiler abstraction point

- if imports are scattered, add a single local module in the fork that owns compiler access
- route all `compile` / `compileModule` calls through that module
- keep future switching between upstream and Rust compiler cheap

3. Source map expectations

- the fork calls sourcemap normalization on both `compiled.js.map` and `compiled.css.map`
- if the package returns limited maps, the fork must tolerate that without crashing

4. Error and warning expectations

- the fork converts thrown compiler errors to Rollup/Vite errors
- warnings are logged separately
- result/error behavior must be close enough to upstream that plugin control flow does not break

5. HMR and CSS expectations

- the fork appends CSS imports based on `compiled.css.code`
- HMR logic patches `compiled.js.code`
- the result object must support these mutations in the same shape as upstream

### Tests to run in the fork after integration

Minimum validation set:

- one dev compile of a simple `.svelte` component
- one build compile of a simple `.svelte` component
- one SSR compile path
- one `.svelte.js` or `.svelte.ts` `compileModule` path
- one component with emitted CSS
- one warning-producing component

Prefer using the fork's own existing unit/e2e test commands instead of only ad hoc local scripts.

### Acceptance bar for the fork integration phase

Consider the fork integration successful only when all of the following are true:

- the fork builds and runs using `@mrwaip/svelte-rs2/compiler` at runtime
- `compile` path in the plugin works for dev and build
- `compileModule` path in the plugin works
- no code path silently falls back to upstream `svelte/compiler`
- TypeScript checks in the fork pass, or any temporary type gaps are documented explicitly
- the fork's test suite has at least one passing end-to-end path powered by the new package

## Open decisions to resolve early

1. Result fidelity for maps

- Temporary acceptable canary choice: return `null` maps or clearly limited maps
- Better long-term choice: match upstream map objects

2. Warning/error model

- Decide whether Rust diagnostics become `warnings` only, or are split into warnings vs thrown compile errors
- Upstream compatibility matters here more than internal convenience

3. AST support

- Returning `ast` in canary is useful for API parity
- If not supported initially, decide whether to omit it, set it to `null`, or throw on `modernAst`

4. Unsupported options

- Do not silently accept options that materially change output
- Throw early for unsupported options such as features not yet implemented

5. Package manager edge cases

- npm optional dependency installs
- Docker and copied `node_modules`
- unsupported architecture error messaging

6. `vite-plugin-svelte` adoption mode

- decide whether the fork should import `@mrwaip/svelte-rs2/compiler` directly
- or via a local compatibility shim module
- do not leave long-term compiler selection scattered across multiple plugin files

## Suggested acceptance bar for first canary

Ship the first canary only when all of the following are true:

- `@mrwaip/svelte-rs2/compiler` imports successfully on supported targets
- `compile` works for a basic `.svelte` component
- `compileModule` works for a basic `.svelte.js` input
- package-level smoke tests pass on every supported target
- unsupported target failure message is clear
- public typings exist
- CI can publish repeatably
- a real `vite-plugin-svelte` integration attempt has been completed, even if some gaps remain documented

## Suggested file additions

Likely new files/directories:

- `crates/napi_compiler/`
- `npm/` or `packages/` workspace for npm artifacts
- `packages/svelte-rs2/package.json`
- `packages/svelte-rs2/compiler/index.js`
- `packages/svelte-rs2/compiler/index.d.ts`
- `packages/svelte-rs2-darwin-arm64/package.json`
- `packages/svelte-rs2-darwin-x64/package.json`
- `packages/svelte-rs2-linux-x64-gnu/package.json`
- `.github/workflows/canary-publish.yml`

Exact layout can be adjusted, but keeping npm packaging isolated from Rust crates will reduce confusion.

## Practical first slice

If this work is split across multiple sessions, the best first bounded slice is:

1. Add the N-API crate
2. Expose direct native `compile` / `compileModule`
3. Add a JS facade package at `@mrwaip/svelte-rs2/compiler`
4. Make it work locally on one platform
5. Freeze the result shape with JS tests

Only after that should platform packaging and publish automation be added.

The next bounded slice after the first canary should be:

1. Patch the `vite-plugin-svelte` fork to use the new runtime compiler import
2. Run the fork's unit/build/dev tests
3. Fix result-shape mismatches found by the plugin
4. Move type imports off upstream `svelte/compiler`
5. Freeze the integration path in documentation

## Notes for the implementing agent

- Do not route Node/npm concerns through `crates/wasm_compiler`
- Keep the public JS API intentionally compatible with Svelte, even if the Rust transport format differs
- Prefer explicit unsupported-option errors over silent partial behavior
- Keep the first target matrix small and honest
- Treat package-level tests as required, not optional
- Treat `vite-plugin-svelte` as the first real consumer and use it to drive adapter correctness
