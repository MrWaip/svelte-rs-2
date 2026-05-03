#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';

const dir = process.argv[2] || 'profile';
const prof = JSON.parse(fs.readFileSync(path.join(dir, 'profile.json'), 'utf8'));
const symsPath = path.join(dir, 'profile.json.syms.json');
const syms = fs.existsSync(symsPath) ? JSON.parse(fs.readFileSync(symsPath, 'utf8')) : null;

const modules = [];
if (syms) {
  for (const m of syms.data) {
    const table = [...m.symbol_table].sort((a, b) => a.rva - b.rva);
    modules.push({ name: m.debug_name, rvas: table.map(s => s.rva), table });
  }
}

function resolveAddr(rva) {
  for (const m of modules) {
    let lo = 0, hi = m.rvas.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (m.rvas[mid] <= rva) lo = mid + 1;
      else hi = mid;
    }
    const i = lo - 1;
    if (i < 0) continue;
    const s = m.table[i];
    if (s.rva <= rva && rva < s.rva + (s.size || 0)) {
      return `${m.name}!${syms.string_table[s.symbol]}`;
    }
  }
  return null;
}

const folded = new Map();

for (const t of prof.threads) {
  const strs = t.stringArray
    ?? (Array.isArray(t.stringTable) ? t.stringTable : t.stringTable?.data ?? []);
  const fn = t.funcTable.name;
  const sf = t.stackTable.frame;
  const sp = t.stackTable.prefix;
  const ff = t.frameTable.func;

  const nameOf = (frIdx) => {
    if (frIdx == null) return null;
    const fi = ff[frIdx];
    if (fi == null) return null;
    const si = fn[fi];
    if (si == null) return null;
    let nm = strs[si];
    if (nm && nm.startsWith('0x')) {
      const rva = parseInt(nm, 16);
      if (Number.isFinite(rva)) {
        const r = resolveAddr(rva);
        if (r) nm = r;
      }
    }
    return nm;
  };

  for (const stkIdx of t.samples.stack) {
    if (stkIdx == null) continue;
    const stack = [];
    let cur = stkIdx;
    while (cur != null) {
      const n = nameOf(sf[cur]);
      if (n) stack.push(n);
      cur = sp[cur];
    }
    stack.reverse();
    const key = stack.join(';');
    folded.set(key, (folded.get(key) || 0) + 1);
  }
}

const out = path.join(dir, 'profile.folded');
const lines = [];
for (const [k, v] of folded) lines.push(`${k} ${v}`);
fs.writeFileSync(out, lines.join('\n') + '\n');

const selfMap = new Map();
const incMap = new Map();
let total = 0;
for (const [stack, n] of folded) {
  total += n;
  const frames = stack.split(';');
  const leaf = frames[frames.length - 1];
  selfMap.set(leaf, (selfMap.get(leaf) || 0) + n);
  const seen = new Set();
  for (const f of frames) {
    if (seen.has(f)) continue;
    seen.add(f);
    incMap.set(f, (incMap.get(f) || 0) + n);
  }
}

function fmt(map, n = 30) {
  const rows = [...map.entries()].sort((a, b) => b[1] - a[1]).slice(0, n);
  return rows.map(([k, v]) => `${String(v).padStart(6)}  ${(100 * v / total).toFixed(2).padStart(5)}%  ${k}`).join('\n');
}

const isSvelte = (k) => /svelte|^profile!/.test(k) && !/dyld|libsystem|criterion|std::rt|core::ptr::drop_in_place$/i.test(k);

const filteredInc = new Map([...incMap].filter(([k]) => isSvelte(k)));
const filteredSelf = new Map([...selfMap].filter(([k]) => !/^dyld!|^libsystem/.test(k)));

const report = [
  `total samples: ${total}`,
  `unique stacks: ${folded.size}`,
  '',
  '=== TOP 30 SELF (excluding dyld/libsystem) ===',
  fmt(filteredSelf, 30),
  '',
  '=== TOP 30 SELF (all, raw) ===',
  fmt(selfMap, 30),
  '',
  '=== TOP 40 INCLUSIVE (svelte/profile only) ===',
  fmt(filteredInc, 40),
  '',
].join('\n');

const topPath = path.join(dir, 'top.txt');
fs.writeFileSync(topPath, report);
console.error(`wrote: ${out}`);
console.error(`wrote: ${topPath}`);
