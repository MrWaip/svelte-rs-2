import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);

const UNSUPPORTED_THROW_OPTIONS = new Set(['ast', 'sourcemap', 'outputFilename']);
const UNSUPPORTED_WARN_OPTIONS = new Set(['modernAst']);

function loadNativeAddon() {
  const localPath = path.resolve(
    path.dirname(new URL(import.meta.url).pathname),
    './native/svelte-rs2.node'
  );

  if (fs.existsSync(localPath)) {
    return require(localPath);
  }

  throw new Error(
    'Native addon was not found at packages/svelte-rs2/compiler/native/svelte-rs2.node. Build crates/napi_compiler and place the .node artifact there.'
  );
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

function normalizeGenerate(value) {
  if (value === false || value === 'false') return 'false';
  if (value === 'server') return 'server';
  return 'client';
}

function normalizeCompileOptions(options = {}) {
  assertSupportedOptions(options);

  return {
    dev: Boolean(options.dev),
    filename: typeof options.filename === 'string' ? options.filename : '(unknown)',
    root_dir: typeof options.rootDir === 'string' ? options.rootDir : undefined,
    name: typeof options.name === 'string' ? options.name : undefined,
    custom_element: Boolean(options.customElement),
    namespace:
      options.namespace === 'svg' || options.namespace === 'mathml' ? options.namespace : 'html',
    css: options.css === 'injected' ? 'injected' : 'external',
    runes: typeof options.runes === 'boolean' ? options.runes : undefined,
    preserve_comments: Boolean(options.preserveComments),
    preserve_whitespace: Boolean(options.preserveWhitespace),
    disclose_version:
      typeof options.discloseVersion === 'boolean' ? options.discloseVersion : undefined,
    hmr: Boolean(options.hmr),
    accessors: Boolean(options.accessors),
    immutable: Boolean(options.immutable),
    compatibility_component_api:
      typeof options.compatibility?.componentApi === 'number'
        ? options.compatibility.componentApi
        : undefined,
    experimental_async: Boolean(options.experimental?.async),
    generate: normalizeGenerate(options.generate)
  };
}

function normalizeModuleCompileOptions(options = {}) {
  assertSupportedOptions(options);

  return {
    dev: Boolean(options.dev),
    filename: typeof options.filename === 'string' ? options.filename : '(unknown)',
    root_dir: typeof options.rootDir === 'string' ? options.rootDir : undefined,
    generate: normalizeGenerate(options.generate)
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

function normalizeCompileResponse(nativeResult, filename, optionWarnings = []) {
  const warnings = [...optionWarnings];
  const errors = [];

  for (const diagnostic of nativeResult.diagnostics ?? []) {
    const normalized = normalizeDiagnostic(diagnostic, filename ?? null);
    if (diagnostic.severity === 'Error') {
      errors.push(normalized);
    } else {
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

  return {
    js: nativeResult.js == null ? null : { code: nativeResult.js, map: null },
    css:
      nativeResult.css == null
        ? null
        : {
            code: nativeResult.css,
            map: null,
            hasGlobal: null
          },
    warnings,
    metadata: {
      canary: true,
      hasCss: nativeResult.css != null,
      unsupported: {
        ast: 'not_returned',
        sourceMap: 'always_null',
        unsupportedOptions: ['ast', 'sourcemap', 'outputFilename']
      }
    },
    ast: null
  };
}

export function compile(source, options = {}) {
  if (typeof source !== 'string') {
    throw new TypeError('compile(source, options): source must be a string');
  }

  const normalizedOptions = normalizeCompileOptions(options);
  const optionWarnings = collectOptionWarnings(options);
  const nativeResult = native.compile(source, normalizedOptions);
  return normalizeCompileResponse(nativeResult, normalizedOptions.filename, optionWarnings);
}

export function compileModule(source, options = {}) {
  if (typeof source !== 'string') {
    throw new TypeError('compileModule(source, options): source must be a string');
  }

  const normalizedOptions = normalizeModuleCompileOptions(options);
  const optionWarnings = collectOptionWarnings(options);
  const nativeResult = native.compileModule(source, normalizedOptions);
  return normalizeCompileResponse(nativeResult, normalizedOptions.filename, optionWarnings);
}
