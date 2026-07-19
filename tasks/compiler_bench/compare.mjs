import { execFileSync } from 'node:child_process';
import { createRequire } from 'node:module';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const worker = resolve(here, 'worker.mjs');

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

const rsvelteAvailable = has('@rsvelte/vite-plugin-svelte-native');
const COMPILERS = ['svelte', 'ours', ...(rsvelteAvailable ? ['rsvelte'] : [])];

if (!rsvelteAvailable) {
    process.stderr.write(
        'rsvelte not installed — showing svelte vs ours only.\n' +
            'to compare against the competitor: npm i -D @rsvelte/vite-plugin-svelte-native\n\n',
    );
}

function run(compiler, mode) {
    const out = execFileSync('node', [worker, compiler, mode], {
        encoding: 'utf8',
        maxBuffer: 64 * 1024 * 1024,
    });
    return JSON.parse(out);
}

const rows = [];
for (const mode of MODES) {
    const results = {};
    for (const compiler of COMPILERS) {
        process.stderr.write(`running ${compiler} / ${mode} ...\n`);
        results[compiler] = run(compiler, mode);
    }
    rows.push({ mode, results });
}

const pad = (value, width) => String(value).padStart(width);
const fmt = (ms) => ms.toFixed(1);
const speedup = (base, ms) => `${(base / ms).toFixed(2)}×`;

const header =
    'mode'.padEnd(12) +
    COMPILERS.map((c) => pad(`${c}(ms)`, 13)).join('') +
    pad('ours vs svelte', 16) +
    (rsvelteAvailable ? pad('rsv vs svelte', 15) + pad('ours vs rsv', 13) : '');

process.stdout.write('\n');
process.stdout.write(`${header}\n`);
process.stdout.write(`${'-'.repeat(header.length)}\n`);

for (const { mode, results } of rows) {
    const svelteMs = results.svelte.median_ms;
    const oursMs = results.ours.median_ms;
    let line = mode.padEnd(12) + COMPILERS.map((c) => pad(fmt(results[c].median_ms), 13)).join('');
    line += pad(speedup(svelteMs, oursMs), 16);
    if (rsvelteAvailable) {
        const rsvMs = results.rsvelte.median_ms;
        line += pad(speedup(svelteMs, rsvMs), 15) + pad(speedup(rsvMs, oursMs), 13);
    }
    process.stdout.write(`${line}\n`);
}

process.stdout.write('\nskipped (files that threw, per compiler):\n');
for (const { mode, results } of rows) {
    const skips = COMPILERS.map((c) => `${c} ${results[c].skipped}`).join('  ');
    process.stdout.write(`  ${mode.padEnd(12)} ${skips}\n`);
}
process.stdout.write('\n');
