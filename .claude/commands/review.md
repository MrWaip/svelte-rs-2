# Code Review

Structured cross-cutting architectural review of the svelte-rs compiler.

For per-crate reviews use: `/review-parser`, `/review-analyzer`, `/review-transform`, `/review-codegen`.

## Step 1: Determine scope

Scope is determined by `$ARGUMENTS`:
- **no arguments** → review files changed since master: `git diff master...HEAD --name-only` + `git diff --name-only`
- **`all`** → review all crates end-to-end

## Step 2: Gather context

Read `CODEBASE_MAP.md` for type signatures and module structure.

**For changed files (default):**
- Run `git diff master...HEAD --name-only` and `git diff --name-only` to get changed files
- Read all changed `.rs` files
- For each changed file, also read its module's `mod.rs` or `lib.rs` for context

**For `all`:**
- Read `lib.rs` and `Cargo.toml` of each crate in `crates/`
- Read `crates/svelte_analyze/src/data.rs` (AnalysisData contract)
- Read `crates/svelte_codegen_client/src/context.rs` (Ctx struct)
- Scan all `use svelte_analyze::` imports in `svelte_codegen_client` and `svelte_transform`
- Scan all `use svelte_codegen_client::` imports in other crates (should be none except `svelte_compiler`)

## Step 3: Review

Use parallel Explore agents (up to 3) to review across these dimensions. Each agent should focus on 2 dimensions.

### 3.1 Boundary integrity

Verify crate dependency boundaries are respected:

- `svelte_codegen_client` may import from `svelte_analyze` ONLY these types: `AnalysisData`, `FragmentKey`, `LoweredFragment`, `FragmentItem`, `ConcatPart`, `ContentType`, `PropsAnalysis`, `PropAnalysis`, `ExportInfo`
- `svelte_transform` may import from `svelte_analyze`: `AnalysisData`, `ParsedExprs`, `ScopeId`, scoping types
- `svelte_transform` may import from `svelte_ast`: `Component`, `Fragment`, `Node`, `NodeId`, `Attribute`
- `svelte_transform` may import from `svelte_js`: `RuneKind`
- **No back-edges**: codegen must NOT import from svelte_parser; analyze must NOT import from codegen or transform
- **No analysis in codegen/transform**: codegen must NOT re-implement analysis logic (rune detection, mutation tracking, scope building). Transform reads precomputed data from `AnalysisData`
- **No codegen decisions in analysis**: analysis must NOT contain logic specific to JS output generation
- **Pipeline order**: parse → analyze → transform → codegen. Transform mutates pre-parsed expressions in-place; codegen consumes them read-only

Flag: any import that violates these boundaries.

### 3.2 AnalysisData sufficiency

Check whether downstream phases have to work around missing analysis data:

- Hardcoded values or heuristics in codegen/transform that should be precomputed in `svelte_analyze`
- Multiple sites computing the same thing from raw AST — should be a precomputed field in `AnalysisData`
- Codegen reading AST nodes to derive information that analysis should provide

Flag: patterns where codegen or transform is doing work that belongs in analysis.

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
- **Transform** uses direct recursion over the template tree + OXC expression trees. It mutates pre-parsed `Expression` ASTs in-place. Flag any visitor pattern in transform.
- **Codegen** uses direct recursion over `Component` + `AnalysisData` + pre-transformed `ParsedExprs`. Flag any visitor/walker pattern in codegen.
- **AST immutability**: the `Component` AST is never mutated after parsing. All metadata lives in `AnalysisData` side tables. The only mutation is in `ParsedExprs` (OXC expressions transformed in-place by `svelte_transform`). Flag any `&mut Component` or `&mut Fragment`.
- **Pass order**: verify the analysis pass order in `svelte_analyze/src/lib.rs` matches `CODEBASE_MAP.md`
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
