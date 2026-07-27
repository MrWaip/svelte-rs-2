const api = await import('@mrwaip/svelte-rs/compiler');

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
if (compileResult.ast !== null) {
  throw new Error('compile result ast must be null in canary');
}

const warned = api.compile('<h1>ok</h1>', {
  filename: 'warn.svelte',
  modernAst: true
});
if (!warned.warnings.some((warning) => warning.code === 'unsupported_option_ignored')) {
  throw new Error('modernAst must produce unsupported_option_ignored warning');
}

let unsupportedOptionError = null;
try {
  api.compile('<h1>bad</h1>', {
    filename: 'bad.svelte',
    ast: true
  });
} catch (error) {
  unsupportedOptionError = error;
}
if (!(unsupportedOptionError instanceof Error)) {
  throw new Error('unsupported compile option must throw');
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

let typeError = null;
try {
  api.compileModule(123, { filename: 'mod.svelte.js' });
} catch (error) {
  typeError = error;
}
if (!(typeError instanceof TypeError)) {
  throw new Error('compileModule must throw TypeError for non-string source');
}

const scriptOnly = await api.preprocess(
  '<script lang="ts">const x: number = 1;</script><h1>hi</h1>',
  {
    script: ({ content, attributes }) => {
      if (attributes.lang !== 'ts') throw new Error('expected lang=ts attribute');
      return { code: content.replace('const x: number = 1;', 'const x = 1;') };
    }
  },
  { filename: 'Preprocess.svelte' }
);
if (!scriptOnly.code.includes('const x = 1;')) {
  throw new Error('preprocess must apply the script hook');
}
if (!scriptOnly.code.includes('<h1>hi</h1>')) {
  throw new Error('preprocess must leave markup outside the script tag untouched');
}
if (typeof scriptOnly.toString !== 'function' || scriptOnly.toString() !== scriptOnly.code) {
  throw new Error('preprocess result must implement toString()');
}
if (!Array.isArray(scriptOnly.dependencies)) {
  throw new Error('preprocess result must contain a dependencies array');
}

const styleAndDeps = await api.preprocess(
  '<style lang="scss">.a { .b { color: red; } }</style>',
  {
    style: ({ content }) => ({
      code: content.replace('.a { .b { color: red; } }', '.a .b { color: red; }'),
      dependencies: ['tokens.scss']
    })
  },
  { filename: 'Style.svelte' }
);
if (!styleAndDeps.code.includes('.a .b { color: red; }')) {
  throw new Error('preprocess must apply the style hook');
}
if (!styleAndDeps.dependencies.includes('tokens.scss')) {
  throw new Error('preprocess must collect hook-reported dependencies');
}

const markupThenScript = await api.preprocess(
  '<script>let x = 1;</script>',
  [
    { markup: ({ content }) => ({ code: content.replace('let x = 1;', 'let x = 1;\n') }) },
    { script: ({ content }) => ({ code: content.replace('let x = 1;', 'let x = 2;') }) }
  ],
  { filename: 'Chained.svelte' }
);
if (!markupThenScript.code.includes('let x = 2;')) {
  throw new Error('preprocess must run multiple groups in sequence');
}

const noHooks = await api.preprocess('<h1>ok</h1>', {}, { filename: 'NoHooks.svelte' });
if (noHooks.code !== '<h1>ok</h1>' || noHooks.map !== null) {
  throw new Error('preprocess with no matching hooks must return the source unchanged');
}

const compiledAfterPreprocess = api.compile(scriptOnly.code, { filename: 'Preprocess.svelte' });
if (!compiledAfterPreprocess.js || typeof compiledAfterPreprocess.js.code !== 'string') {
  throw new Error('compile must succeed on preprocess output');
}

console.log('Smoke tests passed');
