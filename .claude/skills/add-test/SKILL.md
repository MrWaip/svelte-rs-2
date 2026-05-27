---
name: add-test
description: Use whenever adding an e2e compiler-output test (our JS vs the reference compiler) — a new cluster case under `tasks/compiler_tests/`.
---

# Add Compiler Test

Single sanctioned path for an e2e output-comparison case. Adding such a test by hand —
creating the dir, writing `case.svelte`, or registering `compiler_case!` directly — is
not allowed; go through these steps.

Case dir: `cluster_cases/<cluster>/<case>/` (`<case>` may nest). Holds `case.svelte`,
generated `case-svelte.js` + `case-rust.js` (never hand-edit), optional `config.json`.

## Steps

1. **Pick path.** Existing cluster (`runes`, `legacy`, `each`, `events`, `stores`,
   `await`, `const_tag`, `snippets`, `text_reactivity`, `component_props`, …); new one
   only if none fit. `<case>` snake_case. Stop if the dir exists (extend only if same
   feature and `case.svelte` stays small).

2. **Write `case.svelte`** — smallest component for one feature/edge. Reactive assertions
   must mutate the `$state` (unmutated → const-folds → proves nothing); add a green-guard
   case for the must-not-change branch.

3. **`config.json`** if not default runes. Legacy: `{ "runes": false }`. Mode MUST match
   the feature. Keys (`src/cases.rs`): `runes`, `dev`, `name`, `filename`, `namespace`,
   `customElement`, `rootDir`, `preserveComments`, `preserveWhitespace`, `experimental.async`.

4. `just generate` — writes `case-svelte.js` from the reference compiler.

5. **Register** in `clusters/<cluster>.rs`:
   `compiler_case!(<fn_name>, "<cluster>/<case>");`
   Expected-red: add `, ignore = "<reason/tag>"`.
   New cluster → create `clusters/<cluster>.rs` with `use super::*;` and add
   `#[path = "clusters/<cluster>.rs"] mod <cluster>;` to `test_clusters.rs`.

6. `just test-cluster <fn_name>` — red is normal test-first. Never edit snapshots to pass.
