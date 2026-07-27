import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);

const UNSUPPORTED_THROW_OPTIONS = new Set(['ast', 'outputFilename']);
const UNSUPPORTED_WARN_OPTIONS = new Set(['modernAst']);

const native = require('./native/binding.cjs');

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
      line: diagnostic.startLine,
      column: diagnostic.startCol
    },
    end: {
      line: diagnostic.endLine,
      column: diagnostic.endCol
    },
    frame: diagnostic.frame ?? null
  };
}

function attachSourceContent(map, source) {
  if (
    map != null &&
    Array.isArray(map.sources) &&
    map.sources.length === 1 &&
    (!Array.isArray(map.sourcesContent) || map.sourcesContent[0] == null)
  ) {
    map.sourcesContent = [source];
  }
  return map;
}

function buildOutputs(nativeResult, source) {
  return {
    js:
      nativeResult.js == null
        ? null
        : {
            code: nativeResult.js.code,
            map: attachSourceContent(toSourceMap(nativeResult.js.map), source)
          },
    css:
      nativeResult.css == null
        ? null
        : {
            code: nativeResult.css.code,
            map: attachSourceContent(toSourceMap(nativeResult.css.map), source),
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
  const { withDiagnostics = false, warningFilter, source } = extra;
  const keepWarning = (warning) =>
    typeof warningFilter !== 'function' || Boolean(warningFilter(warning));
  const { js, css, metadata } = buildOutputs(nativeResult, source);

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
    warningFilter: options.warningFilter,
    source
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
    warningFilter: options.warningFilter,
    source
  });
}

function toMapJson(rawMap) {
  if (rawMap == null) return null;
  return typeof rawMap === 'string' ? rawMap : JSON.stringify(rawMap);
}

function attributesToObject(attributes) {
  const result = {};
  for (const attr of attributes) {
    result[attr.name] = attr.value == null ? true : attr.value;
  }
  return result;
}

async function applyHookToRegion(hook, region, code, map, filename, dependencies) {
  const processed = await hook({
    content: region.content,
    attributes: attributesToObject(region.attributes),
    markup: code,
    filename
  });
  if (!processed) return { code, map };

  if (Array.isArray(processed.dependencies)) {
    dependencies.push(...processed.dependencies);
  }

  const newContent = typeof processed.code === 'string' ? processed.code : region.content;
  const newMapJson = toMapJson(processed.map);
  if (newMapJson == null && newContent === region.content) {
    return { code, map };
  }

  const spliced = native.spliceRegion(
    code,
    map,
    region.contentStart,
    region.contentEnd,
    newContent,
    newMapJson,
    filename ?? '(unknown)'
  );
  return { code: spliced.code, map: spliced.map };
}

async function processMarkup(hook, code, map, filename, dependencies) {
  const processed = await hook({ content: code, filename });
  if (!processed) return { code, map };

  if (Array.isArray(processed.dependencies)) {
    dependencies.push(...processed.dependencies);
  }

  const newContent = typeof processed.code === 'string' ? processed.code : code;
  const newMapJson = toMapJson(processed.map);
  if (newMapJson == null && newContent === code) {
    return { code, map };
  }

  const documentEnd = Buffer.byteLength(code, 'utf-8');
  const spliced = native.spliceRegion(
    code,
    map,
    0,
    documentEnd,
    newContent,
    newMapJson,
    filename ?? '(unknown)'
  );
  return { code: spliced.code, map: spliced.map };
}

async function processScript(hook, code, map, filename, dependencies) {
  let regions = native.findPreprocessorRegions(code);
  for (const which of ['moduleScript', 'instanceScript']) {
    const scriptRegion = regions[which];
    if (scriptRegion == null) continue;

    const before = code;
    const result = await applyHookToRegion(
      hook,
      scriptRegion.region,
      code,
      map,
      filename,
      dependencies
    );
    code = result.code;
    if (code !== before) {
      regions = native.findPreprocessorRegions(code);
    }
    map = result.map;
  }
  return { code, map };
}

async function processStyle(hook, code, map, filename, dependencies) {
  const regions = native.findPreprocessorRegions(code);
  if (regions.style == null) return { code, map };
  return applyHookToRegion(hook, regions.style, code, map, filename, dependencies);
}

export async function preprocess(source, preprocessor, options = {}) {
  if (typeof source !== 'string') {
    throw new TypeError('preprocess(source, preprocessor, options): source must be a string');
  }

  const filename =
    options.filename ?? (!Array.isArray(preprocessor) ? preprocessor?.filename : undefined);
  const groups = preprocessor ? (Array.isArray(preprocessor) ? preprocessor : [preprocessor]) : [];

  let code = source;
  let map = null;
  const dependencies = [];

  for (const group of groups) {
    if (group.markup) {
      ({ code, map } = await processMarkup(group.markup, code, map, filename, dependencies));
    }
    if (group.script) {
      ({ code, map } = await processScript(group.script, code, map, filename, dependencies));
    }
    if (group.style) {
      ({ code, map } = await processStyle(group.style, code, map, filename, dependencies));
    }
  }

  return {
    code,
    map: map == null ? null : attachSourceContent(toSourceMap(native.sourceMapToJson(map)), source),
    dependencies: [...new Set(dependencies)],
    toString: () => code
  };
}
