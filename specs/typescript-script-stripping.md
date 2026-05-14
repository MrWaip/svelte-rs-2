# TypeScript Script Stripping

## Current state
- **Working**: 16/16 use cases
- **Tests**: 16/16 green
- Last updated: 2026-05-22

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
- `<script>/** @type {Foo} */ let x;</script>` (JSDoc type annotations on plain JS scripts)

## Use cases

- [x] TypeScript wrappers inside template expression tags are stripped before client output matches reference (test: `ts_strip_expression_tag`)
- [x] TypeScript `satisfies` expressions are stripped in emitted client output (test: `ts_strip_satisfies`)
- [x] TypeScript non-null assertions are stripped in emitted client output (test: `ts_strip_non_null`)
- [x] TypeScript non-null assertion at the tail of an optional chain (`expr?.x!`) is stripped (test: `ts_strip_non_null_chain`)
- [x] TypeScript wrappers inside `{#const ...}` initializers are stripped in emitted client output (test: `ts_strip_const_tag`)
- [x] TypeScript wrappers inside regular dynamic attributes are stripped in emitted client output (test: `ts_strip_attribute`)
- [x] Instance `<script lang="ts">` with surviving runtime JavaScript strips type syntax while preserving the remaining script logic (test: `ts_strip_script_types`)
- [x] Comment-only or otherwise effectively-empty `<script lang="ts">` blocks must not preserve orphaned script comments in final client JS, and must not introduce extra template cursor operations such as `$.next(...)` or `$.reset(...)` after the script disappears (test: `diagnose_svg_city_icon`)
- [x] JSDoc `/** @type ... */` on rune-bound declarations (`$props`, `$derived`, `$derived.by`) must be preserved in client output (test: `script_jsdoc_comment`)
- [x] Type-only imports (`import type { X } from '...'`) must not contribute to top-level symbol collision when deriving the filename-based component name. Repro: filename `DepositMethod.svelte` plus `import type { DepositMethod } from './types'` previously bumped the emitted function to `DepositMethod_1`. Owning layer: analyze — `collect_component_top_level_symbol_names` now filters out symbols carrying `SymbolFlags::TypeImport`. (test: `diagnose_filename_name_collides_with_ts_type_import`)
- [x] TypeScript parameter type annotations inside inline arrow functions used as attribute expression values are stripped in emitted client output. Repro: `<div onclick={(event: Event) => event.stopPropagation()}>`. Reference emits `(event) => ...`; ours preserves the `: Event` annotation literally. Owning layer: codegen/transform — the script-pipeline TS-stripping pass covers `as`/`satisfies`/`!` on the outer expression, but does not descend into parameters of nested inline functions inside attribute expressions. (test: `ts_strip_handler_param_annotation`)
- [x] Parenthesised `TSAsExpression` heads on optional chains must drop the now-redundant outer parentheses after type stripping. Repro: `const z = (obj?.x as Foo)?.y;`. Reference emits `const z = obj?.x?.y;`; ours keeps the parens, yielding `const z = (obj?.x)?.y;`. Owning layer: codegen/transform — the script-pipeline TS-stripping pass replaces the `TSAsExpression` with its inner expression but leaves the surrounding `ParenthesizedExpression` node intact, even when it no longer affects evaluation order. (test: `ts_strip_as_paren_optional_chain`)
- [x] A `TSAsExpression` (or `TSSatisfiesExpression` / `TSNonNullExpression`) wrapping a literal initializer of a top-level `const` must not hide that initializer from the analyze-side init-known detection that drives attribute dynamism. Repro: `<script lang="ts">const value = 'x' as string;</script><input {value} />`. Reference emits a one-shot `$.set_value(input, value)`; ours wraps it in `$.template_effect(() => $.set_value(input, value))`. Owning layer: analyze (3.B `ScriptAnalysis` — `extract_init_known` in `utils/script_info.rs`) — the helper unwraps `ParenthesizedExpression` / `UnaryExpression` but does not see through TS-only wrappers, so the binding's `init_known` stays `false`, `is_symbol_dynamic` flips it to dynamic, and the attribute is classified as dynamic. (test: `diagnose_ts_cast_const_init_marks_attr_dynamic`)
- [x] An empty trailing line comment as the sole body of a `catch {}` block must not survive script stripping as an orphan statement at the parent scope. Repro: a `try { ... } catch { // }` inside a function declared in a `<script lang="ts">`. Reference drops the comment; ours emits a stray top-level `//` line between the function and the next top-level statement. Owning layer: codegen/transform — the script-pipeline TS-stripping pass empties the catch block but the leading-comment attached to the (now removed) inner placeholder reattaches to the next sibling statement at the outer scope. (test: `ts_strip_catch_empty_comment_orphan`)
- [x] TS function-type parameter names declared inside script type annotations (e.g. `let action: (node: Node) => void;`) must not register as JS bindings in `ComponentSemantics` and therefore must not enter `IdentGen` conflicts. Otherwise codegen's first `gen_ident("node")` for a fragment child anchor returns `node_1` instead of `node`, shifting every subsequent `node_N` by +1. Owning layer: analyze (3.A.1) — `JsSemanticVisitor::visit_formal_parameter` honours the existing `in_ts_type` flag, symmetric to `visit_identifier_reference`. (test: `diagnose_ts_type_param_node_skews_anchor_idents`; unit: `js_visitor_skips_ts_function_type_parameter_binding`)
- [x] Pure TypeScript type syntax must be stripped from the `oxc::Program` exactly once, inside the parser's `js_postprocess` mut-pass (renamed from `span_shift`). After this consolidation, analyze, transform, and codegen never observe `TSAsExpression` / `TSSatisfiesExpression` / `TSNonNullExpression` / `TSTypeAssertion` / `TSInstantiationExpression` / `TSTypeAnnotation` / type-only imports/exports / `declare` members / abstract class members / `TSIndexSignature` / type parameters / type arguments / `TSTypeAliasDeclaration` / `TSInterfaceDeclaration` / `declare global { …type-only… }` / etc. Constructs that have runtime semantics we do NOT compile — `TSEnumDeclaration`, `TSModuleDeclaration` (with non-type body), `TSGlobalDeclaration` (with non-type body), constructor parameter properties (`accessibility`/`readonly`/`override` on `FormalParameter`), decorators, `AccessorProperty` — stay in the AST so analyze validation can emit `TypescriptInvalidFeature` diagnostics; compilation halts on error before transform/codegen. Owning layer: `svelte_parser::js_postprocess` — single `JsPostprocessor: VisitMut` per parse entry point that both span-shifts and TS-strips in one walk, plus retain-fold over `program.body` / `BlockStatement::body` / `FunctionBody::statements` / `StaticBlock::body` / `ClassBody::body` and `std::mem::replace` on TS expression wrappers. Pass runs at every parser entry point: `process_program` (instance + module script + standalone module), `process_expression` (template expression tags), `process_statement` (`{@const}`, each-block context, each-block index), `process_binding_pattern` / `process_formal_parameters` (snippet decl). Migration removed `svelte_transform::transformer::ts_cleanup` (file deleted), all defensive `is_typescript_syntax()` / `in_ts_type` / `SymbolFlags::TypeImport` guards in `JsSemanticVisitor`, `collect_component_top_level_symbol_names`, and `block_semantics::builder::walker`, plus `relocate_orphaned_comments` is now in the parser. (test: `ts_stripped_from_parser_output_in_instance_script`)

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
- [x] `ts_strip_non_null_chain`
- [x] `ts_strip_const_tag`
- [x] `ts_strip_attribute`
- [x] `ts_strip_script_types`
- [x] `diagnose_svg_city_icon`
- [x] `script_jsdoc_comment`
- [x] `diagnose_filename_name_collides_with_ts_type_import`
- [x] `ts_strip_handler_param_annotation`
- [x] `ts_strip_as_paren_optional_chain`
- [x] `ts_strip_catch_empty_comment_orphan`
- [x] `diagnose_ts_cast_const_init_marks_attr_dynamic`
- [x] `ts_stripped_from_parser_output_in_instance_script`
