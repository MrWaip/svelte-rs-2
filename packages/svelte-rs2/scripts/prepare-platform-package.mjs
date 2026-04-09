import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const packageRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(packageRoot, '..', '..');

const PLATFORM_PACKAGE_BY_TARGET = {
  'darwin-arm64': 'packages/svelte-rs2-darwin-arm64',
  'darwin-x64': 'packages/svelte-rs2-darwin-x64',
  'linux-x64': 'packages/svelte-rs2-linux-x64-gnu'
};

function currentTarget() {
  return `${process.platform}-${process.arch}`;
}

function nativeLibPath() {
  const targetDir = process.env.NAPI_BUILD_PROFILE === 'debug' ? 'debug' : 'release';

  if (process.platform === 'win32') {
    return path.resolve(repoRoot, 'target', targetDir, 'napi_compiler.dll');
  }

  const ext = process.platform === 'darwin' ? 'dylib' : 'so';
  return path.resolve(repoRoot, 'target', targetDir, `libnapi_compiler.${ext}`);
}

function outputPackageDir() {
  const target = currentTarget();
  const relPath = PLATFORM_PACKAGE_BY_TARGET[target];
  if (!relPath) {
    throw new Error(
      `Unsupported target for platform package preparation: ${target}. ` +
        'Supported targets: darwin-arm64, darwin-x64, linux-x64.'
    );
  }
  return path.resolve(repoRoot, relPath);
}

const srcPath = nativeLibPath();
if (!fs.existsSync(srcPath)) {
  throw new Error(`Native library was not produced: ${srcPath}`);
}

const pkgDir = outputPackageDir();
const dstPath = path.resolve(pkgDir, 'svelte-rs2.node');
fs.copyFileSync(srcPath, dstPath);

console.log(`Copied ${srcPath} -> ${dstPath}`);
