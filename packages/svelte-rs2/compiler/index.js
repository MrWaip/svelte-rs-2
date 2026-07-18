import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);

const UNSUPPORTED_THROW_OPTIONS = new Set(['ast', 'outputFilename']);
const UNSUPPORTED_WARN_OPTIONS = new Set(['modernAst']);

const PLATFORM_PACKAGE_BY_TARGET = {
  'darwin-arm64': '@mrwaip/svelte-rs2-darwin-arm64',
  'darwin-x64': '@mrwaip/svelte-rs2-darwin-x64',
  'linux-x64': '@mrwaip/svelte-rs2-linux-x64-gnu',
  'linux-arm64': '@mrwaip/svelte-rs2-linux-arm64-gnu'
};

function localNativeAddonPath() {
  const currentFile = fileURLToPath(import.meta.url);
  return path.resolve(
    path.dirname(currentFile),
    './native/svelte-rs2.node'
  );
}

function loadFromLocalDevelopmentPath() {
  const localPath = localNativeAddonPath();
  if (!fs.existsSync(localPath)) {
    return null;
  }
  return require(localPath);
}

function loadFromPlatformPackage() {
  const target = `${process.platform}-${process.arch}`;
  const pkg = PLATFORM_PACKAGE_BY_TARGET[target];

  if (!pkg) {
    throw new Error(
      `Unsupported platform for @mrwaip/svelte-rs2/compiler: ${target}. ` +
        'Supported targets: darwin-arm64, darwin-x64, linux-x64, linux-arm64.'
    );
  }

  try {
    return require(pkg);
  } catch (error) {
    throw new Error(
      `Native addon package ${pkg} is not installed. ` +
        'Reinstall dependencies for this platform or use optionalDependencies-aware install.',
      { cause: error }
    );
  }
}

function loadNativeAddon() {
  const local = loadFromLocalDevelopmentPath();
  if (local) {
    return local;
  }

  return loadFromPlatformPackage();
}

const native = loadNativeAddon();

function assertSupportedOptions(options) {
  for (const key of Object.keys(options)) {
    if (UNSUPPORTED_THROW_OPTIONS.has(key)) {
      throw new Error(`Unsupported option in canary: ${key}`);
    }
  }
}

function collectOptionWarnings(options) {
  const warnings = [];
  for (const key of Object.keys(options)) {
    if (UNSUPPORTED_WARN_OPTIONS.has(key)) {
      warnings.push({
        code: 'unsupported_option_ignored',
        message: `Option ${key} is ignored in canary`,
        filename: options.filename ?? null,
        start: null,
        end: null,
        frame: null
      });
    }
  }
  return warnings;
}

class SourceMap {
  constructor(parsed) {
    Object.assign(this, parsed);
  }

  toString() {
    return JSON.stringify(this);
  }

  toUrl() {
    return (
      'data:application/json;charset=utf-8;base64,' +
      Buffer.from(this.toString(), 'utf-8').toString('base64')
    );
  }
}

function toSourceMap(raw) {
  if (raw == null) {
    return null;
  }
  const parsed = typeof raw === 'string' ? JSON.parse(raw) : raw;
  return new SourceMap(parsed);
}

function normalizeGenerate(value) {
  if (value === false || value === 'false') return 'false';
  if (value === 'server') return 'server';
  return 'client';
}

function normalizeInputMap(value) {
  if (value == null) return undefined;
  if (typeof value === 'string') return value;
  return JSON.stringify(value);
}

function normalizeSuppress(value) {
  if (!Array.isArray(value)) return undefined;
  const codes = value.filter((code) => typeof code === 'string');
  return codes.length ? codes : undefined;
}

function normalizeCompileOptions(options = {}) {
  assertSupportedOptions(options);

  return {
    dev: Boolean(options.dev),
    filename: typeof options.filename === 'string' ? options.filename : '(unknown)',
    rootDir: typeof options.rootDir === 'string' ? options.rootDir : undefined,
    name: typeof options.name === 'string' ? options.name : undefined,
    customElement: Boolean(options.customElement),
    namespace:
      options.namespace === 'svg' || options.namespace === 'mathml' ? options.namespace : 'html',
    css: options.css === 'injected' ? 'injected' : 'external',
    runes: typeof options.runes === 'boolean' ? options.runes : undefined,
    preserveComments: Boolean(options.preserveComments),
    preserveWhitespace: Boolean(options.preserveWhitespace),
    discloseVersion:
      typeof options.discloseVersion === 'boolean' ? options.discloseVersion : undefined,
    hmr: Boolean(options.hmr),
    accessors: Boolean(options.accessors),
    immutable: Boolean(options.immutable),
    compatibilityComponentApi:
      typeof options.compatibility?.componentApi === 'number'
        ? options.compatibility.componentApi
        : undefined,
    experimentalAsync: Boolean(options.experimental?.async),
    generate: normalizeGenerate(options.generate),
    sourcemap: normalizeInputMap(options.sourcemap),
    suppress: normalizeSuppress(options.suppress)
  };
}

