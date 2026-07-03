# Narsil MCP — search protocol

narsil-mcp is the primary code-search tool for this repo. Prefer it over `grep`/`rg`.

## Core rule

**An empty result is not a failure and not a disconnect.** The server responds
instantly and the index is complete (check with `get_index_status`). Empty output
means the *query hit the wrong tool or was phrased wrong* — escalate within narsil,
do not fall back to `grep` on the first miss.

`reindex` is **not** the fix for empty results. Only reindex when the code changed
on disk and results are genuinely stale.

## Tool selection

Pick by what you know, not by habit:

| You have… | Use | Notes |
|---|---|---|
| Multi-word / prose query ("how are store subs reclassified") | `semantic_search` | BM25 + code-aware tokenization. Handles natural language. |
| An exact symbol name (`store_candidate_refs`) | `find_symbols` (pattern) or `search_code` (single token) | `search_code` is a strict keyword match. |
| A symbol whose definition you want | `get_symbol_definition` / `go_to_definition` | Returns source with line numbers. |
| A symbol's call sites | `find_references` / `find_symbol_usages` | |
| Structure overview | `get_project_structure` / `get_export_map` | |

**`search_code` ≠ `semantic_search`.** With `Neural embeddings: disabled`,
`search_code` degrades to a strict keyword match: a multi-word prose query needs
almost all tokens to co-occur, so it returns 0. `semantic_search` is the BM25 tool
that actually answers prose. (Verified: the same prose query gives `search_code` → 0,
`semantic_search` → 5.)

## Escalation ladder (do this before touching grep)

1. Prose query returned 0 → retry with `semantic_search`, not `search_code`.
2. Still 0 → drop `file_pattern` (esp. `original/**`, which matched nothing) and
   simplify the query to the single most distinctive identifier.
3. Looking for a definition/usages → switch to `find_symbols` /
   `get_symbol_definition` / `find_references` — these key off the symbol table,
   not full-text.
4. Only after the ladder is exhausted, use `grep`/`rg` — and say why narsil could
   not answer.

## Quick checks

- `get_index_status` — confirm the index is `✓ Complete` and see enabled features.
- File paths from results are real; open them with `get_file` / `get_excerpt` or Read.
