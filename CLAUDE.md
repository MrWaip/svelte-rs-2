# Project Instructions

Detailed crate API and type reference: `CODEBASE_MAP.md` (read when you need type signatures or module structure).
Gotchas, data flow per pass, node-type checklist, output examples: `GOTCHAS.md` (read when adding a new feature or debugging unexpected output).

## Testing

All tests in `crates/svelte_parser` must follow the span-based pattern described in `/test-pattern`.

Rules:
- Use `assert_node`, `assert_script`, `assert_if_block` helpers (defined in the test module)
- Prefer `assert_<node_type>` helpers for repeated structural checks. One-off field access or `if let` is acceptable when the check is unique to a single test.
- Add new `assert_<node_type>` helpers when new node types need testing
- Exception: `assert!(result.is_err())` for error tests needs no helper

When writing or modifying any test in `svelte_parser`, apply `/test-pattern` automatically.

### svelte_analyze tests

Tests live in `crates/svelte_analyze/src/tests.rs`. Each test parses a `.svelte` snippet via `parse_with_js()`, runs `analyze()`, and asserts on `AnalysisData` fields. Follow `/test-pattern` for helpers and structure.

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
Codegen (`svelte_codegen_client`) uses direct recursion — no visitor pattern. Extract shared logic between root and nested fragment codegen into common functions; direct recursion ≠ code duplication.

### Quick navigation

| Feature area | Svelte reference | Our crate |
|---|---|---|
| AST types | `reference/compiler/types/template.d.ts` | `svelte_ast/src/lib.rs` |
| Shared types + OXC utils | — | `svelte_types/src/lib.rs` |
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

To port a new feature, use `/port-svelte <feature description>`.

Read `ROADMAP.md` for the full feature catalog and current priorities.

When `/port-svelte` discovers deferred items (edge cases, validations, blocked work), add them to the **Deferred** section at the bottom of `ROADMAP.md`, grouped under the parent feature name with its tier reference (e.g., `### feature-name (Tier N)`).

### Legacy features (Svelte 4 → removed in Svelte 6)

Legacy Svelte 4 syntax (deprecated in Svelte 5, scheduled for removal in Svelte 6) is ported with isolation in mind so it can be cleanly deleted later.

**Conventions:**

1. **`Legacy` suffix** in all type/function names: `OnDirectiveLegacy`, `gen_on_directive_legacy`, `build_legacy_event_handler`.
2. **`LEGACY(svelte4):` doc-comment** on every struct, enum variant, and top-level function:
   ```rust
   /// LEGACY(svelte4): on:directive syntax. Deprecated in Svelte 5, remove in Svelte 6.
   pub struct OnDirectiveLegacy { ... }
   ```
   Short inline comments use the same tag: `// LEGACY(svelte4): on:directive handled separately`.
3. **Easy removal** — keep legacy code in self-contained blocks/functions. Avoid mixing legacy logic into non-legacy code paths. Ideal: grep `LEGACY(svelte4)` → delete all hits → compile → done.

## Code style

### Architecture boundaries — STRICT ENFORCEMENT

**Before proposing or writing code, verify it goes in the correct layer. Never take a shortcut by placing logic in the wrong crate.**

Layers and their responsibilities:
- `svelte_types` — shared domain types (`ExpressionInfo`, `RuneKind`, `ParsedExprs`, `JsParseResult`, etc.) and OXC parsing utilities. Leaf crate, depended on by all others.
- `svelte_parser` — produces immutable AST. Owns JS expression pre-parsing (`parse_js` → `JsParseResult`). Entry point: `parse_with_js(&alloc, source) → (Component, JsParseResult, Diagnostics)`.
- `svelte_analyze` — single-pass composite visitor. Owns ALL derived data, classifications, flags, precomputation → `AnalysisData` side tables (keyed by `NodeId`). Entry point: `analyze(&component, js_result) → (AnalysisData, ParsedExprs, Diagnostics)`.
- `svelte_codegen_client` — consumes AST + AnalysisData + ParsedExprs to produce JS output. Owns only JS output construction logic.

