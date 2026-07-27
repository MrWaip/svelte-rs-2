import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { resolve } from 'node:path';
import { performance } from 'node:perf_hooks';
import { discoverFiles, repoRoot, resolveSearchDir } from './corpus.mjs';

const argv = process.argv.slice(2);
const asyncFlag = argv.includes('--async');
const positional = argv.filter((a) => !a.startsWith('--'));
const compiler = positional[0];
const mode = positional[1];
const { searchDir, explicitDir } = resolveSearchDir(positional[2]);
const repeats = Number(process.env.BENCH_REPEATS ?? 5);
const warmups = Number(process.env.BENCH_WARMUPS ?? 1);

const MODES = {
    client: { generate: 'client', dev: false },
    'client-dev': { generate: 'client', dev: true },
    ssr: { generate: 'server', dev: false },
    'ssr-dev': { generate: 'server', dev: true },
};

if (!MODES[mode]) throw new Error(`unknown mode: ${mode}`);
const baseOptions = { ...MODES[mode], ...(asyncFlag ? { experimental: { async: true } } : {}) };

const require = createRequire(import.meta.url);

if (compiler === 'rsvelte-wasm' && baseOptions.dev) {
    process.stdout.write(
        JSON.stringify({
            compiler,
            mode,
            files: 0,
            skipped: 0,
            median_ms: null,
            timings_ms: [],
            unsupported: true,
        }),
    );
    process.exit(0);
}

async function loadCompile() {
    if (compiler === 'svelte') {
        const mod = await import('svelte/compiler');
        return (src, options) => mod.compile(src, options);
    }
    if (compiler === 'ours') {
        const mod = await import(resolve(repoRoot, 'packages/svelte-rs/compiler/index.js'));
        return (src, options) => {
            const result = mod.compile(src, options);
            if (!result || !result.js) throw new Error('no js output');
            return result;
        };
    }
    if (compiler === 'rsvelte') {
        const mod = await import('@rsvelte/vite-plugin-svelte-native');
        return (src, options) => {
            const result = mod.compile(src, options);
            if (!result || !result.js) throw new Error('no js output');
            return result;
        };
    }
    if (compiler === 'rsvelte-wasm') {
        const mod = await import('@rsvelte/compiler');
        const wasmPath = require.resolve('@rsvelte/compiler/rsvelte_lint_bg.wasm');
        await mod.default({ module_or_path: readFileSync(wasmPath) });
        const compileFn = baseOptions.generate === 'server' ? mod.compile_server : mod.compile_client;
        return (src, options) => {
            const result = compileFn(src, options.filename ?? '');
            if (!result || !result.success) throw new Error(result?.error ?? 'no js output');
            return { js: result.js, css: result.css };
        };
    }
    throw new Error(`unknown compiler: ${compiler}`);
}

const compile = await loadCompile();

const relFiles = await discoverFiles(searchDir, explicitDir);

const sources = relFiles.map((rel) => ({
    filename: rel,
    src: readFileSync(resolve(searchDir, rel), 'utf8'),
}));

if (sources.length === 0) {
    process.stdout.write(
        JSON.stringify({
            compiler,
            mode,
            files: 0,
            skipped: 0,
            median_ms: null,
            timings_ms: [],
        }),
    );
    process.exit(0);
}

const survivors = [];
let skipped = 0;
for (const item of sources) {
    try {
        compile(item.src, { ...baseOptions, filename: item.filename });
        survivors.push(item);
    } catch {
        skipped += 1;
    }
}

function runPass() {
    const start = performance.now();
    for (const item of survivors) {
        try {
            compile(item.src, { ...baseOptions, filename: item.filename });
        } catch {
            void 0;
        }
    }
    return performance.now() - start;
}

for (let i = 0; i < warmups; i++) runPass();

const timings = [];
for (let i = 0; i < repeats; i++) timings.push(runPass());
timings.sort((a, b) => a - b);
const median = timings[Math.floor(timings.length / 2)];

process.stdout.write(
    JSON.stringify({
        compiler,
        mode,
        files: survivors.length,
        skipped,
        median_ms: median,
        timings_ms: timings,
    }),
);
