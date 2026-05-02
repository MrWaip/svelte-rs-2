import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import Benchmark from 'benchmark';
import { withCodSpeed } from '@codspeed/benchmark.js-plugin';
import { glob } from 'glob';
import { compile, compileModule } from 'svelte/compiler';

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, 'benches/compiler');

const svelteFiles = (await glob('**/*.svelte', { cwd: root })).sort();
const moduleFiles = (await glob('**/*.svelte.js', { cwd: root })).sort();

const suite = withCodSpeed(new Benchmark.Suite('svelte_node'));

for (const rel of svelteFiles) {
    const src = readFileSync(resolve(root, rel), 'utf8');
    suite.add(`compile[benches/compiler/${rel}]`, () => {
        compile(src, { generate: 'client', dev: false });
    });
    suite.add(`compile_dev[benches/compiler/${rel}]`, () => {
        compile(src, { generate: 'client', dev: true });
    });
}

for (const rel of moduleFiles) {
    const src = readFileSync(resolve(root, rel), 'utf8');
    const filename = `benches/compiler/${rel}`;
    suite.add(`compile_module[${filename}]`, () => {
        compileModule(src, { generate: 'client', dev: false, filename });
    });
    suite.add(`compile_module_dev[${filename}]`, () => {
        compileModule(src, { generate: 'client', dev: true, filename });
    });
}

suite.on('cycle', (event) => console.log(String(event.target)));

suite.run({ async: false });
