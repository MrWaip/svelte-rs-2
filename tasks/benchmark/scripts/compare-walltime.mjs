#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { performance } from 'node:perf_hooks';
import { glob } from 'glob';
import { compile, compileModule } from 'svelte/compiler';

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, '../../..');
const benchRoot = join(repoRoot, 'tasks', 'benchmark', 'benches', 'compiler');

function parseArgs(argv) {
    const opts = { seconds: 3, warmup: 0.5, minIters: 5, filter: null, build: false, async: false };
    for (let i = 0; i < argv.length; i++) {
        const a = argv[i];
        if (a === '--seconds') opts.seconds = parseFloat(argv[++i]);
        else if (a.startsWith('--seconds=')) opts.seconds = parseFloat(a.slice(10));
        else if (a === '--warmup') opts.warmup = parseFloat(argv[++i]);
        else if (a.startsWith('--warmup=')) opts.warmup = parseFloat(a.slice(9));
        else if (a === '--min-iters') opts.minIters = parseInt(argv[++i], 10);
        else if (a.startsWith('--min-iters=')) opts.minIters = parseInt(a.slice(12), 10);
        else if (a === '--filter') opts.filter = argv[++i];
        else if (a.startsWith('--filter=')) opts.filter = a.slice(9);
        else if (a === '--build') opts.build = true;
        else if (a === '--async') opts.async = true;
    }
    return opts;
}

function buildRustBin() {
    console.log('>>> cargo build --release -p benchmark --bin bench_once');
    const r = spawnSync('cargo', ['build', '--release', '-p', 'benchmark', '--bin', 'bench_once'], {
        cwd: repoRoot,
        stdio: 'inherit',
    });
    if (r.status !== 0) throw new Error(`cargo build failed (${r.status})`);
}

