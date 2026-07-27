import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { glob } from 'glob';

const here = dirname(fileURLToPath(import.meta.url));

export const repoRoot = resolve(here, '..', '..');

export function resolveSearchDir(target) {
    if (!target || target === '.') return { searchDir: repoRoot, explicitDir: false };
    return { searchDir: resolve(process.cwd(), target), explicitDir: true };
}

export async function discoverFiles(searchDir, explicitDir) {
    const relFiles = await glob('**/*.svelte', {
        cwd: searchDir,
        ignore: explicitDir ? [] : ['**/node_modules/**', 'original/**', 'target/**'],
    });
    return relFiles.sort();
}
