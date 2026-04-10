import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, '..');

const version = process.argv[2];

if (!version) {
  throw new Error('Usage: node scripts/set-canary-version.mjs <version>');
}

const packageJsonPaths = [
  'packages/svelte-rs2/package.json',
  'packages/svelte-rs2-linux-x64-gnu/package.json',
  'packages/svelte-rs2-darwin-arm64/package.json',
  'packages/svelte-rs2-darwin-x64/package.json'
];

for (const relPath of packageJsonPaths) {
  const filePath = path.resolve(repoRoot, relPath);
  const pkg = JSON.parse(fs.readFileSync(filePath, 'utf8'));
  pkg.version = version;

  if (relPath === 'packages/svelte-rs2/package.json') {
    for (const depName of Object.keys(pkg.optionalDependencies ?? {})) {
      pkg.optionalDependencies[depName] = version;
    }
  }

  fs.writeFileSync(filePath, `${JSON.stringify(pkg, null, 2)}\n`);
}

console.log(`Updated package versions to ${version}`);
