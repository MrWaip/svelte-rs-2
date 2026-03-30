---
name: svelte-reference-map
description: Mapping between the reference Svelte compiler in `reference/compiler/` and the Rust implementation in this repo. Use when porting a feature, tracing where a Svelte concept lives in either codebase, or matching a Rust module to its reference-compiler counterpart.
---

# Svelte Reference Map

Use this skill to jump between the reference JS compiler and the Rust crates quickly.

## File mapping

| Feature area | Svelte reference | Our crate |
|---|---|---|
| AST types | `reference/compiler/types/template.d.ts` | `svelte_ast/src/lib.rs` |
| Shared types + OXC utils | -- | `svelte_parser/src/types.rs` |
| Parser + JS pre-parsing | `reference/compiler/phases/1-parse/` | `svelte_parser/src/lib.rs`, `svelte_parser/src/parse_js.rs` |
| Analysis | `reference/compiler/phases/2-analyze/visitors/` | `svelte_analyze/src/` |
| Expression transform | `reference/compiler/phases/3-transform/client/visitors/` | `svelte_transform/src/lib.rs` |
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
| `context.state.init` / `context.state.update` | `InitStatements` and `UpdateStatements` returned from `gen_*` helpers |
| `node.metadata.foo` | `ctx.analysis.foo(node_id)` |
| `context.visit(node)` | direct recursion in our codegen |
| `b.call(...)`, `b.declaration(...)` | `builder.rs` helpers |
| mutable metadata attached during analyze | immutable side tables in `AnalysisData` |

## Porting rule

Match the reference compiler's behavior and output, not its internal architecture. Use the mapping above to find the right file, then re-implement the behavior using this repo's phase boundaries and side-table design.
