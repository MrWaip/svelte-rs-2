import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { glob } from 'glob';

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, '..');
const require = createRequire(resolve(repoRoot, 'packages/svelte-rs2/compiler/index.js'));
const native = require(resolve(repoRoot, 'packages/svelte-rs2/compiler/native/binding.cjs'));

const rounds = Number(process.env.PGO_ROUNDS ?? 3);
const corpusDirs = [
    resolve(repoRoot, 'tasks/benchmark/benches/compiler'),
    resolve(repoRoot, 'tasks/compiler_tests/cases2'),
    ...(process.env.PGO_CORPUS ?? '').split(/\s+/).filter(Boolean),
];

const files = [];
for (const dir of corpusDirs) {
    for (const rel of await glob('**/*.{svelte,svelte.js,svelte.ts}', { cwd: dir })) {
        const path = resolve(dir, rel);
        try {
            files.push([path, readFileSync(path, 'utf8')]);
        } catch {
            /* unreadable entry — skip */
        }
    }
}

process.stderr.write(`pgo-train: ${files.length} files x ${rounds} rounds\n`);

const modes = [];
for (const generate of ['client', 'server']) {
    for (const dev of [false, true]) {
        modes.push({ generate, dev });
    }
}

for (let round = 0; round < rounds; round++) {
    for (const [filename, source] of files) {
        const isModule = filename.endsWith('.js') || filename.endsWith('.ts');
        for (const mode of modes) {
            try {
                if (isModule) {
                    native.compileModule(source, { ...mode, filename });
                } else {
                    native.compile(source, { ...mode, filename });
                }
            } catch {
                /* invalid fixtures are expected in the corpus */
            }
        }
    }
}
