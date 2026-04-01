---
name: review-simplify
description: Behavior-preserving simplification review workflow across parser, analyze, transform, and codegen. Use when code became hard to read, branching is repetitive, or a module likely has simplification opportunities. Do not trigger for phase-ownership violations; use `review-boundaries` for those.
---

# Simplify Review

## 1) Define scope

Default scope is codegen plus transform unless the user points to a crate or path.

## 2) Do a mandatory cross-layer scan first

Collect repeated multi-accessor decisions, large branching logic, and places where upstream data could simplify downstream code.

If there are no Level 1 cross-layer opportunities, say what you checked and why each case belongs where it is.

## 3) Review within-crate and cross-file patterns

Look for:

- duplicated logic
- unnecessary abstractions
- dead weight
- deep nesting
- similar operations implemented differently in the same crate

## 4) Apply a balance check

Only keep findings that improve readability without creating generic plumbing, over-abstraction, or god-functions.

## 5) Report

Write a ranked report such as `SIMPLIFY_REVIEW.md` with level, impact, current state, proposed change, and why the simplification is worth it.

Rules:
- preserve behavior exactly
- prefer explicit readable code over clever abstractions
- require a balance check before keeping a finding
