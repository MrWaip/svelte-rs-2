---
name: dump-ast
description: Dumps the ESTree JSON AST that OXC produces for a JS/TS snippet. Use before constructing a new AST node in builder.rs or a codegen visitor, when unsure how OXC represents a JS construct, or when debugging an AST mismatch.
argument-hint: "[js-code]"
allowed-tools: Bash
---

# Dump OXC AST: $ARGUMENTS

Parses JavaScript through OXC and displays the ESTree-compatible JSON AST.

```
just dump-ast '$ARGUMENTS'
```

Show the JSON output. If parsing fails:

- wrap in parentheses if OXC expects a statement: `($ARGUMENTS)`
- try as module-level code if it's a declaration