Boundary rules:
1. **Immutable AST** — AST is immutable after parsing. Derived data goes into `AnalysisData`, never into AST nodes.
2. **Analysis owns classification** — any derived data, classification, flag, or precomputation belongs in `svelte_analyze`. If codegen would need to re-traverse AST nodes to collect/classify data, that data must be computed in analyze instead.
3. **JS parsing in parser** — JS expression parsing belongs in `svelte_parser` (`parse_js`), not in analyze or codegen.
4. **SymbolId over strings** — all identifier lookups must go through `SymbolId`. `FxHashSet<String>` and `FxHashMap<String, _>` must never be keyed by identifier names. The only acceptable use of name strings is in JS output generation (building string literals, property names for emitted code). If `SymbolId` is not available for a given scope level, extend `ComponentScoping` — do not fall back to string sets.
5. **OXC as direct dependency** — OXC types (`Expression<'a>`, `Program<'a>`) are used directly across crates. `svelte_types` provides shared domain types and OXC parsing utilities; `ParsedExprs<'a>` carries OXC ASTs from parser through analyze/transform to codegen.
6. **No codegen data caching** — codegen-internal enums/structs that cache or duplicate AST data to avoid re-lookups are a smell. The classification belongs in `AnalysisData`.
7. **Correct over minimal** — never propose a "simple" or "minimal" fix in the wrong layer when a correct architectural approach exists. If unsure which layer owns the logic, ask.

Additional rules:
- `FxHashMap`/`FxHashSet` everywhere instead of std `HashMap`.
- Sub-struct fields in `AnalysisData` (`ElementFlags`, `FragmentData`, etc.) are `pub(crate)` — use accessor methods from outside `svelte_analyze`. In codegen, prefer `Ctx` shortcuts over chained access through `ctx.analysis.sub_struct.method()`.
- AST stores `Span` for JS expressions. `ParsedExprs<'a>` (defined in `svelte_types`) caches parsed OXC `Expression<'a>` ASTs (populated in `svelte_parser::parse_js`, consumed in transform/codegen). No JS subtree copying between phases.
- OXC and `ComponentScoping` share the same `SymbolId` space for script-level bindings, so `SymbolId` from OXC can be used directly with `ComponentScoping` methods without name round-tripping.

### Naming

- `gen_*` — creates and returns statements.
- `process_*` — mutates provided `&mut Vec` in-place.
- `emit_*` — appends specialized statements to a `&mut Vec`.
- `pub(crate)` by default; `pub` only for entry points and types.

### Rust idioms

- Early return over deep nesting.
- Exhaustive `match` for enums; `if let` when only one variant matters.
- `.copied()`, `.is_some_and()`, `.map_or()` over verbose match/if-let for simple Option ops.
- `.remove()` for ownership transfer from side tables (not `.get().cloned()`).
- `unwrap_or_else(|| panic!(...))` only for internal invariants, never for user errors. User errors → `Diagnostic`.
- Repeating `match` patterns on an enum → extract into a method on that enum.
- Comments answer "why", never describe what the line does.

### Phase boundaries: fat analyze, dumb codegen

Each compiler phase has a strict responsibility. When adding new features, place logic in the correct phase:

- **Parser** (`svelte_parser`) — returns structured data. If a new AST node contains JS, the parser must deliver it parsed (via `svelte_types`), not as a raw Span for downstream re-parsing. Never introduce a Span-only field that forces analyze or codegen to parse text.
- **Analyze** (`svelte_analyze`) — answers semantic questions. If codegen needs to decide between output modes based on 2+ flags, analyze should pre-compute that decision and expose it as an enum or accessor. Codegen should never dig deeper than one method call into `AnalysisData`.
- **Codegen** (`svelte_codegen_client`) — flat mapper. Match on enums, format output. Zero decision logic. No `oxc_parser::Parser::new()`, no `starts_with('{')` heuristics, no multi-pass traversal of `el.attributes` to collect information.

**Red flags in codegen** (do not introduce):
- `oxc_parser::Parser::new()` — re-parsing in codegen means parser missed structure
- `starts_with('[')`, `split(',')`, `split_once(':')` — string parsing means AST lost structure
- `.iter().find(...)` + `.iter().filter_map(...)` + `.iter().any(...)` on the same collection — repeated traversal means analyze should provide a summary
- `ctx.analysis.foo(id).and_then(|x| x.bar.first()).and_then(|r| r.baz).is_some_and(|s| ...)` — deep chaining means analyze should expose an accessor
- `let needs_X = flag_a || (flag_b && flag_c)` combining 2+ analysis flags — means analyze should pre-compute the decision

**Green flags in codegen** (this is what we want):
```rust
// Single accessor call → flat match
match ctx.attr_output_mode(id) {
    AttrOutputMode::Static => { ... }
    AttrOutputMode::DynamicGetter => { ... }
}
```

When in doubt, run `/audit-boundaries` to check for violations, then `/migrate-boundary #N` to fix them.
