---
name: update-svelte
description: Bump the pinned Svelte version across the whole repo — pin, vendored `original/`, docs, README, playground import map — then regenerate expected outputs and run the checks. Manual launch only via `/update-svelte`.
argument-hint: "[version, e.g. 5.56.0]"
allowed-tools: Bash, Read, Edit, Grep
disable-model-invocation: true
---

# update-svelte

The Svelte version is pinned across the repo and must move as one. Sites are of three kinds:

- **pin/prose** — `package.json`, `package-lock.json`, `README.md`: text occurrences of the version.
- **vendored mirror** — `original/`: a byte-for-byte copy of `node_modules/svelte` (source of truth for parity). `original/compiler/` = `node_modules/svelte/src/compiler/`, `original/package.json` = `node_modules/svelte/package.json`, `original/docs/` = `documentation/docs/` from the GitHub monorepo.
- **playground import map** — `playground/index.html`: a JSPM-generated dependency graph the browser loads straight from the CDN. Independent of `node_modules` — its own pin.

Every other consumer reads Svelte from `node_modules`, driven by the single pin in `package.json` — no separate steps.

Done when the OLD version appears nowhere unintended, `original/` diffs clean against `node_modules/svelte`, the playground resolves `svelte@NEW`, and `just test-compiler`, `just test-diagnostics`, `just lint` are all green.

Work from the repo root.

## Task boundary

This skill does **not** fix new parity divergences the newer Svelte introduces: in step 9 those surface as `just test-compiler` failures. That is separate compiler work, not hand-editing of expected files — list such failures in the report; do not paper over them.

## Steps

### 1. Fix OLD and NEW

- `OLD` — the current `"svelte"` value in `package.json`.
- `NEW` — the invocation argument. If absent, ask the user; `npm view svelte version` gives latest for reference.

Criterion: both versions known and `OLD ≠ NEW`.

### 2. Move the pin and install

- In `package.json` replace `"svelte": "OLD"` → `"svelte": "NEW"` (`dependencies` section, exact pin, no caret).
- `npm install` — updates `package-lock.json` and pulls the new `node_modules/svelte`.

Criterion: `node -p "require('svelte/package.json').version"` prints `NEW`.

### 3. Re-vendor `original/` from the installed package

```
rsync -a --delete node_modules/svelte/src/compiler/ original/compiler/
cp node_modules/svelte/package.json original/package.json
```

Criterion (both diffs empty):

```
diff -rq original/compiler node_modules/svelte/src/compiler
diff -q  original/package.json node_modules/svelte/package.json
```

### 4. Re-vendor `original/docs/` from the GitHub tag

The npm tarball has no docs — pull them from the monorepo at tag `svelte@NEW`.

```
TMP="$(mktemp -d)"
curl -fsSL "https://github.com/sveltejs/svelte/archive/refs/tags/svelte@NEW.tar.gz" | tar xz -C "$TMP"
SRC="$(find "$TMP" -type d -path '*/documentation/docs' | head -1)"
[ -n "$SRC" ] || { echo "docs for svelte@NEW not found — check the tag; skip"; exit 1; }
rsync -a --delete "$SRC/" original/docs/
```

The `[ -n "$SRC" ]` guard is load-bearing: an empty `SRC` turns the next line into `rsync --delete "/" …`, mirroring the filesystem root.

Criterion: `original/docs/index.md` exists and `git status original/docs/` shows the refresh. If the URL 404s, check the tag name at github.com/sveltejs/svelte/tags and retry; if the network is unavailable, report and continue.

### 5. Update prose

In `README.md` replace every `OLD` → `NEW`.

Criterion: `grep -c OLD README.md` → `0`.

### 6. Regenerate the playground import map

`playground/index.html` loads the reference compiler from the JSPM CDN, so its `scopes` block pins svelte's transitive deps too — hand-swapping the version alone can leave them stale. Regenerate the map for `svelte@NEW` with the JSPM generator:

```
npm i --no-save @jspm/generator
node --input-type=module -e '
  import { Generator } from "@jspm/generator";
  const g = new Generator({ defaultProvider: "jspm.io", env: ["browser","module","production"] });
  await g.install("svelte@NEW/compiler");
  console.log(JSON.stringify(g.getMap(), null, 2));
'
```

From the output, replace in `playground/index.html`:

- `imports["svelte/compiler"]` — the new CDN URL,
- the whole `scopes["https://ga.jspm.io/"]` block.

Then set the UI label — the `drawer-subtitle` line reading `svelte/compiler · v…` — to `svelte/compiler · vNEW`.

If the generator is unavailable, fall back to the online generator at generator.jspm.io (provider `jspm.io`, env `browser`/`module`/`production`, install `svelte@NEW/compiler`).

Criteria:

- `grep -c 'svelte@OLD' playground/index.html` → `0`
- `curl -fsI "https://ga.jspm.io/npm:svelte@NEW/src/compiler/index.js"` → `200`
- (manual) `just playground`, open it — the svelte side compiles a sample and the label reads `vNEW`.

### 7. Sweep: no OLD version left

```
grep -rn 'OLD' . --exclude-dir=node_modules --exclude-dir=.git --exclude-dir=target
```

Criterion: output empty, or only clearly-historical occurrences (changelog and the like). Any pin/vendored/prose/import-map hit of OLD means a missed site — go back and fix it.

### 8. Regenerate expected outputs

`just generate` — rewrites every `case-svelte.js` from the new `svelte/compiler`. A large diff here is expected: it is the new reference output.

Criterion: the command finishes without errors.

### 9. Verify green

Run in order: `just test-compiler`, `just test-diagnostics`, `just lint`.

Criterion: all three green. For `test-compiler`, apply the Task boundary above: new failures are parity work to report, not expected files to hand-edit.
