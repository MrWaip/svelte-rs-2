#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';

const root = process.argv[2] || 'profile';
const dirs = fs.readdirSync(root, { withFileTypes: true })
  .filter(d => d.isDirectory())
  .map(d => path.join(root, d.name));

const selfTotal = new Map();
const incTotal = new Map();
const perCase = new Map();
let totalAll = 0;

for (const dir of dirs) {
  const fp = path.join(dir, 'profile.folded');
  if (!fs.existsSync(fp)) continue;

  const caseName = path.basename(dir);
  const caseSelf = new Map();
  let caseTotal = 0;

  for (const line of fs.readFileSync(fp, 'utf8').split('\n')) {
    if (!line) continue;
    const m = line.match(/^(.+) (\d+)$/);
    if (!m) continue;
    const stack = m[1];
    const n = +m[2];
    totalAll += n;
    caseTotal += n;
    const frames = stack.split(';');
    const leaf = frames[frames.length - 1];
    selfTotal.set(leaf, (selfTotal.get(leaf) || 0) + n);
    caseSelf.set(leaf, (caseSelf.get(leaf) || 0) + n);
    const seen = new Set();
    for (const f of frames) {
      if (seen.has(f)) continue;
      seen.add(f);
      incTotal.set(f, (incTotal.get(f) || 0) + n);
    }
  }

  perCase.set(caseName, { self: caseSelf, total: caseTotal });
}

function fmt(map, n, filter = () => true) {
  return [...map.entries()]
    .filter(([k]) => filter(k))
    .sort((a, b) => b[1] - a[1])
    .slice(0, n)
    .map(([k, v]) => `${String(v).padStart(7)}  ${(100 * v / totalAll).toFixed(2).padStart(5)}%  ${k}`)
    .join('\n');
}

const isNotSystem = (k) => !/^dyld!|^libsystem/.test(k);
const isSvelte = (k) => /^profile!|svelte/i.test(k) && isNotSystem(k);

const cases = [...perCase.entries()]
  .map(([n, v]) => `  ${n.padEnd(40)} ${v.total} samples`)
  .join('\n');

const out = [
  `cases: ${dirs.length}`,
  `total samples (sum): ${totalAll}`,
  '',
  '=== PER-CASE SAMPLE COUNT ===',
  cases,
  '',
  '=== AGGREGATE TOP 50 SELF (excluding dyld/libsystem) ===',
  fmt(selfTotal, 50, isNotSystem),
  '',
  '=== AGGREGATE TOP 30 SELF (raw, with system frames) ===',
  fmt(selfTotal, 30),
  '',
  '=== AGGREGATE TOP 50 INCLUSIVE (svelte/profile only) ===',
  fmt(incTotal, 50, isSvelte),
  '',
].join('\n');

const outPath = path.join(root, 'aggregate.top.txt');
fs.writeFileSync(outPath, out);
console.error(`wrote: ${outPath}`);
