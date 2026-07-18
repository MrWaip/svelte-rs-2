import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { performance } from 'node:perf_hooks';
import { glob } from 'glob';

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, '..', '..');

const compiler = process.argv[2];
const mode = process.argv[3];
const repeats = Number(process.env.BENCH_REPEATS ?? 5);
const warmups = Number(process.env.BENCH_WARMUPS ?? 1);

const MODES = {
    client: { generate: 'client', dev: false },
    'client-dev': { generate: 'client', dev: true },
    ssr: { generate: 'server', dev: false },
    'ssr-dev': { generate: 'server', dev: true },
};

if (!MODES[mode]) throw new Error(`unknown mode: ${mode}`);
const baseOptions = MODES[mode];

async function loadCompile() {
    if (compiler === 'svelte') {
        const mod = await import('svelte/compiler');
        return (src, options) => mod.compile(src, options);
    }
    if (compiler === 'ours') {
        const mod = await import(resolve(repoRoot, 'packages/svelte-rs2/compiler/index.js'));
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
    throw new Error(`unknown compiler: ${compiler}`);
}

const compile = await loadCompile();

const relFiles = (
    await glob('**/*.svelte', {
        cwd: repoRoot,
        ignore: ['**/node_modules/**', 'original/**', 'target/**'],
    })
).sort();

const sources = relFiles.map((rel) => ({
    filename: rel,
    src: readFileSync(resolve(repoRoot, rel), 'utf8'),
}));

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
