import { execFileSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const worker = resolve(here, 'worker.mjs');

const MODES = ['client', 'client-dev', 'ssr', 'ssr-dev'];
const COMPILERS = ['svelte', 'ours'];

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
    const svelte = results.svelte;
    const ours = results.ours;
    rows.push({
        mode,
        files: ours.files,
        skippedOurs: ours.skipped,
        skippedSvelte: svelte.skipped,
        svelteMs: svelte.median_ms,
        oursMs: ours.median_ms,
        speedup: svelte.median_ms / ours.median_ms,
    });
}

const pad = (value, width) => String(value).padStart(width);
const fmt = (ms) => ms.toFixed(1);

process.stdout.write('\n');
process.stdout.write(
    `${'mode'.padEnd(12)}${pad('files', 7)}${pad('skip(o/s)', 12)}${pad('svelte(ms)', 13)}${pad('ours(ms)', 11)}${pad('speedup', 10)}\n`,
);
process.stdout.write(`${'-'.repeat(65)}\n`);
for (const r of rows) {
    process.stdout.write(
        `${r.mode.padEnd(12)}${pad(r.files, 7)}${pad(`${r.skippedOurs}/${r.skippedSvelte}`, 12)}${pad(fmt(r.svelteMs), 13)}${pad(fmt(r.oursMs), 11)}${pad(`${r.speedup.toFixed(2)}×`, 10)}\n`,
    );
}
process.stdout.write('\n');
