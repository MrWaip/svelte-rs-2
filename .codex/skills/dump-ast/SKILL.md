---
name: dump-ast
description: Parse JS code through OXC and inspect the ESTree-like JSON AST. Use proactively when implementing builders, transforms, or codegen for a JS construct, before adding new builder logic for unfamiliar syntax, or when debugging AST-shape mismatches.
---

# Dump OXC AST

Run:

```bash
just dump-ast '<js-code>'
```

Show the JSON output directly.

If parsing fails:
- wrap the expression in parentheses when OXC expects expression context
- turn the input into module-level code when it is really a declaration or statement

Use this before guessing how OXC represents destructuring, spread, optional chaining, decorators, or other syntax that is easy to misremember.
