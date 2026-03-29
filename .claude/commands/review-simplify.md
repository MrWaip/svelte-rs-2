---
description: Find simplification opportunities across compiler layers. Use when code feels overly complex, after multiple features were ported, or when a module is hard to read.
argument-hint: "[crate-or-path | empty=codegen+transform]"
allowed-tools: Bash, Read, Grep, Glob, Agent, LSP, Write
---

# Simplification Review: $ARGUMENTS

Find places where code can be made simpler without changing behavior. Change HOW the code works, never WHAT it does.

## Scope

Determined by `$ARGUMENTS`:
- **No argument** → `svelte_codegen_client` + `svelte_transform`
- **Crate name** (e.g. `codegen`, `analyze`, `parser`) → that crate only
- **File path** → that file only

## Step 1: Gather context

Read before reviewing:
1. `CODEBASE_MAP.md` — module structure, existing helpers and abstractions
2. `crates/svelte_analyze/src/data.rs` — what AnalysisData accessors exist
3. `crates/svelte_codegen_client/src/context.rs` — what Ctx shortcuts exist

## Step 2: Cross-layer scan (mandatory)

**This step is mandatory and cannot be skipped.** It runs before within-crate review.

For each file in scope, collect every `if/else if/else` chain, `match` with 3+ arms, and multi-accessor call (`ctx.analysis.foo() && ctx.analysis.bar()`). For each:

1. **Count accessors** — how many analyze/scoping calls does this decision require?
2. **Check repetition** — does the same decision pattern appear in other codegen/transform files? (grep for the accessor names)
3. **Evaluate** — could analyze expose a single accessor/enum that replaces this multi-step decision?

Record your findings. If zero cross-layer opportunities exist, you must show evidence: list the decisions you checked and why each is correctly in codegen (e.g. "single flag read", "depends on post-transform value", "output formatting choice").

**"Looks well-structured" without listing what you checked is not acceptable.**

### Level 1 signals

**Parser → Analyze:**
- Parser delivers raw spans where structured AST would eliminate analyze re-work
- Parser doesn't pre-parse JS expressions that analyze needs to classify

**Analyze → Transform/Codegen:**
- Analyze stores raw facts where a pre-computed enum/accessor would eliminate downstream branching
- Analyze answers exist but codegen doesn't use them (builds its own logic instead)
- Multiple codegen files repeat the same multi-flag decision — analyze should own it

**IS an opportunity:** Three codegen files each check `is_dynamic && has_call && !is_simple` to decide memoization — analyze should expose `needs_memo(id) -> bool`.
**NOT an opportunity:** Codegen checks one analyze flag to pick between two output formats — that's codegen's job.

## Step 3: Within-crate and cross-file review

### Level 2: Within-crate simplification (from Step 3)

**Signals:**
- **Duplicated logic** — same pattern in 2+ functions/branches that could be unified. Only flag if unification doesn't require adding parameters that obscure intent.
- **Unnecessary abstraction** — type/trait/helper exists for one call site. Inlining would be clearer.
- **Dead weight** — parameter always receives same value, enum variant never handled differently from default, match arm identical to another.
- **Deep nesting** — 3+ levels of if/match/for. Could be flattened with early returns or extracted match.

### Level 3: Cross-file patterns within a crate

Look for the same conceptual operation done in different ways across files in the same crate.

**Signals:**
- Two files handle the same attribute/node type with similar but slightly different logic
- A helper exists in one file that another file would benefit from
- The same visitor pattern is implemented independently in multiple places

**IS an opportunity:** `element.rs` and `component.rs` both build attribute props with the same dynamic/static branching — could share a helper.
**NOT an opportunity:** Two files handle different node types with genuinely different logic that happens to look similar structurally.

## Step 4: Balance check

For each potential finding, verify it passes the balance test:

1. **Does unification actually simplify?** If the unified version needs 3+ parameters or a complex generic to handle all cases — the duplication is cheaper. Skip.
2. **Does the change preserve readability?** Fewer lines ≠ simpler. A 10-line match with clear branches is simpler than a 5-line generic helper you need to read twice. Skip if readability decreases.
3. **Is this over-engineering?** Creating an abstraction for 2 occurrences that may diverge later is premature. Flag only if 3+ occurrences or the pattern is clearly stable.
4. **Would this create a god-function?** Combining too many concerns into one function/helper makes the code harder to understand. Skip.

## Step 5: Report

Save to `SIMPLIFY_REVIEW.md` in project root (overwrite if exists).

Format per finding:

```
### #N — [One-line title]

**Level**: [1: cross-layer | 2: within-crate | 3: cross-file]
**Impact**: high | medium | low
**Files**: [list of affected files]

**Current state**: [what the code does now — quote the patterns]

**Proposed change**: [concrete — what to extract/inline/unify and where]

**Balance**: [why this simplification is worth it — what becomes easier to read/maintain]
```

**Impact criteria:**
- **high** — removes significant branching or duplication; makes a module noticeably easier to read top-to-bottom; Level 1 cross-layer improvements
- **medium** — cleaner but localized; removes a repeated pattern in 2-3 places
- **low** — cosmetic; single occurrence; marginal readability gain

In chat, output summary only:
- Total findings by level and impact
- Top 3 titles
- "Full report saved to SIMPLIFY_REVIEW.md"

## Rules

- **Read-only.** Do not modify source files.
- **Preserve functionality.** Change HOW, never WHAT. Every simplification must produce identical behavior.
- **No boundary violations.** If the problem is "code in wrong phase" — that's `/review-boundaries`. Here we ask "can upstream do better so downstream is simpler", which is an opportunity, not a violation.
- **No style-only findings.** Renaming, comment rewording, import reordering are out of scope.
- **Balance over brevity.** Prefer explicit readable code over clever compact solutions. If in doubt, skip the finding.
- **Cross-layer first.** Level 1 findings are the most valuable. Prioritize them.
