import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const packageRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(packageRoot, '..', '..');

function nativeLibPath() {
  const targetDir = process.env.NAPI_BUILD_PROFILE === 'debug' ? 'debug' : 'release';

  if (process.platform === 'win32') {
    return path.resolve(repoRoot, 'target', targetDir, 'napi_compiler.dll');
  }

  const ext = process.platform === 'darwin' ? 'dylib' : 'so';
  return path.resolve(repoRoot, 'target', targetDir, `libnapi_compiler.${ext}`);
}

const srcPath = nativeLibPath();
if (!fs.existsSync(srcPath)) {
  throw new Error(`Native library was not produced: ${srcPath}`);
}

const dstPath = path.resolve(packageRoot, 'compiler', 'native', 'svelte-rs2.node');
fs.copyFileSync(srcPath, dstPath);

console.log(`Copied ${srcPath} -> ${dstPath}`);
