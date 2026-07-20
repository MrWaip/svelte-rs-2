import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, resolve } from 'node:path';
import { performance } from 'node:perf_hooks';
import { fileURLToPath } from 'node:url';
import { glob } from 'glob';

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, '..', '..');
const require = createRequire(import.meta.url);

const dirArg = process.argv[2];
if (!dirArg) {
    process.stderr.write('usage: node breakdown.mjs <dir> [mode] [top]\n');
    process.exit(1);
}
const mode = process.argv[3] ?? 'client';
const top = Number(process.argv[4] ?? 15);
const samples = Number(process.env.BENCH_REPEATS ?? 9);
const batch = Number(process.env.BENCH_BATCH ?? 1);

const MODES = {
    client: { generate: 'client', dev: false },
    'client-dev': { generate: 'client', dev: true },
    ssr: { generate: 'server', dev: false },
    'ssr-dev': { generate: 'server', dev: true },
};
if (!MODES[mode]) throw new Error(`unknown mode: ${mode}`);
const opts = MODES[mode];
const flameFlags = [
    ...(opts.generate === 'server' ? ['--server'] : []),
    ...(opts.dev ? ['--dev'] : []),
].join(' ');

const searchDir = resolve(process.cwd(), dirArg);

const svelte = await import('svelte/compiler');
const oursMod = await import(resolve(repoRoot, 'packages/svelte-rs2/compiler/index.js'));
const ours = (src, o) => {
    const r = oursMod.compile(src, o);
    if (!r || !r.js) throw new Error('no js output');
    return r;
};
const ref = (src, o) => svelte.compile(src, o);

const relFiles = (await glob('**/*.svelte', { cwd: searchDir, ignore: [] })).sort();
if (!relFiles.length) {
    process.stderr.write(`no .svelte files in ${searchDir}\n`);
    process.exit(0);
}

function medianMs(compile, src, o) {
    for (let i = 0; i < 2; i++) compile(src, o);
    const s = [];
    for (let i = 0; i < samples; i++) {
        const t = performance.now();
        for (let b = 0; b < batch; b++) compile(src, o);
        s.push((performance.now() - t) / batch);
    }
    s.sort((a, b) => a - b);
    return s[Math.floor(s.length / 2)];
}

const rows = [];
const failed = [];
const total = relFiles.length;
const tty = process.stderr.isTTY;
process.stderr.write(`timing ${total} files (median of ${samples}×${batch}) ...\n`);
let done = 0;
for (const rel of relFiles) {
    done += 1;
    if (tty && (done % 20 === 0 || done === total)) {
        process.stderr.write(`\r  ${done}/${total}   `);
        if (done === total) process.stderr.write('\n');
    }
    const src = readFileSync(resolve(searchDir, rel), 'utf8');
    const o = { ...opts, filename: rel };
    let oursMs;
    try {
        oursMs = medianMs(ours, src, o);
    } catch (e) {
        failed.push({ rel, error: String(e.message ?? e).split('\n')[0] });
        continue;
    }
    let refMs = null;
    try {
        refMs = medianMs(ref, src, o);
    } catch {
        refMs = null;
    }
    rows.push({
        rel,
        abs: resolve(searchDir, rel),
        ours: oursMs,
        svelte: refMs,
        ratio: refMs == null ? null : refMs / oursMs,
    });
}

const pad = (v, w) => String(v).padStart(w);
const fmt = (ms) => (ms == null ? 'n/a' : ms.toFixed(3));
const x = (r) => (r == null ? 'n/a' : `${r.toFixed(1)}×`);

function table(list) {
    process.stdout.write(
        `  ${pad('ours(ms)', 10)}${pad('svelte(ms)', 12)}${pad('x', 8)}  file\n`,
    );
    for (const r of list) {
        process.stdout.write(
            `  ${pad(fmt(r.ours), 10)}${pad(fmt(r.svelte), 12)}${pad(x(r.ratio), 8)}  ${r.rel}\n`,
        );
    }
}

process.stdout.write(
    `\nper-file breakdown — ${searchDir}\nmode=${mode} — ${rows.length} files timed` +
        (failed.length ? `, ${failed.length} failed in ours` : '') +
        ` — median of ${samples}×${batch}\n\n`,
);

process.stdout.write(`slowest for ours (top ${top} by our compile time):\n`);
table([...rows].sort((a, b) => b.ours - a.ours).slice(0, top));

const withRatio = rows.filter((r) => r.ratio != null);
const svelteSorted = withRatio.map((r) => r.svelte).sort((a, b) => a - b);
const svelteMedian = svelteSorted[Math.floor(svelteSorted.length / 2)] ?? 0;
const meaningful = withRatio.filter((r) => r.svelte >= svelteMedian);
process.stdout.write(
    `\nweakest lead (top ${top} by lowest ours-vs-svelte ratio, files ≥ median svelte time):\n`,
);
table([...meaningful].sort((a, b) => a.ratio - b.ratio).slice(0, top));

if (failed.length) {
    process.stdout.write(`\nfailed to compile in ours (${failed.length}):\n`);
    for (const f of failed.slice(0, top)) {
        process.stdout.write(`  ${f.rel}  —  ${f.error}\n`);
    }
    if (failed.length > top) process.stdout.write(`  … and ${failed.length - top} more\n`);
}

const hottest = [...rows].sort((a, b) => b.ours - a.ours)[0];
if (hottest) {
    process.stdout.write(
        `\nflamegraph the hottest file:\n  just bench-flame ${hottest.abs}${flameFlags ? ` ${flameFlags}` : ''}\n\n`,
    );
}