function runRust(file, { seconds, warmup, minIters, dev, module, async: asyncFlag }) {
    const bin = join(repoRoot, 'target', 'release', 'bench_once');
    const args = [file, String(seconds), String(warmup), String(minIters)];
    if (dev) args.push('--dev');
    if (asyncFlag) args.push('--async');
    if (module !== undefined) args.push(module ? '--module' : '--compile');

    const r = spawnSync(bin, args, { cwd: repoRoot, encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
    if (r.status !== 0) {
        throw new Error(`bench_once failed for ${file}: ${r.stderr || r.stdout}`);
    }
    return r.stdout
        .split(/\r?\n/)
        .map((s) => s.trim())
        .filter(Boolean)
        .map((s) => Number(s));
}

function runJsLoop(fn, { seconds, warmup, minIters }) {
    const warmupDeadline = performance.now() + warmup * 1000;
    while (performance.now() < warmupDeadline) fn();

    const samples = [];
    const deadline = performance.now() + seconds * 1000;
    while (true) {
        const t0 = performance.now();
        fn();
        samples.push((performance.now() - t0) * 1e6);
        if (samples.length >= minIters && performance.now() >= deadline) break;
    }
    return samples;
}

function stats(samples) {
    const n = samples.length;
    if (n === 0) return null;
    const sorted = [...samples].sort((a, b) => a - b);
    const mean = samples.reduce((s, v) => s + v, 0) / n;
    const median = sorted[Math.floor(n / 2)];
    const p95 = sorted[Math.min(n - 1, Math.floor(n * 0.95))];
    const min = sorted[0];
    return { mean, median, p95, min, n };
}

function fmtMs(ns) {
    const ms = ns / 1e6;
    if (ms >= 100) return ms.toFixed(1);
    if (ms >= 10) return ms.toFixed(2);
    if (ms >= 1) return ms.toFixed(3);
    return ms.toFixed(4);
}

function geomean(values) {
    if (values.length === 0) return NaN;
    const sumLn = values.reduce((s, v) => s + Math.log(v), 0);
    return Math.exp(sumLn / values.length);
}

function printTable(rows) {
    const headers = ['case', 'rust med', 'rust mean', 'rust n', 'js med', 'js mean', 'js n', 'speedup (med)'];
    const widths = headers.map((h) => h.length);

    const formatted = rows.map((r) => [
        r.label,
        fmtMs(r.rust.median),
        fmtMs(r.rust.mean),
        String(r.rust.n),
        fmtMs(r.js.median),
        fmtMs(r.js.mean),
        String(r.js.n),
        `${(r.js.median / r.rust.median).toFixed(2)}x`,
    ]);

    for (const row of formatted) {
        for (let i = 0; i < row.length; i++) widths[i] = Math.max(widths[i], row[i].length);
    }

    const fmtRow = (cols) =>
        cols.map((c, i) => (i === 0 ? c.padEnd(widths[i]) : c.padStart(widths[i]))).join('  ');

    console.log('');
    console.log(fmtRow(headers));
    console.log(widths.map((w) => '-'.repeat(w)).join('  '));
    for (const row of formatted) console.log(fmtRow(row));
}

async function main() {
    const opts = parseArgs(process.argv.slice(2));
    if (opts.build) buildRustBin();

    const svelteFiles = (await glob('**/*.svelte', { cwd: benchRoot })).sort();
    const moduleFiles = (await glob('**/*.svelte.js', { cwd: benchRoot })).sort();

    const cases = [];
    for (const rel of svelteFiles) {
        const abs = join(benchRoot, rel);
        const labelRel = `benches/compiler/${rel}`;
        cases.push({ kind: 'compile', dev: false, abs, label: `compile[${labelRel}]` });
        cases.push({ kind: 'compile', dev: true, abs, label: `compile_dev[${labelRel}]` });
    }
    for (const rel of moduleFiles) {
        const abs = join(benchRoot, rel);
        const labelRel = `benches/compiler/${rel}`;
        cases.push({ kind: 'module', dev: false, abs, label: `compile_module[${labelRel}]` });
        cases.push({ kind: 'module', dev: true, abs, label: `compile_module_dev[${labelRel}]` });
    }

    const filtered = opts.filter ? cases.filter((c) => c.label.includes(opts.filter)) : cases;

    if (filtered.length === 0) {
        console.error('no cases matched');
        process.exit(1);
    }

    console.log(
        `cases: ${filtered.length}, seconds: ${opts.seconds}, warmup: ${opts.warmup}s, min-iters: ${opts.minIters}, async: ${opts.async}\n`,
    );

    const rows = [];
    for (const c of filtered) {
        process.stderr.write(`>>> ${c.label}\n`);
        const source = readFileSync(c.abs, 'utf8');

        const rustSamples = runRust(c.abs, {
            seconds: opts.seconds,
            warmup: opts.warmup,
            minIters: opts.minIters,
            dev: c.dev,
            module: c.kind === 'module',
            async: opts.async,
        });

        const jsCompileOpts = {
            generate: 'client',
            dev: c.dev,
            filename: c.abs,
            ...(opts.async ? { experimental: { async: true } } : {}),
        };
        const jsFn =
            c.kind === 'compile'
                ? () => compile(source, jsCompileOpts)
                : () => compileModule(source, jsCompileOpts);
        const jsSamples = runJsLoop(jsFn, {
            seconds: opts.seconds,
            warmup: opts.warmup,
            minIters: opts.minIters,
        });

        rows.push({ label: c.label, rust: stats(rustSamples), js: stats(jsSamples) });
    }

    rows.sort((a, b) => b.js.median / b.rust.median - a.js.median / a.rust.median);
    printTable(rows);

    const speedups = rows.map((r) => r.js.median / r.rust.median);
    const gm = geomean(speedups);
    const min = rows.reduce((a, b) =>
        a.js.median / a.rust.median < b.js.median / b.rust.median ? a : b,
    );
    const max = rows.reduce((a, b) =>
        a.js.median / a.rust.median > b.js.median / b.rust.median ? a : b,
    );

    console.log('');
    console.log(`geomean speedup:  ${gm.toFixed(2)}x  (n=${rows.length})`);
    console.log(`min:              ${(min.js.median / min.rust.median).toFixed(2)}x  ${min.label}`);
    console.log(`max:              ${(max.js.median / max.rust.median).toFixed(2)}x  ${max.label}`);
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
