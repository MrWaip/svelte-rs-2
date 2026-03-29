---
description: Find phase boundary and data flow violations across compiler layers. Use when tech debt accumulated, after porting features, or periodically for hygiene.
argument-hint: "[crate-or-path | empty=all-codegen]"
allowed-tools: Bash, Read, Grep, Glob, Agent, LSP, Write
---

# Boundary & Data Flow Review: $ARGUMENTS

Find places where code violates phase boundaries or data flows through the wrong path.

## Scope

Determined by `$ARGUMENTS`:
- **No argument** → `svelte_codegen_client` + `svelte_transform` (most common violation sites)
- **Crate name** (e.g. `codegen`, `transform`, `analyze`) → that crate only
- **File path** → that file only

## Step 1: Gather context

Read before reviewing:
1. `CODEBASE_MAP.md` — type signatures, AnalysisData fields, module structure
2. `crates/svelte_analyze/src/data.rs` — what accessors/enums already exist
3. `crates/svelte_codegen_client/src/context.rs` — what Ctx shortcuts exist

**You must know what analyze already provides before claiming something is missing.**

## Step 2: Review

For each file in scope, look for these violation classes. Each class has a definition, signals, and a calibration example showing the boundary between violation and acceptable code.

### Class 1: Re-parsing

Codegen/transform creates a new parser or extracts structure from source text that the parser should have delivered as AST.

**Signals:**
- `oxc_parser::Parser::new()` in codegen/transform
- Building a new allocator + parsing from string slices

**IS a violation:** `parse_ce_expression` in codegen creates a new OXC parser to parse custom element config from source text.
**NOT a violation:** `source_text(span)` to read a string literal value — this is span reading, not parsing. Flag only if the result is then split/parsed further.

### Class 2: String classification

Codegen/transform uses string comparisons on tag names, attribute names, or identifier names to make classification decisions that analyze should pre-compute.

**Signals:**
- `tag_name == "input"`, `a.name == "style"`, `a.name == "class"`
- `name.starts_with('$')` to detect stores
- `.as_str() == "..."` for identifier classification

**IS a violation:** `tag_name == "input" && a.name == "value"` to decide `$.set_value` vs `$.set_attribute` — classification decision, should be an enum from analyze.
**NOT a violation:** String comparisons in the parser itself, or in analyze during classification passes. Also not a violation: string literals used to construct JS output (e.g. `"$.get"`).

### Class 3: AST re-traversal

Codegen/transform iterates children with find/filter/any to collect data that analyze should have pre-computed into a side table.

**Signals:**
- `.iter().find(|a| matches!(a, ...))` in codegen for DATA COLLECTION
- `.iter().filter_map(...)` + `.iter().any(...)` on same collection
- Walking member expression chains to find root identifier

**IS a violation:** `el.attributes.iter().find(class attr).filter_map(class directives).any(dynamic check)` — three traversals to collect info analyze should provide as `ClassOutputInfo`.
**NOT a violation:** Iterating children for OUTPUT GENERATION (e.g. `for child in fragment.nodes` to emit each child's JS). Codegen must walk the tree to produce output — that's its job.

### Class 4: Raw handoff

Analyze collects facts but doesn't draw conclusions. Downstream combines 2+ flags into a decision without a named enum/accessor.

**Signals:**
- `let needs_X = flag_a || (flag_b && flag_c)` in codegen/transform
- Multi-branch `if/else if/else` where each branch produces a different output shape
- `ctx.analysis.foo(id).and_then(|x| x.bar).and_then(|r| r.baz)` — deep chaining into AnalysisData

**IS a violation:** `let needs_memo = has_call || (is_non_simple && is_dynamic)` combining 3 flags into one of 4 output modes — should be `AttrOutputMode` enum.
**NOT a violation:** Single flag read: `if ctx.analysis.is_dynamic(id)`. Also not: combining a pre-computed flag with a post-transform value (e.g. `kind != StateRaw && should_proxy(&transformed_value)` — second operand depends on transform output).

### Class 5: Late knowledge

A downstream phase re-discovers something an upstream phase already knew. The knowledge exists but was discarded or not passed through.

**Signals:**
- Analyze computes a value, stores only a boolean, downstream re-derives the full value
- Transform re-classifies identifiers that analyze already classified
- Same function called in multiple phases (e.g. `detect_rune()` in both parser and analyze)

**IS a violation:** Analyze strips "capture" suffix from event names to set `capture: bool`, discards stripped name. Codegen calls `strip_capture_event()` again.
**NOT a violation:** Transform reading `ComponentScoping` accessors — that's consuming analyze output, not re-discovering.

### Class 6: Types that lie

A type permits states that the data flow never produces. Consumers handle impossible cases.

**Signals:**
- `Option<T>` that is always `Some` after a certain phase
- Field with different semantics depending on which map it lives in
- Name implies deep check, implementation is shallow

**IS a violation:** `ExpressionInfo::has_state` always equals `is_dynamic` for template expressions but means something different for attr expressions — same field, different semantics per map.
**NOT a violation:** `Option<ScopeId>` with `.unwrap_or(parent_scope)` — legitimate fallback.

## Step 3: Verify each finding

Before reporting a finding, you MUST:

1. **Check existing accessors** — grep `data.rs`, `context.rs`, `scope.rs` for an accessor that already provides what you think is missing. If it exists, the finding is invalid.
2. **Check consumers** — who reads the problematic code? If only one consumer in one place, impact is low. If 3+ consumers repeat the same pattern, impact is high.
3. **Check if justified** — some patterns look like violations but are intentional (e.g. string check for unresolved globals like `$effect` where no SymbolId exists). If after investigation the pattern is justified — **drop it, do not include**.

## Step 4: Aggregate

After collecting findings across files:

1. **Group by root cause** — if the same violation appears in 3 files, it's ONE finding with 3 occurrences, not 3 findings
2. **Rank by downstream impact** — how many lines of code get simpler if this root cause is fixed?
3. **Cross-reference** — does fixing finding A also fix finding B? Note dependencies.

## Step 5: Report

Save to `BOUNDARY_REVIEW.md` in project root (overwrite if exists).

Format per finding:

```
### #N — [One-line title]

**Class**: [1-6]
**Impact**: critical | warning | suggestion
**Occurrences**: N locations across M files
**Evidence**:
- `file:line` — [what happens here, quote the pattern]

**Verified**: [what you checked — existing accessors, consumers, justification]

**Root cause**: [why this violation exists — 1-2 sentences]

**Fix**: [concrete change — what type/accessor to add where, what codegen code gets replaced]

**Simplifies**: [what downstream code becomes unnecessary once fixed]
```

**Impact criteria:**
- **critical** — wrong phase owns a decision; fix requires cross-crate changes; multiple downstream consumers affected
- **warning** — data flows suboptimally; fix is contained; 2-3 occurrences
- **suggestion** — minor; single occurrence; fix is small

In chat, output summary only:
- Total findings by impact
- Top 3 titles
- "Full report saved to BOUNDARY_REVIEW.md"

## Rules

- **Read-only.** Do not modify source files.
- **No performance findings.** This review is about architecture, not speed.
- **No style findings.** Naming, formatting, comment quality are out of scope.
- **No codegen-internal duplication.** Similar match arms in codegen is a simplification opportunity, not a boundary violation. (That's `/review-simplify`.)
- **Verified findings only.** Every finding must pass Step 3. Unverified findings are worse than no findings.
- **Group by root cause.** 5 occurrences of one problem = 1 finding, not 5.
