---
name: svelte-reference-map
description: Mapping between Svelte reference compiler and our Rust crates. Loaded automatically when working on compiler crates to quickly find corresponding reference files.
user-invocable: false
paths: "crates/**/*.rs"
---

# Svelte Reference Map

Mapping between the reference Svelte v5 compiler (`reference/compiler/`) and our Rust implementation.

## File mapping

| Feature area | Svelte reference | Our crate |
|---|---|---|
| AST types | `reference/compiler/types/template.d.ts` | `svelte_ast/src/lib.rs` |
| Shared types + OXC utils | -- | `svelte_parser/src/types.rs` |
| Parser + JS pre-parsing | `reference/compiler/phases/1-parse/` | `svelte_parser/src/lib.rs`, `svelte_parser/src/parse_js.rs` |
| Analysis | `reference/compiler/phases/2-analyze/visitors/` | `svelte_analyze/src/` |
| Expression transform | `reference/compiler/phases/3-transform/client/visitors/` (rune rewrites) | `svelte_transform/src/lib.rs` |
| Client codegen entry | `reference/compiler/phases/3-transform/client/transform-client.js` | `svelte_codegen_client/src/lib.rs` |
| Template transform | `reference/compiler/phases/3-transform/client/transform-template/` | `svelte_codegen_client/src/template/` |
| Fragment codegen | `reference/compiler/phases/3-transform/client/visitors/shared/fragment.js` | `svelte_codegen_client/src/template/mod.rs` |
| Element codegen | `reference/compiler/phases/3-transform/client/visitors/RegularElement.js` + `shared/element.js` | `svelte_codegen_client/src/template/element.rs` |
| Attributes | `reference/compiler/phases/3-transform/client/visitors/Attribute.js` + `SpreadAttribute.js` | `svelte_codegen_client/src/template/attributes.rs` |
| IfBlock | `reference/compiler/phases/3-transform/client/visitors/IfBlock.js` | `svelte_codegen_client/src/template/if_block.rs` |
| EachBlock | `reference/compiler/phases/3-transform/client/visitors/EachBlock.js` | `svelte_codegen_client/src/template/each_block.rs` |
| ConstTag | `reference/compiler/phases/3-transform/client/visitors/ConstTag.js` | `svelte_codegen_client/src/template/const_tag.rs` |
| BindDirective | `reference/compiler/phases/3-transform/client/visitors/BindDirective.js` | `svelte_codegen_client/src/template/attributes.rs` |
| Script transform | `reference/compiler/phases/3-transform/client/visitors/Program.js` + `VariableDeclaration.js` | `svelte_codegen_client/src/script.rs` |
| JS builders | `reference/compiler/utils/builders.js` | `svelte_codegen_client/src/builder.rs` |

## Concept mapping

| Svelte concept | Our equivalent |
|---|---|
| `context.state.init` / `context.state.update` | `InitStatements`, `UpdateStatements` returned from `gen_*` functions |
| `node.metadata.foo` (mutated during analyze) | `ctx.analysis.foo(node_id)` (immutable side table) |
| `b.call('$.get', ...)` inline in visitor | Transform phase rewrites Expression, codegen just emits |
| `context.state.template.push(...)` | `TemplateBuilder::push_str()` in codegen |
| `context.visit(node)` | Direct recursion in our codegen |
| `b.call(...)`, `b.declaration(...)` | `builder.rs` equivalents |
