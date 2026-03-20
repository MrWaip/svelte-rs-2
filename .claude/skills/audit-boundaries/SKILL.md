# Audit: Phase Boundary Violations in Codegen & Transform

## Context

You are auditing a Svelte compiler written in Rust, built on the OXC ecosystem.

### Architecture

```
Parser → AST (structured data, no Span-only fields where structure is needed)
           ↓
       Analyze → AnalysisData (facts + derived decisions: enums, accessors, pre-computed answers)
           ↓
       Transform → mutated ParsedExprs (rewrites rune reads/writes)
           ↓
       Codegen_client → JavaScript output
       (future: Codegen_ssr, Codegen_native — each consumes AnalysisData independently)
```

### Design principle: fat analyze, dumb codegen

- **Parser** delivers structured AST — no Span-only fields that force downstream re-parsing
- **Analyze** delivers ready answers — enums, pre-computed bools, accessor methods. Codegen should never dig deeper than one level into AnalysisData
- **Codegen** is a flat mapper — match on enums, format output. Zero decision logic

Red flag in codegen:
```rust
ctx.analysis.attr_expression(id)
    .and_then(|info| info.references.first())
    .and_then(|r| r.symbol_id)
    .is_some_and(|sym| ctx.is_import_sym(sym))
```

Green flag in codegen:
```rust
ctx.attr_needs_getter(id)
```

No separate HIR/Lower crate — analyze itself provides the semantic decisions.
Each codegen backend asks analyze different questions and maps answers to its own runtime.

## Execution

**Read-only.** Agents may read files. Do not run `cargo test`, `cargo check`, `cargo build`, or any command that modifies the working tree.

Launch agents immediately — one per violation class (Class 1–4), each scanning independently across both `svelte_codegen_client` and `svelte_transform`. Each agent gathers its own context as its first step — no sequential prefetch. All agents also read `CODEBASE_MAP.md` for type signatures and module structure.

**Wait for every agent to report completion before writing the report.** Do not poll agent output files manually. Do not fall back to doing the audit yourself. Do not start writing the output until all agents have returned their results. If an agent takes longer than expected, wait — do not proceed without its findings.

After all agents complete, compile their findings into a single report (see Output Format).

## What to Find

Scan these crates for phase boundary violations:
- `crates/svelte_codegen_client/`
- `crates/svelte_transform/src/`

Cross-reference with `crates/svelte_analyze/src/` to understand what data IS available vs what codegen/transform recompute themselves.

### Violation Classes (ordered by severity)

#### Class 1: Full Re-parse in Codegen
Codegen instantiates a parser (OXC, regex, or other) to parse source text that the parser phase should have already structured.

**Signals:**
- `oxc_parser::Parser::new()` inside codegen/transform
- Building a new allocator + parsing expressions from string slices
- Any `parse_*` function defined inside codegen

**Example:** `parse_ce_expression` in custom element codegen creates a new OXC parser instance to parse the custom element config object from source text. This should be a structured AST node from the parser.

#### Class 2: String Re-parsing
Codegen/transform manually extracts structure from string representations using string operations, instead of receiving structured data from the parser.

**Signals:**
- `starts_with('[')`, `starts_with('{')` to determine syntax kind
- `split(',')`, `split('=')`, `split_once(':')` to extract names
- `&pattern[1..pattern.len()-1]` style index slicing
- `.trim()` chains on extracted substrings

**Example:** `gen_destructured_callback` determines array vs object destructuring via `pattern.starts_with('[')`, parses variable names via split/trim, handles aliases via `split_once(':')`. This should come from the parser as a typed structure: `enum DestructureKind { Array, Object }` with `Vec<DestructuredBinding { name, alias, default_value }>`.

#### Class 3: AST Re-traversal in Codegen
Codegen/transform iterates over child nodes (attributes, children, fragments) with find/filter/any to collect information that Analyze could have provided as a ready-made structure.

**Signals:**
- `.iter().find(|a| matches!(a, ...))` inside codegen
- `.iter().filter_map(...)` to collect specific node types
- `.iter().any(...)` to check for existence of specific variants
- These patterns appear in codegen for DATA COLLECTION, not for output generation

**Example:** `process_class_attribute_and_directives` traverses `el.attributes` three times — `find` for the class attr, `filter_map` for class directives, `any` to check if any directive is dynamic. All of this should come from analyze as a single `ClassOutputInfo` structure.

#### Class 4: Derived Flags Without a Name
Codegen combines 2+ boolean/analysis flags into a composite decision that determines the output form. The same combination may appear in multiple places.

**Signals:**
- `let needs_X = flag_a || (flag_b && flag_c)` patterns
- Multi-branch `if/else if/else` chains where each branch produces a different output shape
- Same flag combination appearing in 2+ files
- A `match` or `if` that could be replaced by a single enum/accessor from analyze

**Calibration example:** In attribute codegen, `let needs_memo = has_call || (is_non_simple && is_dynamic)`, followed by branching on `is_dynamic` and `shorthand`. Four boolean flags combine into one of four output modes (static / shorthand / dynamic getter / memoized). This should be a single `AttrOutputMode` enum from analyze.

**Boundary case example:** `gen_render_tag` receives `is_dynamic`, `is_chain`, `callee_is_getter` from analyze via dedicated API methods and branches on them. The flags are not recomputed, and the combination appears in one place only. This is borderline — mention it but with low priority.

### What is NOT a Violation

Flag violations only when:
- The same flag combination repeats in 2+ places, OR
- Codegen itself extracts/computes the flags from AST or strings (Classes 1-3)

Do NOT flag:
- Simple `if/else` on a single flag from analysis
- One-off branching that only appears in one place and uses pre-computed data
- Output formatting logic (choosing between `$.event` vs `$.delegated` call syntax — that IS codegen's job)
- Dev mode guards (`if ctx.dev { ... }`)

### Good Examples (reference point for target state)

Mark places where codegen CORRECTLY consumes data — takes a ready structure from analyze/transform and only maps it to output without additional logic. Output these in a separate "✅ Good Examples" section.

**Example 1:** `gen_key_block` — takes `build_node_thunk`, `gen_fragment`, assembles `$.key()` call. No re-parsing, no flag combination, no AST traversal.

**Example 2:** `gen_custom_element` uses `ctx.analysis.exports` — just maps to an array of strings. Data came ready, codegen only formats.

**Example 3:** `emit_debug_tags` — takes `debug_tags_for_fragment`, pre-transformed expressions from `ctx.parsed.debug_tag_exprs`, assembles output. Zero analysis logic.

## Output Format

For each found violation:
- **Pattern**: short name (e.g. "manual reactivity mode detection")
- **Class**: 1-4 (from above)
- **Where**: file:lines (all occurrences)
- **Occurrence count**: across the codebase
- **What is aggregated**: list the flags/checks being combined
- **Proposed type**: what to add in parser/analyze. Calibration:
  - Single bool decision → accessor method (`fn needs_getter(&self, id) -> bool`)
  - 3+ distinct output modes → enum (`enum AttrOutputMode { Static, Shorthand, ... }`)
  - Structured data lost by parser → AST type (`struct DestructuredBinding { name, alias, default }`)
- **Target layer**: "parser" (Class 1-2, structured AST), "analyze" (Class 3-4, accessors/enums), or "codegen refactor" (internal cleanup, no cross-crate change)

Sort: by class (ascending = most severe first), then by occurrence count (descending) within each class.

For good examples:
- **Where**: file:function
- **Why it's good**: one sentence
