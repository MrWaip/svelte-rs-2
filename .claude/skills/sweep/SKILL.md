---
name: sweep
description: Sweep a directory of Svelte files for parity divergences against svelte/compiler. Use when parity-checking a whole folder at once, or when another skill needs sweep usage.
---

# Sweep a directory for parity divergences

A sweep compiles every `.svelte` / `.svelte.js` / `.svelte.ts` under a directory with both our compiler and `svelte/compiler`, then buckets the mismatches. It is the directory-scale sibling of `/quick-check`: same reference and same normalizers, many files at once, in parallel.

```bash
just sweep-run <dir> [--mode=auto|runes|legacy] [--async] [--chunk=N] [--print-diffs] [--out=<file>]
```

Needs `node_modules` (run `npm install` if absent) — the reference side is `svelte/compiler`.

## What it always does

- **Both generate targets, both build modes** — every file is compared across `client` and `server`, each in prod and dev (4 cells). No flag toggles this; each finding is tagged with the cell that diverged: `[client]`, `[client-dev]`, `[server]`, `[server-dev]`.
- **Dry-run** — one run reports the whole tree; it never halts at the first mismatch.
- **No per-case config** — a sweep ignores any `config.json` beside a file; the mode comes only from `--mode` (default `auto`, i.e. svelte auto-detects runes vs legacy).

## Flags

- `--mode=auto|runes|legacy` — force the runes verdict on both sides. Default `auto`.
- `--async` — `experimental.async` on both sides. Off by default, so an `await` component errors on both sides and the file reports as **matched** — its output divergence stays invisible. Pass it to sweep an async tree. Components only.
- `--chunk=N` — files per reference (node) batch, run in parallel. Default 300. Lower it if node runs out of memory on a huge tree.
- `--print-diffs` — append a unified `reference` vs `rust` diff per `js`/`css` mismatch.
- `--out=<file>` — write the full detail (buckets + diffs) to a file; the terminal keeps only the summary table.

## Reading the output

The **summary table** at the bottom is the tail you read first — one row per reason, `files` (distinct files hit) and `hits` (file × cell occurrences):

```
════════╤═══════╤═══════
 reason │ files │  hits
────────┼───────┼───────
 js     │     3 │     5
════════╧═══════╧═══════
OK:       1401/1404 files matched
MISMATCH: 3/1404 files
```

Above it, the **detailed buckets** list the paths per reason. Reasons, in report order:

- `rust-error` — our compiler errored/panicked, reference succeeded.
- `reference-error` — reference errored, ours succeeded.
- `errors-differ` — both errored, codes disagree.
- `diagnostics-mismatch` — error-severity warning codes differ.
- `js` / `css` — output diverges. These carry a size (changed lines), subgrouped `xs`/`s`/`m`/`l`/`xl` — biggest first, so the `xl` at the top of the bucket is the meatiest divergence.
- `read-error` / `format-error` / `reference-missing` — the file couldn't be read, TS-stripped, or came back absent from the reference batch.

Exit code: `0` all match, `1` any mismatch, `2` bad args / not a directory / no files / missing deps, `4` reference compiler failed.

## Trust the verdict, then narrow

A sweep uses the **same normalizers as the harness** (`test_support`), so a sweep-green file is a harness-green file. It is *stricter* than the harness in one way: it compares all 4 cells even when a registered case gates some out via its config — so a sweep can surface a real `[server]`-only or dev-only divergence that `just test-compiler` never asserts for that case. That is a genuine finding, not noise.

From a sweep result, narrow to one file before fixing:

- reproduce it alone with `/quick-check <file>` — add `--generate=server` for a `[server]`/`[server-dev]` tag, `--dev` for a dev cell, `--async` if the sweep used it.
- hand a confirmed, persistent divergence to `/dig-better` (it owns cluster-picking), or a permanent case to `/add-test` / `/add-diagnostic-test`.

## Not for

- One file or inline source → `/quick-check`, no sweep needed.
- A build-artifact clean-up (`cargo sweep`) — unrelated; that is `just sweep-2d`.
