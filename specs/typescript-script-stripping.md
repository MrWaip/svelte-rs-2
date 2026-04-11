# TypeScript Script Stripping

## Current state
- **Working**: 6/7 known use cases
- **Next**: comment-only `<script lang="ts">` with no surviving JS statements must not leak comments into client output or perturb template cursor state
- Last updated: 2026-04-11

## Source

- Mapped from [specs/unknown.md](/Users/klobkov/personal-code/svelte-rs-2/specs/unknown.md:1) after `diagnose_svg_city_icon` isolated a TypeScript-script stripping gap without an owning feature spec
- User request: create a durable owning spec before porting the next slice

## Syntax variants

- `<script lang="ts">let value: string = 'x';</script>`
- `<script lang="ts">import type { Foo } from 'pkg';</script>`
- `<script lang="ts">{expr as T}; expr satisfies T; expr!;</script>`
- `<div value={foo as string} />`
- `{#const value = expr as T}`
- `<script lang="ts">// comment only</script>`

## Use cases

- [x] TypeScript wrappers inside template expression tags are stripped before client output matches reference (test: `ts_strip_expression_tag`)
- [x] TypeScript `satisfies` expressions are stripped in emitted client output (test: `ts_strip_satisfies`)
- [x] TypeScript non-null assertions are stripped in emitted client output (test: `ts_strip_non_null`)
- [x] TypeScript wrappers inside `{#const ...}` initializers are stripped in emitted client output (test: `ts_strip_const_tag`)
- [x] TypeScript wrappers inside regular dynamic attributes are stripped in emitted client output (test: `ts_strip_attribute`)
- [x] Instance `<script lang="ts">` with surviving runtime JavaScript strips type syntax while preserving the remaining script logic (test: `ts_strip_script_types`)
- [ ] Comment-only or otherwise effectively-empty `<script lang="ts">` blocks must not preserve orphaned script comments in final client JS, and must not introduce extra template cursor operations such as `$.next(...)` or `$.reset(...)` after the script disappears (test: `diagnose_svg_city_icon`, moderate)

## Out of scope

- SSR output for TypeScript-bearing components
- Diagnostic policy for unsupported TypeScript syntax outside `<script>` tags
- Module-script-specific parity beyond cases proven by a focused repro

## Reference

- Reference compiler:
- `reference/compiler/index.js`
- `reference/compiler/phases/1-parse/remove_typescript_nodes.js`
- `reference/compiler/phases/3-transform/index.js`
- Rust implementation:
- `crates/svelte_parser/src/parse_js.rs`
- `crates/svelte_codegen_client/src/script/pipeline.rs`
- `crates/svelte_codegen_client/src/lib.rs`
- `tasks/compiler_tests/test_v3.rs`

## Test cases

- [x] `ts_strip_expression_tag`
- [x] `ts_strip_satisfies`
- [x] `ts_strip_non_null`
- [x] `ts_strip_const_tag`
- [x] `ts_strip_attribute`
- [x] `ts_strip_script_types`
- [ ] `diagnose_svg_city_icon`
