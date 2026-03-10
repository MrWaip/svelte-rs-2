# Port Svelte feature: $ARGUMENTS

Reference Svelte compiler is in `reference/compiler/`. Our Rust compiler is in `crates/svelte_*`.

## Step 1: Test case

- Create `tasks/compiler_tests/cases2/<test_name>/case.svelte` with a minimal example of the feature
- Run `cargo run -p generate_test_cases` to generate `case-svelte.js` (expected output from Svelte v5)
- Add test function in `tasks/compiler_tests/test_v3.rs`: `fn <test_name>() { assert_compiler("<test_name>"); }`

## Step 2: Parser & AST

Check if `case.svelte` uses syntax not yet supported by our parser.

- Compare with `reference/compiler/types/template.d.ts` for AST node shapes
- If new node types, attributes, or directives are needed:
  1. Add types to `crates/svelte_ast/src/lib.rs`
  2. Add parsing to `crates/svelte_parser/src/lib.rs` (and scanner if new tokens needed)
  3. Add parser tests following `/test-pattern`

## Step 3: Analysis

Read the Svelte analysis visitors in `reference/compiler/phases/2-analyze/visitors/` to understand what metadata the feature needs.

- Check `node.metadata` fields used by the transform visitor
- Verify our `AnalysisData` has equivalent data
- If not, add or extend a pass in `crates/svelte_analyze/src/`
- Add analyze tests following `/test-pattern`

## Step 4: Codegen

Read the Svelte transform visitor in `reference/compiler/phases/3-transform/client/visitors/`.

Key navigation:
| Feature | Svelte reference | Our module |
|---|---|---|
| Fragment | `client/visitors/shared/fragment.js` | `template/mod.rs` |
| Element | `client/visitors/RegularElement.js` + `shared/element.js` | `template/element.rs` |
| Attributes | `client/visitors/Attribute.js` + `SpreadAttribute.js` | `template/attributes.rs` |
| IfBlock | `client/visitors/IfBlock.js` | `template/if_block.rs` |
| EachBlock | `client/visitors/EachBlock.js` | `template/each_block.rs` |
| BindDirective | `client/visitors/BindDirective.js` | `template/attributes.rs` |
| Script | `client/visitors/Program.js` + `VariableDeclaration.js` | `script.rs` |

Implement in the corresponding `svelte_codegen_client` module.

Key differences from Svelte:
- We use direct recursive functions, not AST walker (zimmerframe)
- We use `AnalysisData` side tables, not mutated AST metadata
- We store `Span` and re-parse in codegen via `svelte_js`, not stored expressions
- Our `$.template()` builds equivalent calls to Svelte's `html` tagged template

## Step 5: Verify

Run the test:
```
cargo test -p compiler_tests --test compiler_tests_v3 <test_name>
```

Compare `case-rust.js` vs `case-svelte.js`. Fix mismatches. Ensure all existing tests still pass:
```
cargo test -p compiler_tests --test compiler_tests_v3
```
