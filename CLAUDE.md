# Project Instructions

Detailed crate API and type reference: `CODEBASE_MAP.md` (read when you need type signatures or module structure).

## Testing

All tests in `crates/svelte_parser` must follow the span-based pattern described in `/test-pattern`.

Rules:
- Use `assert_node`, `assert_script`, `assert_if_block` helpers (defined in the test module)
- No inline `if let Node::...` structural checks — use helpers instead
- Add new `assert_<node_type>` helpers when new node types need testing
- Exception: `assert!(result.is_err())` for error tests needs no helper

When writing or modifying any test in `svelte_parser`, apply `/test-pattern` automatically.

## Porting from Svelte compiler

Reference Svelte compiler source is in `reference/compiler/`. Use it to understand how features should work.

### Quick navigation

| Feature area | Svelte reference | Our crate |
|---|---|---|
| AST types | `reference/compiler/types/template.d.ts` | `svelte_ast/src/lib.rs` |
| Parser | `reference/compiler/phases/1-parse/` | `svelte_parser/src/lib.rs` |
| Analysis | `reference/compiler/phases/2-analyze/visitors/` | `svelte_analyze/src/` |
| Client codegen entry | `reference/compiler/phases/3-transform/client/transform-client.js` | `svelte_codegen_client/src/lib.rs` |
| Template transform | `reference/compiler/phases/3-transform/client/transform-template/` | `svelte_codegen_client/src/template/` |
| Fragment codegen | `reference/compiler/phases/3-transform/client/visitors/shared/fragment.js` | `svelte_codegen_client/src/template/mod.rs` |
| Element codegen | `reference/compiler/phases/3-transform/client/visitors/RegularElement.js` + `shared/element.js` | `svelte_codegen_client/src/template/element.rs` |
| Attributes | `reference/compiler/phases/3-transform/client/visitors/Attribute.js` + `SpreadAttribute.js` | `svelte_codegen_client/src/template/attributes.rs` |
| IfBlock | `reference/compiler/phases/3-transform/client/visitors/IfBlock.js` | `svelte_codegen_client/src/template/if_block.rs` |
| EachBlock | `reference/compiler/phases/3-transform/client/visitors/EachBlock.js` | `svelte_codegen_client/src/template/each_block.rs` |
| BindDirective | `reference/compiler/phases/3-transform/client/visitors/BindDirective.js` | `svelte_codegen_client/src/template/attributes.rs` |
| Script transform | `reference/compiler/phases/3-transform/client/visitors/Program.js` + `VariableDeclaration.js` | `svelte_codegen_client/src/script.rs` |
| JS builders | `reference/compiler/utils/builders.js` | `svelte_codegen_client/src/builder.rs` |

To port a new feature, use `/port-svelte <feature description>`.
