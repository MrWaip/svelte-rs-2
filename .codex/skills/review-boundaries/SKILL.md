---
name: review-boundaries
description: Review workflow for compiler phase-boundary and data-flow violations across parser, analyze, transform, and codegen. Use when auditing architecture hygiene, after feature ports, or when code likely drifted into the wrong layer. Do not trigger for style-only cleanup or behavior-preserving simplification reviews.
---

# Boundary Review

## 1) Define scope

Default scope is `svelte_codegen_client` plus `svelte_transform`. Narrow to one crate or one file when the user specifies it.

## 2) Gather context before judging

Read:

- `CLAUDE.md`
- `AGENTS.md`
- `CODEBASE_MAP.md`
- `crates/svelte_analyze/src/data.rs` when you need to know what accessors already exist
- `crates/svelte_codegen_client/src/context.rs` when reviewing codegen shortcuts

Do not report a missing accessor until you verify that it truly does not exist.

## 3) Look for these violation classes

1. re-parsing in downstream phases
2. string-based classification that should use analysis data or `SymbolId`
3. AST re-traversal in codegen/transform to collect derived facts
4. downstream recombination of multiple analysis flags into a hidden decision
5. late rediscovery of facts that an upstream phase already knew
6. misleading types that encode impossible or ambiguous states

## 4) Verify each candidate finding

Before reporting a finding:

- check existing accessors and helper methods
- check how many consumers repeat the pattern
- check whether the pattern is intentional and justified

Drop unverified findings.

## 5) Aggregate by root cause

Group repeated occurrences into one finding. Rank by downstream impact, not by line count.

## 6) Report concretely

For each finding include:

- class
- impact
- occurrences
- evidence with file references
- what you verified
- root cause
- upstream-first fix strategy
- what downstream code would simplify

Saving a structured note such as `BOUNDARY_REVIEW.md` is appropriate for larger audits.

Focus on ownership and data flow, not formatting, performance micro-tuning, or naming style.
