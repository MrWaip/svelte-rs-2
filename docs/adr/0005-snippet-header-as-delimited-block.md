# 5. Snippet header is a delimited `{…}` block, not a bespoke signature parse

Status: accepted (2026-07-23)

## Context

`{#snippet name<T>(params)}` is a fixed signature (identifier, optional type parameters,
optional params, `}`), not a free JS expression like `{#if expr}`. The Original hand-parses
its boundary — a pointy-bracket matcher for the generics, a paren-counting loop for the
params, then require `}`. Our scanner already has one boundary primitive every other `{…}`
tag uses: `collect_js_until_brace`.

## Decision

Scan the snippet header as a delimited block: read the identifier, then `collect_js_until_brace`
to the closing `}`; OXC parses the captured `name<T>(params)` downstream. No bespoke
bracket-counting.

Type parameters need no scanner handling (`<`/`>` are ordinary characters) and are erased
downstream — `parse_snippet_decl_with_alloc` wraps the header as `const name = (params) => {}`,
dropping `<…>`. The snippet name comes from the parsed binding-identifier `symbol_id`, never
from slicing the header source.

## Consequences

- One boundary mechanism for every `{…}` tag; no hand-rolled JS bracket-matching in the scanner.
- **Malformed headers diverge from the Original.** A missing `}` lets `collect_js_until_brace`
  swallow `{/snippet}` and error at EOF instead of the Original's exact offset. The Original's
  `compiler-errors/malformed-snippet{,-2}` samples are dropped (renamed `*.svelte.skip`). We do
  not pursue byte-exact error positions on malformed snippet syntax.
- A malformed header leaves the `decl` statement unbound — a legitimate state: `StmtRef::id()`
  returns `DUMMY`, `JsAst::stmt` returns `None`, consumers skip the snippet. No panic.

## Alternatives

- **Port the Original's header parser.** Byte-exact including malformed positions; rejected —
  reintroduces hand-rolled JS bracket-matching for the sole gain of error offsets we don't value.
- **`scan_js_pattern(MatchingParen)` for the params.** Rejected — the full JS scanner misreads
  `/snippet}` in an unclosed `(` as a regex literal, giving another wrong position.
