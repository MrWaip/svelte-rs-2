---
name: quick-check
description: Use when you have an ad-hoc Svelte component or `.svelte.js`/`.svelte.ts` module (pasted inline, scratch file, or a single existing file) and need a one-shot parity check against `svelte/compiler` without registering a persistent case.
argument-hint: "<path-or-inline-source> [--mode=auto|runes|legacy] [--generate=client|server] [--dev] [--async] [--filename=<name>] [--print=diff|ours|ref|both]"
allowed-tools: Bash, Write
---

# Quick-check component or module

One-shot compile of one component (compares JS + whitespace-normalized CSS) or `.svelte.js`/`.svelte.ts` module (JS only) against the reference compiler. Dispatch is by extension: `.svelte.js`/`.svelte.ts` → `compile_module`, else `compile`. Does NOT register tests or touch compiler code.

**Not for:** persisted finding → `/dig-better`; directory survey → `/sweep`; permanent case → `/add-test` or `/add-diagnostic-test`; about to fix a crate → `/dig-better` first.

## Run

```bash
just quick-check <path> [flags]                  # existing file
printf '%s' "$SRC" | just quick-check - [flags]  # inline source via stdin — no repo files
```

Prefer stdin for inline source: `-` reads stdin, the reference side gets an auto-removed temp file in `/tmp`, nothing lands in the repo. Type is inferred from `--filename` extension (default `.svelte`); for modules add `--filename=x.svelte.js` or `x.svelte.ts`. Use `cat <<'EOF'` so `$state`/backticks survive shell quoting. Only fall back to git-ignored scratch (`tasks/quick_check/scratch.svelte{,.js,.ts}`) when re-running while hand-editing; never write to `tasks/compiler_tests/cases2/`.

Flags (forwarded to both sides; pass through whatever the user named):

- `--mode=auto|runes|legacy` — default runes unless `<svelte:options runes={false}/>`. Ignored for modules.
- `--generate=client|server` — default client.
- `--dev` — dev build.
- `--async` — `experimental.async` on both sides. Required for `await` in the instance script or a template expression: without it both sides error as `experimental_async`. Modules ignore it.
- `--filename=<name>` — component-name resolution / module filename.
- `--print=diff|ours|ref|both` — default `diff` (interleaved `<` ours / `>` ref). `both` prints full outputs even on match; `ours`/`ref` print one side. Exit code unchanged.

## Exit codes

- `0` OK — JS + CSS match. Report flags applied + JS line / CSS byte counts.
- `1` MISMATCH — diff under `-- JS --` / `-- CSS --`; summarize divergence categories, call out first mismatching block.
- `2` INPUT — missing file / bad args.
- `3` PANIC — Rust panicked or no JS; name the message, guess owning layer (parser/analyze/transform/codegen).
- `4` NODE — reference failed; likely invalid syntax or missing npm deps.

Warn and skip SvelteKit/MDsveX-specific input (unresolvable imports/types). Standalone runes-in-module code is fine even with unresolved external imports.
