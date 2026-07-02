---
name: add-test
description: Use whenever adding an e2e compiler-output test (our JS vs the reference compiler) — a new cluster case under `tasks/compiler_tests/`.
---

# Add Compiler Test

Single sanctioned path for an e2e output-comparison case. Adding such a test by hand —
creating the dir, writing `case.svelte`, or registering `compiler_case!` directly — is
not allowed; go through these steps.

Case dir: `cluster_cases/<cluster>/<case>/`. Holds `case.svelte`, generated
`case-svelte.js` + `case-rust.js` (never hand-edit), optional `config.json`.

## Two files, two mechanisms — never conflate

The expected and actual snapshots come from **different commands at different times** — the
#1 mistake is conflating them:

| File | Written by | When |
|---|---|---|
| `case-svelte.js` (expected, reference compiler) | `just generate` | step 5 |
| `case-rust.js` (actual, our compiler) | the **test run** materializes it (`assert_compiler`, `harness.rs`) | step 7, only if the test actually runs |

So `case-rust.js` is materialized only once a test **runs**: `just generate` never writes
it, and an unregistered or `#[ignore]`d case never runs — until then there is no actual
snapshot to compare. This one fact drives steps 5–7 and Red vs ignore below.

## Steps

1. **Pick path & cluster.** Existing cluster (`runes`, `legacy`, `each`, `events`,
   `stores`, `await`, `const_tag`, `snippets`, `text_reactivity`, `component_props`, …);
   new one only if none fit. Stop if the dir exists (extend only if same feature and
   `case.svelte` stays small).

2. **Name the case (see Naming below).** `<case>` snake_case, describing the
   behavior/branch. Must-not-change branches end in `_guard`. No ad-hoc prefixes.

3. **Write `case.svelte`** — smallest component for one feature/edge. Reactive assertions
   must mutate the `$state` (unmutated → const-folds → proves nothing); add a `_guard`
   case for the must-not-change branch.

4. **`config.json`** if not default runes. Legacy: `{ "runes": false }`. Mode MUST match
   the feature. Keys (`src/cases.rs`): `runes`, `dev`, `name`, `filename`, `namespace`,
   `customElement`, `rootDir`, `preserveComments`, `preserveWhitespace`, `experimental.async`.

5. `just generate` — writes `case-svelte.js` (the reference snapshot) only.

6. **Register** in `clusters/<cluster>.rs` — this is the step `just generate` does NOT do:
   `compiler_case!(<fn_name>, "<cluster>/<case>");`
   Expected-red divergence you are NOT fixing now: add `, ignore = "<reason/tag>"` (see
   Red vs ignore below).
   New cluster → create `clusters/<cluster>.rs` with `use super::*;` and add
   `#[path = "clusters/<cluster>.rs"] mod <cluster>;` to `test_clusters.rs`.

7. `just test-cluster <fn_name>` — runs the test (incl. `#[ignore]`, via
   `--include-ignored`), writing `case-rust.js`. Red is normal test-first. Never edit
   snapshots to pass.

## Naming

- `<case>` is snake_case and names the **behavior or Original branch** it pins, never an
  ordinal (`case1`, `t_…`) and never an ad-hoc prefix (`g_…` is banned — it is not a
  convention, it appeared once by mistake).
- A **green-guard** case (a must-not-change branch that must stay GREEN) ends in
  **`_guard`** — the one sanctioned marker. This is the codebase convention (`muted_guard`,
  `elem_unquoted_guard`, …).
- Keep names parallel inside a cluster so the fork-set reads at a glance:
  `elem_reactive` / `elem_unquoted_guard` / `elem_concat_text_guard`.

## Structure

- Each case is its **own directory** under `cluster_cases/<cluster>/<case>/` — never
  several cases sharing one dir via filename tricks.
- Nest (`cluster_cases/<cluster>/<group>/<case>/`) only when a feature has real sub-axes;
  the string passed to `compiler_case!` mirrors the directory path exactly.
- The cluster file is `clusters/<first-path-segment>.rs` (e.g. cases under
  `cluster_cases/attribute/single_expr/…` register in `clusters/attribute_single_expr.rs`).

## Red vs ignore

An `#[ignore]`d case never runs, so it never materializes `case-rust.js` and never checks
parity (per the table above). The trap: `just test-compiler` is `cargo test` with **no**
`--include-ignored`, so it **skips** every ignored case; only `just test-cluster <fn>` /
`just test-case <fn>` pass `--include-ignored` and actually execute them.

- **Test-first, fixing now** → register WITHOUT `ignore`. The case is red; iterate with
  `just test-cluster <fn>` (includes ignored, materializes `case-rust.js`). When green it
  joins the suite.
- **Known divergence, not fixing now** → `ignore = "<reason>"` so the green suite
  (`just test-compiler`) stays green — the case stays inert until you remove `ignore`.
