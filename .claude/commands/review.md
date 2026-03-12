# Code Review

Structured architectural review of the svelte-rs compiler.

## Step 1: Choose scope

Use `AskUserQuestion` to ask:

**Question:** "Какой scope ревью?"
**Options:**
- "Весь проект" — review all crates end-to-end
- "Недавние изменения" — review only files changed since master (`git diff master...HEAD` + uncommitted changes)
- "Конкретный крейт" — review a single crate (ask which one in a follow-up question)

## Step 2: Gather context

Read `CODEBASE_MAP.md` for type signatures and module structure.

**For "Весь проект":**
- Read `lib.rs` and `Cargo.toml` of each crate in `crates/`
- Read `crates/svelte_analyze/src/data.rs` (AnalysisData contract)
- Read `crates/svelte_codegen_client/src/context.rs` (Ctx struct)
- Scan all `use svelte_analyze::` imports in `svelte_codegen_client`
- Scan all `use svelte_codegen_client::` imports in other crates (should be none except `svelte_compiler`)

**For "Недавние изменения":**
- Run `git diff master...HEAD --name-only` and `git diff --name-only` to get changed files
- Read all changed `.rs` files
- For each changed file, also read its module's `mod.rs` or `lib.rs` for context

**For "Конкретный крейт":**
- Ask which crate via `AskUserQuestion` (options: svelte_ast, svelte_parser, svelte_analyze, svelte_codegen_client, svelte_js, svelte_compiler)
- Read all `.rs` files in that crate's `src/`

## Step 3: Review

Use parallel Explore agents (up to 3) to review across these dimensions. Each agent should focus on 2 dimensions.

### 3.1 Boundary integrity

Verify crate dependency boundaries are respected:

- `svelte_codegen_client` may import from `svelte_analyze` ONLY these types: `AnalysisData`, `FragmentKey`, `LoweredFragment`, `FragmentItem`, `ConcatPart`, `ContentType`, `PropsAnalysis`, `PropAnalysis`, `ExportInfo`
- **No back-edges**: codegen must NOT import from svelte_parser; analyze must NOT import from codegen
- **No analysis in codegen**: codegen must NOT re-implement analysis logic (rune detection, mutation tracking, scope building). It should only read precomputed data from `AnalysisData`
- **No codegen decisions in analysis**: analysis must NOT contain logic specific to JS output generation

Flag: any `use svelte_analyze::` import in codegen that is not in the allowed list above.

### 3.2 AnalysisData sufficiency

Check whether codegen has to work around missing analysis data:

- Hardcoded values or heuristics in codegen that should be precomputed in a pass in `svelte_analyze`
- Multiple codegen sites computing the same thing from raw AST — this should be a precomputed field in `AnalysisData`
- Codegen reading AST nodes to derive information that analysis should provide (e.g., checking node children to determine content type instead of using `content_types`)

Flag: patterns where codegen is doing work that belongs in analysis.

### 3.3 Over-engineering

Look for unnecessary complexity:

- Abstractions (traits, structs, enums) used only once
- Trait implementations with a single implementor (exception: `TemplateVisitor` in analyze is intentionally generic)
- Unnecessary generic type parameters
- Builder/factory patterns where a plain function would work
- Feature flags or backwards-compatibility shims that aren't needed
- Wrapper types that add no value

Flag: code that could be simplified without losing functionality.

### 3.4 Architecture conformance

Verify the project follows its stated design principles:

- **Analysis** uses the composite visitor pattern (`walker.rs` + `TemplateVisitor` trait). Independent passes are combined via tuple visitors for a single tree walk. Flag any analysis pass that uses direct recursion instead.
- **Codegen** uses direct recursion over `Component` + `AnalysisData`. Flag any visitor/walker pattern in codegen.
- **AST immutability**: the `Component` AST is never mutated after parsing. All metadata lives in `AnalysisData` side tables. Flag any `&mut Component` or `&mut Fragment` in analysis or codegen.
- **9-pass analysis order**: verify the pass order in `svelte_analyze/src/lib.rs` matches `CODEBASE_MAP.md`
- **Side-table architecture**: codegen re-parses JS expressions from source via stored `Span`s; it does not store parsed AST nodes. Flag any stored OXC AST in `AnalysisData`.

### 3.5 Optimizations

Look for performance issues:

- Redundant `.clone()` calls where a reference would suffice
- `HashMap` lookups in hot loops that could be precomputed into a `Vec` or cached
- `String` allocations where `&str` or `Cow<str>` would work
- Repeated `.collect()` into `Vec` followed by iteration (could use iterators directly)
- Unnecessary `Box` or `Arc` where stack allocation is fine
- Large structs passed by value instead of reference

Flag: with estimated impact (hot path vs cold path).

### 3.6 General quality

- Dead code (unused functions, unreachable match arms)
- `TODO` / `FIXME` / `HACK` audit — list them all with context
- Error handling consistency: are errors properly propagated or silently ignored?
- Naming consistency across crates
- Test coverage gaps for public APIs

## Step 4: Report

Output a structured report grouped by dimension. For each finding use this format:

```
### [Dimension Name]

**[critical/warning/suggestion]** `file_path:line` — Description of the issue.
  → Recommended fix or action.
```

At the end, include a summary:
- Total findings by severity
- Top 3 most impactful issues to address first
