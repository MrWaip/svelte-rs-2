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

### svelte_analyze tests

Tests live in `crates/svelte_analyze/tests/`. Each test parses a `.svelte` snippet, runs `analyze()`, and asserts on `AnalysisData` fields. Follow `/test-pattern` for helpers and structure.

### svelte_codegen_client tests

Compiler tests live in `tasks/compiler_tests/cases2/`. Each case has `case.svelte` (input), `case-svelte.js` (expected), `case-rust.js` (actual).

## Just commands

All common operations are in the `justfile`. Use `just` instead of raw cargo commands:

| Command | What it does |
|---|---|
| `just generate` | Generate `case-svelte.js` for all compiler test cases (runs node + oxc) |
| `just test-compiler` | Run all compiler integration tests |
| `just test-case <name>` | Run a single compiler test case |
| `just test-case-verbose <name>` | Run a single compiler test case with output |
| `just test-all` | Run all tests across all crates |
| `just test-parser` | Run parser tests |
| `just test-analyzer` | Run analyzer tests |

| `just generate-benchmark [name] [chunks]` | Generate benchmark `.svelte` file (default: `big_v1`, 50 chunks) |
| `just compare-benchmark [file]` | Wall-clock comparison: Rust vs Svelte JS compiler |

## Benchmarks

Two complementary systems:

- **CodSpeed (CI, automatic)** — runs on every push, tracks CPU instruction count via Valgrind. Deterministic, no noise. Alerts on regressions. Only measures Rust.
- **`just compare-benchmark` (local, manual)** — wall-clock Rust vs JS comparison. Run after major changes or for reporting.

Benchmark files are versioned (`big_v1.svelte`, `big_v2.svelte`, ...). When porting a feature that adds new syntax, `/port-svelte` Step 8 creates a new version. Old files are never modified — their CodSpeed history stays valid.

## General rules for commands

If stuck after 3 attempts on the same issue, stop and report what you've tried. Do not loop indefinitely.

## Porting from Svelte compiler

Reference Svelte compiler source is in `reference/compiler/`. Use it to understand **what** output to produce, not **how** to implement it.

### Design principle

Match the JS output exactly. Design the internals for Rust: direct recursion over side tables,
no mutable AST metadata. Don't replicate JS workarounds,
intermediate abstractions, or patterns that exist only because of zimmerframe/estree-walker.

**Exception — `svelte_analyze` uses a single-pass composite visitor** (`walker.rs`).
Each analysis pass implements `TemplateVisitor` for only the nodes it cares about.
Independent passes are combined into a single tree traversal via tuple composite visitors
(e.g., `(ReactivityVisitor, ElseifVisitor)` = one walk instead of two).
Codegen (`svelte_codegen_client`) uses direct recursion — no visitor pattern there.

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

Read `TODO.md` for current work items. Full feature catalog in `ROADMAP.md`.
