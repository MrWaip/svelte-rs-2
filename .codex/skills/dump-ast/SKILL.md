---
name: dump-ast
description: Dump JavaScript or TypeScript through OXC to inspect the ESTree-like AST. Use when implementing builders, transforms, or codegen for a JS construct, when unsure how OXC represents a syntax form, or when debugging AST-shape mismatches.
---

# Dump OXC AST

Run:

```bash
just dump-ast '<js-code>'
```

Show the JSON output directly.

If parsing fails:

- try wrapping the expression in parentheses
- try turning the input into module-level code when it is really a declaration

Use this proactively before adding new builder logic or traversal for unfamiliar JS syntax.
