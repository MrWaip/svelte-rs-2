import { execFileSync } from 'node:child_process';
import { createRequire } from 'node:module';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const worker = resolve(here, 'worker.mjs');

const targetDir = process.argv[2] && process.argv[2] !== '.' ? process.argv[2] : null;

const MODES = ['client', 'client-dev', 'ssr', 'ssr-dev'];

const require = createRequire(import.meta.url);
function has(pkg) {
    try {
        require.resolve(pkg);
        return true;
    } catch {
        return false;
    }
}

const COMPETITORS = [
    { id: 'rsvelte', label: 'rsv-native', pkg: '@rsvelte/vite-plugin-svelte-native' },
    { id: 'rsvelte-wasm', label: 'rsv-wasm', pkg: '@rsvelte/compiler' },
].filter((c) => {
    if (has(c.pkg)) return true;
    process.stderr.write(
        `${c.id} not installed — skipping. to compare against it: npm i -D ${c.pkg}\n`,
    );
    return false;
});

const COMPILERS = ['svelte', 'ours', ...COMPETITORS.map((c) => c.id)];
const LABELS = { svelte: 'svelte', ours: 'ours', ...Object.fromEntries(COMPETITORS.map((c) => [c.id, c.label])) };

function run(compiler, mode) {
    const argv = [worker, compiler, mode, ...(targetDir ? [targetDir] : [])];
    const out = execFileSync('node', argv, {
        encoding: 'utf8',
        maxBuffer: 64 * 1024 * 1024,
    });
    return JSON.parse(out);
}

process.stderr.write(`benchmarking .svelte files in: ${targetDir ?? '<repo root>'}\n`);

const rows = [];
for (const mode of MODES) {
    const results = {};
    for (const compiler of COMPILERS) {
        process.stderr.write(`running ${compiler} / ${mode} ...\n`);
        results[compiler] = run(compiler, mode);
    }
    rows.push({ mode, results });
}

const W = 15;
const SW = 19;
const pad = (value, width = W) => String(value).padStart(width);
const fmt = (ms) => (ms == null ? 'n/a' : ms.toFixed(1));
const speedup = (base, ms) => (ms == null || base == null ? 'n/a' : `${(base / ms).toFixed(2)}×`);

const speedupCols = ['svelte', ...COMPETITORS.map((c) => c.id)];
const header =
    'mode'.padEnd(12) +
    COMPILERS.map((c) => pad(`${LABELS[c]}(ms)`)).join('') +
    speedupCols.map((c) => pad(`ours vs ${LABELS[c]}`, SW)).join('');

process.stdout.write('\n');
process.stdout.write(`${header}\n`);
process.stdout.write(`${'-'.repeat(header.length)}\n`);

for (const { mode, results } of rows) {
    const oursMs = results.ours.median_ms;
    let line = mode.padEnd(12) + COMPILERS.map((c) => pad(fmt(results[c].median_ms))).join('');
    line += speedupCols.map((c) => pad(speedup(results[c].median_ms, oursMs), SW)).join('');
    process.stdout.write(`${line}\n`);
}

process.stdout.write('\nskipped (files that threw, per compiler):\n');
for (const { mode, results } of rows) {
    const skips = COMPILERS.map((c) =>
        results[c].unsupported ? `${c} n/a` : `${c} ${results[c].skipped}`,
    ).join('  ');
    process.stdout.write(`  ${mode.padEnd(12)} ${skips}\n`);
}
process.stdout.write('\n');
