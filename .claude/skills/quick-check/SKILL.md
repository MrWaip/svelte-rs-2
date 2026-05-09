---
name: quick-check
description: Fast parity probe for a single Svelte component against the reference `svelte/compiler`. Use when the user pastes an ad-hoc Svelte component or points at a `.svelte` file and wants to see whether our compiler panics or produces matching JS — without registering a persistent compiler test case. No changes to compiler code or persistent tests.
argument-hint: "<path-or-inline-source> [--mode=auto|runes|legacy] [--generate=client|server] [--dev] [--filename=<name>]"
allowed-tools: Bash, Write
---

# Quick-check component

One-shot compile of a single Svelte component against the reference compiler. Compares both the emitted JS and the scoped CSS (CSS is whitespace-normalized via `compact_css_for_injection`, so indentation diffs do not trip parity). Prints `OK` on parity or a per-section diff on mismatch. Does NOT register persistent tests or modify compiler code.

## Step 1: Resolve the input

- Argument is an existing `.svelte` path → use as-is.
- Argument is inline Svelte source → write to a scratch file first:
  - Path: `tasks/quick_check/scratch.svelte` (workspace-local, safe to overwrite).

Never write to `tasks/compiler_tests/cases2/`.

## Step 2: Run

```bash
just quick-check <path> [flags]
```

Flags (all optional, both sides — our compiler and reference — receive them):

- `--mode=auto|runes|legacy` — compile mode. Default: our compiler's default (`runes` unless `<svelte:options runes={false}/>` flips it).
- `--generate=client|server` — runtime target. Default: `client`.
- `--dev` — dev-build flag.
- `--filename=<name>` — override component filename (affects component-name resolution).

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
- Reusable scratch file only: `tasks/quick_check/scratch.svelte`.
- Do NOT commit the scratch file — it is git-ignored.
- If inline source appears to be SvelteKit / TypeScript / MDsveX specific, warn and skip.
