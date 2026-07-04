---
name: add-test
description: Use whenever adding an e2e compiler-output test (our JS vs the reference compiler) — a new cluster case under `tasks/compiler_tests/`.
---

# Add Compiler Test

Single sanctioned path for an e2e output-comparison case. Adding such a test by hand —
creating the dir, writing `case.svelte`, or registering `compiler_case!` directly — is
not allowed; go through these steps.

Case dir: `cluster_cases/<cluster>/<case>/`. Holds `case.svelte`, generated
`case-svelte.js` + `case-svelte.dev.js` (expected) and `case-rust.js` + `case-rust.dev.js`
(actual, never hand-edit), optional `config.json`.

## Four snapshots: expected vs actual × prod vs dev — never conflate

Each case pins **both** compile modes, so it has four snapshot files. The #1 mistake is
conflating expected (reference compiler) with actual (ours):

| File | Mode | Written by | When |
|---|---|---|---|
| `case-svelte.js` (expected) | prod | `just generate` | step 5 |
| `case-svelte.dev.js` (expected) | dev | `just generate` | step 5 |
| `case-rust.js` (actual) | prod | the `<fn>::prod` test run (`assert_compiler_prod`, `harness.rs`) | step 7 |
| `case-rust.dev.js` (actual) | dev | the `<fn>::dev` test run (`assert_compiler_dev`, `harness.rs`) | step 7 |

`compiler_case!(<fn>, "<path>")` expands to a module with **two** tests — `<fn>::prod` and
`<fn>::dev`. Each `case-rust*.js` is materialized only once its test **runs**: `just
generate` never writes them, and an unregistered or `#[ignore]`d test never runs — until
then there is no actual snapshot to compare. This drives steps 5–7 and Red vs ignore below.

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

5. `just generate` — writes the reference snapshots `case-svelte.js` + `case-svelte.dev.js`
   only.

6. **Register** in `clusters/<cluster>.rs` — the step `just generate` does NOT do:
   `compiler_case!(<fn_name>, "<cluster>/<case>");`. Divergence you are NOT fixing now → see
   Red vs ignore below.
   New cluster → create `clusters/<cluster>.rs` with `use super::*;` and add
   `#[path = "clusters/<cluster>.rs"] mod <cluster>;` to `test_clusters.rs`.

7. `just test-cluster <fn_name>` — runs both `::prod` and `::dev` (incl. `#[ignore]`, via
   `--include-ignored`), writing `case-rust.js` + `case-rust.dev.js`. Red is normal
   test-first. Never edit snapshots to pass.

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

An `#[ignore]`d test never runs, so it never materializes its `case-rust*.js` and never
checks parity (per the table above). The trap: `just test-compiler` runs `cargo nextest
run`, which **skips** `#[ignore]`d tests by default, so ignored cases don't execute there;
only `just test-cluster <fn>` / `just test-case <fn>` (plain `cargo test … --
--include-ignored`) run them. The `<fn>` filter is a substring, so it matches both
`<fn>::prod` and `<fn>::dev`.

- **Test-first, fixing now** → register WITHOUT `ignore`. The case is red; iterate with
  `just test-cluster <fn>` (includes ignored, materializes `case-rust.js` +
  `case-rust.dev.js`). When both modes are green it joins the suite.
- **Both modes diverge, not fixing now** → `ignore = "<reason>"` — ignores `::prod` and
  `::dev` so the green suite (`just test-compiler`) stays green until you remove `ignore`.
- **Only dev diverges** → `[prod, dev_todo]` keeps `::prod` in the suite and ignores just
  `::dev` (or `[prod]` to drop dev entirely), so prod parity is enforced meanwhile.
