---
name: dump-ast
description: Parse JS code through OXC and display ESTree JSON AST. Use proactively when implementing codegen, transforms, or debugging to understand how OXC represents a specific JS/TS construct in its AST. MUST use this skill before writing any new AST node construction in builder.rs, before implementing a new codegen visitor, or when unsure how OXC represents a JS pattern (destructuring, spread, optional chaining, etc.). Also use when debugging AST mismatch issues or when porting a new Svelte feature that involves JS expression handling.
argument-hint: "[js-code]"
allowed-tools: Bash
---

# Dump OXC AST: $ARGUMENTS

Parses JavaScript through OXC and displays ESTree-compatible JSON AST.

## Step 1: Parse

```
just dump-ast '$ARGUMENTS'
```

## Step 2: Display

Show the JSON output.

If parsing fails:
- Wrap in parentheses if OXC expects a statement: `($ARGUMENTS)`
- Try as module-level code if it's a declaration

## When to use proactively

Use this skill whenever you need to understand how OXC represents a specific JS construct — for example when implementing codegen, writing transforms, or debugging parser output mismatches.
