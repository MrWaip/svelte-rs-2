---
name: port
description: Spec-driven feature port workflow for implementing Svelte compiler behavior in this repo. Use when the user asks to port or implement a feature, continue work from `specs/<feature>.md`, or add support for a missing Svelte construct across parser, analyze, transform, and codegen.
---

# Port Feature

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

## 1) Resume from spec when possible

If the input is a spec path or a matching spec exists, read it first and continue from `Current state` instead of replanning.

## 2) If no spec exists, plan first

Use the reference compiler to learn expected behavior, not to copy implementation shape.

Build:

- a use-case checklist
- an implementation plan by layer
- a spec file using `spec-template`

The plan text must include: **"Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries."**

Defer out-of-scope cases explicitly instead of silently dropping them.
When a case is deferred, add a dedicated unchecked checkbox for it in the matching spec `Use cases` deferred subsection so later sessions can see it in feature-local context.
If no matching spec exists yet, tell the user explicitly that the deferred case was not recorded because there is no corresponding spec.

## 3) Add test cases before or alongside implementation

Create one focused compiler case per selected use case when needed. Generate expected JS once with `just generate` and never hand-edit generated outputs.

## 4) Implement in layer order

Typical order:

1. AST and parser
2. analyze side tables or accessors
3. transform if required
4. client codegen

Cross-check architecture with `AGENTS.md`, `CLAUDE.md`, `phase-boundaries`, and `svelte-reference-map`.

## 5) Verify progressively

Run each affected test case individually, then `just test-compiler`. Widen to `just test-all` when the feature affects shared infrastructure.

## 6) Update tracking

Update:

- spec `Current state`
- completed use cases and tasks
- `ROADMAP.md` if the feature is now actually done
- deferred items as unchecked checkboxes in the spec if new scope edges were discovered