function normalizeModuleCompileOptions(options = {}) {
  assertSupportedOptions(options);

  return {
    dev: Boolean(options.dev),
    filename: typeof options.filename === 'string' ? options.filename : '(unknown)',
    rootDir: typeof options.rootDir === 'string' ? options.rootDir : undefined,
    generate: normalizeGenerate(options.generate),
    sourcemap: normalizeInputMap(options.sourcemap),
    suppress: normalizeSuppress(options.suppress)
  };
}

function normalizeDiagnostic(diagnostic, filenameFallback) {
  return {
    code: diagnostic.code,
    message: diagnostic.message,
    filename: filenameFallback,
    start: {
      line: diagnostic.start_line,
      column: diagnostic.start_col
    },
    end: {
      line: diagnostic.end_line,
      column: diagnostic.end_col
    },
    frame: diagnostic.frame ?? null
  };
}

function buildOutputs(nativeResult) {
  return {
    js:
      nativeResult.js == null
        ? null
        : {
            code: nativeResult.js.code,
            map: toSourceMap(nativeResult.js.map)
          },
    css:
      nativeResult.css == null
        ? null
        : {
            code: nativeResult.css.code,
            map: toSourceMap(nativeResult.css.map),
            hasGlobal: nativeResult.css.hasGlobal ?? null
          },
    metadata: {
      canary: true,
      hasCss: nativeResult.css != null,
      unsupported: {
        ast: 'not_returned',
        unsupportedOptions: ['ast', 'outputFilename']
      }
    }
  };
}

function normalizeCompileResponse(nativeResult, filename, optionWarnings = [], extra = {}) {
  const { withDiagnostics = false, warningFilter } = extra;
  const keepWarning = (warning) =>
    typeof warningFilter !== 'function' || Boolean(warningFilter(warning));
  const { js, css, metadata } = buildOutputs(nativeResult);

  if (withDiagnostics) {
    const diagnostics = [];
    for (const warning of optionWarnings) {
      const normalized = { ...warning, severity: 'warning' };
      if (keepWarning(normalized)) diagnostics.push(normalized);
    }
    for (const diagnostic of nativeResult.diagnostics ?? []) {
      const severity = diagnostic.severity === 'Error' ? 'error' : 'warning';
      const normalized = { ...normalizeDiagnostic(diagnostic, filename ?? null), severity };
      if (severity === 'warning' && !keepWarning(normalized)) continue;
      diagnostics.push(normalized);
    }
    return { js, css, diagnostics, metadata, ast: null };
  }

  const warnings = optionWarnings.filter(keepWarning);
  const errors = [];

  for (const diagnostic of nativeResult.diagnostics ?? []) {
    const normalized = normalizeDiagnostic(diagnostic, filename ?? null);
    if (diagnostic.severity === 'Error') {
      errors.push(normalized);
    } else if (keepWarning(normalized)) {
      warnings.push(normalized);
    }
  }

  if (errors.length > 0) {
    const error = new Error(errors[0].message || 'Compilation failed');
    error.code = errors[0].code;
    error.warnings = warnings;
    error.diagnostics = errors;
    throw error;
  }

  return { js, css, warnings, metadata, ast: null };
}

export function compile(source, options = {}) {
  if (typeof source !== 'string') {
    throw new TypeError('compile(source, options): source must be a string');
  }

  const normalizedOptions = normalizeCompileOptions(options);
  const optionWarnings = collectOptionWarnings(options);
  const nativeResult = native.compile(source, normalizedOptions);
  return normalizeCompileResponse(nativeResult, normalizedOptions.filename, optionWarnings, {
    withDiagnostics: Boolean(options.withDiagnostics),
    warningFilter: options.warningFilter
  });
}

export function compileModule(source, options = {}) {
  if (typeof source !== 'string') {
    throw new TypeError('compileModule(source, options): source must be a string');
  }

  const normalizedOptions = normalizeModuleCompileOptions(options);
  const optionWarnings = collectOptionWarnings(options);
  const nativeResult = native.compileModule(source, normalizedOptions);
  return normalizeCompileResponse(nativeResult, normalizedOptions.filename, optionWarnings, {
    withDiagnostics: Boolean(options.withDiagnostics),
    warningFilter: options.warningFilter
  });
}
