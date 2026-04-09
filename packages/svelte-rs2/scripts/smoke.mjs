import { execSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const packageRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(packageRoot, '..', '..');

function run(cmd) {
  execSync(cmd, { stdio: 'inherit', cwd: repoRoot });
}

function copyNativeAddon() {
  const targetDir = path.resolve(packageRoot, 'compiler/native');
  fs.mkdirSync(targetDir, { recursive: true });

  const ext = process.platform === 'darwin' ? 'dylib' : process.platform === 'win32' ? 'dll' : 'so';
  const srcName = process.platform === 'win32' ? 'napi_compiler.dll' : `libnapi_compiler.${ext}`;
  const srcPath = path.resolve(repoRoot, 'target/debug', srcName);
  const dstPath = path.resolve(targetDir, 'svelte-rs2.node');

  if (!fs.existsSync(srcPath)) {
    throw new Error(`Native library was not produced: ${srcPath}`);
  }

  fs.copyFileSync(srcPath, dstPath);
}

run('cargo build -p napi_compiler');
copyNativeAddon();

const api = await import('@mrwaip/svelte-rs2/compiler');

const compileResult = api.compile('<script>let count = 1;</script><h1>{count}</h1>', {
  filename: 'Counter.svelte'
});

if (!compileResult || typeof compileResult !== 'object') {
  throw new Error('compile must return an object');
}
if (!compileResult.js || typeof compileResult.js.code !== 'string') {
  throw new Error('compile result must contain js.code');
}
if (!Array.isArray(compileResult.warnings)) {
  throw new Error('compile result must contain warnings array');
}
if (!('metadata' in compileResult)) {
  throw new Error('compile result must contain metadata');
}

const moduleResult = api.compileModule('let x = $state(1); export { x };', {
  filename: 'mod.svelte.js'
});

if (!moduleResult || typeof moduleResult !== 'object') {
  throw new Error('compileModule must return an object');
}
if (!moduleResult.js || typeof moduleResult.js.code !== 'string') {
  throw new Error('compileModule result must contain js.code');
}
if (!Array.isArray(moduleResult.warnings)) {
  throw new Error('compileModule result must contain warnings array');
}
if (!('metadata' in moduleResult)) {
  throw new Error('compileModule result must contain metadata');
}

console.log('Smoke tests passed');
