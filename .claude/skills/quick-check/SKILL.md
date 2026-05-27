---
name: quick-check
description: Use when you have an ad-hoc Svelte component or `.svelte.js`/`.svelte.ts` module (pasted inline, scratch file, or a single existing file) and need a one-shot parity check against `svelte/compiler` without registering a persistent case.
argument-hint: "<path-or-inline-source> [--mode=auto|runes|legacy] [--generate=client|server] [--dev] [--filename=<name>] [--print=diff|ours|ref|both]"
allowed-tools: Bash, Write
---

# Quick-check component or module

One-shot compile of a single Svelte component or standalone `.svelte.js` / `.svelte.ts` module against the reference compiler. Components compare both the emitted JS and the scoped CSS (CSS is whitespace-normalized via `compact_css_for_injection`, so indentation diffs do not trip parity). Modules compare JS only (no CSS, no template). Prints `OK` on parity or a per-section diff on mismatch. Does NOT register persistent tests or modify compiler code.

## When NOT to use

- Persisted parity finding (sweep mismatch, registered case) → `/dig`.
- Directory-level survey across many files → `just sweep-run <dir> --dry-run`.
- Capturing the behavior as a permanent test → `/add-test` (compiler) or `/add-diagnostic-test` (diagnostics).
- About to modify compiler crates to fix the divergence → escalate to `/dig` first.

The Rust side dispatches by file extension: `.svelte.js` / `.svelte.ts` → `compile_module`, anything else → `compile`. The reference side does the same.

## Step 1: Resolve the input

- Argument is an existing `.svelte` / `.svelte.js` / `.svelte.ts` path → use as-is.
- Argument is inline source → write to a scratch file first. Pick the extension by content:
  - looks like a component (has markup, `<script>`, `<style>`, or directives) → `tasks/quick_check/scratch.svelte`.
  - looks like a standalone module (top-level `import`/`export`, `$state`/`$derived` at module scope, no markup) → `tasks/quick_check/scratch.svelte.js`.
  - TypeScript module → `tasks/quick_check/scratch.svelte.ts`.

All three scratch files are git-ignored. Never write to `tasks/compiler_tests/cases2/`.

## Step 2: Run

```bash
just quick-check <path> [flags]
```

Flags (all optional, both sides — our compiler and reference — receive them):

- `--mode=auto|runes|legacy` — compile mode. Default: our compiler's default (`runes` unless `<svelte:options runes={false}/>` flips it). Ignored for module inputs (modules are always runes).
- `--generate=client|server` — runtime target. Default: `client`.
- `--dev` — dev-build flag.
- `--filename=<name>` — override filename (affects component-name resolution for components; passed to `compile_module` for modules).
- `--print=diff|ours|ref|both` — output mode. Default `diff` (interleaved `< ours / ref >` comparison, silent body on match). Use `both` to print the **full** rust and reference outputs side-by-side (labeled `==== RUST JS ====` / `==== REFERENCE JS ====`), printed **even when they match** — this is the way to read a fork in context when the interleaved diff is ambiguous (note: `< left` is OURS, `> right` is REFERENCE). `ours` / `ref` print just one side's full output. Exit code is unchanged (0 match / 1 mismatch).

If the user explicitly named a mode, generate target, or dev flag, pass it through — do not assume defaults match.

Under the hood this invokes `cargo run -q -p quick_check -- <path> [flags]` after installing the reference compiler's npm deps. Flags are forwarded to the reference side via `QUICK_CHECK_CONFIG`.

## Step 3: Interpret the exit code

- `0` — our JS and (whitespace-normalized) CSS both match reference. Report `OK` + JS line count + CSS byte count.
- `1` — JS and/or CSS mismatch. Diff printed per section under `-- JS --` / `-- CSS (whitespace-normalized) --` headers. Summarize the categories of divergence (e.g. missing CSS scope class on a bare-pseudo combinator, statement ordering, missing helper call, wrong rune lowering). Call out the first mismatching block in each section.
- `2` — bad input (missing file, bad arguments).
- `3` — our Rust compiler panicked or produced no JS. Name the panic message and guess the first owning layer: parser, analyze, transform, codegen.
- `4` — reference compiler (node side) failed. Likely invalid Svelte syntax or missing npm deps.

## Step 4: Report

Keep report terse. Include:

- exit code summary (OK / MISMATCH / PANIC / INPUT / NODE)
- applied flags (mode/generate/dev/filename) — they appear in the binary's output line; surface them so the user sees what was actually probed
- first owning layer if panic or mismatch is clearly layer-scoped
- suggested next command if the user wants durable coverage:
  - `/diagnose` — turn mismatch into a spec-owned follow-up
  - `/add-test <name>` — capture passing behavior as a persistent case

## Rules

- No edits to compiler crates.
- No edits to `tasks/compiler_tests/cases2/` or `tasks/compiler_tests/test_v3.rs`.
- Reusable scratch files only: `tasks/quick_check/scratch.svelte`, `tasks/quick_check/scratch.svelte.js`, `tasks/quick_check/scratch.svelte.ts`.
- Do NOT commit scratch files — all three are git-ignored.
- If inline source appears to be SvelteKit / MDsveX specific (uses imports/types that won't resolve in isolation), warn and skip. Pure standalone runes-in-module code is fine even with unresolved external imports — `compile_module` doesn't need them to resolve.
