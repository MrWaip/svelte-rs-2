import { readFileSync, readdirSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';

const oursApi = await import('@mrwaip/svelte-rs2/compiler');

const casesDir = path.join(path.dirname(fileURLToPath(import.meta.url)), 'cluster_cases/preprocess');

function normalizeMapSources(map, dir) {
  if (!map) return null;
  const prefix = pathToFileURL(dir + path.sep).href;
  return [...map.sources].map((source) =>
    source.startsWith(prefix) ? source.slice(prefix.length) : source
  );
}

function loadCase(name) {
  const dir = path.join(casesDir, name);
  const source = readFileSync(path.join(dir, 'component.svelte'), 'utf-8');
  const expected = JSON.parse(readFileSync(path.join(dir, 'expected.json'), 'utf-8'));
  return { name, dir, source, expected, preprocessorsPath: path.join(dir, 'preprocessors.mjs') };
}

async function runCase({ name, dir, source, expected, preprocessorsPath }) {
  const { default: createPreprocessors } = await import(preprocessorsPath);
  const filename = `${name}.svelte`;

  const result = await oursApi.preprocess(source, createPreprocessors(), { filename });

  if (result.code !== expected.code) {
    return {
      name,
      ok: false,
      reason: `code mismatch:\n--- ours ---\n${result.code}\n--- expected (svelte/compiler) ---\n${expected.code}`
    };
  }

  const ourDeps = [...result.dependencies].sort();
  const expectedDeps = [...expected.dependencies].sort();
  if (JSON.stringify(ourDeps) !== JSON.stringify(expectedDeps)) {
    return {
      name,
      ok: false,
      reason: `dependencies mismatch: ours=${JSON.stringify(ourDeps)}, expected=${JSON.stringify(expectedDeps)}`
    };
  }

  const ourSources = normalizeMapSources(result.map, dir);
  if (JSON.stringify(ourSources) !== JSON.stringify(expected.mapSources)) {
    return {
      name,
      ok: false,
      reason: `map.sources mismatch: ours=${JSON.stringify(ourSources)}, expected=${JSON.stringify(expected.mapSources)}`
    };
  }

  return { name, ok: true };
}

function listCaseNames() {
  return readdirSync(casesDir).filter((entry) =>
    readdirSync(path.join(casesDir, entry)).includes('component.svelte')
  );
}

const requestedName = process.argv[2];
const names = requestedName ? [requestedName] : listCaseNames();

const results = [];
for (const name of names) {
  results.push(await runCase(loadCase(name)));
}

let failed = 0;
for (const result of results) {
  if (result.ok) {
    console.log(`ok   ${result.name}`);
  } else {
    failed++;
    console.error(`FAIL ${result.name}\n  ${result.reason}`);
  }
}

if (!requestedName) {
  console.log(`\n${results.length - failed}/${results.length} preprocess cases passed`);
}
if (failed > 0) {
  process.exit(1);
}
