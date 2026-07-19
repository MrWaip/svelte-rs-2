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

console.log('Smoke tests passed');
