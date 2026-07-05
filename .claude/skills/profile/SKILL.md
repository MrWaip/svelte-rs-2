---
name: profile
description: Compiler profiling and hotspot analysis.
argument-hint: "[fresh]"
allowed-tools: Bash, Read, Grep, Glob, mcp__narsil-mcp__*
disable-model-invocation: true
---

# `./profile/` layout

`./profile/all/` — all cases × prod/dev (`just bench-flame-all`):
- `aggregate.top.txt` — aggregated top across all cases
- `<slug>_<mode>/top.txt` — top of a single case
- `<slug>_<mode>/profile.folded` — folded stacks (can be up to 6 MB; do NOT read whole, only `Grep` by function name)
- `<slug>_<mode>/profile.json`, `profile.json.syms.json` — raw dumps, do NOT read whole; work through `jq`/`grep` surgically

`./profile/one/` — a single case (`just bench-flame <path> [--dev]`):
- same artifacts as in `<slug>_<mode>/` above

`slug` = path from `tasks/benchmark/benches/compiler/` with `/` → `_`, without `.svelte`/`.svelte.js`.
`mode` = `prod` | `dev`.

## Step 1

If `$ARGUMENTS == "fresh"` — run `just bench-flame-all`. Tell the user the run takes ~4 min and start immediately, without asking for confirmation. After the run — read `./profile/all/aggregate.top.txt`.

Otherwise — read the existing `./profile/all/aggregate.top.txt`. If the file is missing — tell the user to run `/profile fresh` or `just bench-flame-all` manually and stop.

## Step 2

Analyze `./profile/all/`. Work through the top-5 hotspots from `aggregate.top.txt` — every one, not just the first.
For each, find the driver-case: compare `<slug>_<mode>/top.txt` across subdirectories and pick where this function's self-% is highest.

## Step 3

Report on the hotspots: what's hot, why, and what to do about it.

## Later in the session

When the user picks an option — dig into the driver-case of that hotspot and propose concrete changes.

To profile a single case in isolation — `just bench-flame <path> [--dev]` (artifacts in `./profile/one/`, see above).
