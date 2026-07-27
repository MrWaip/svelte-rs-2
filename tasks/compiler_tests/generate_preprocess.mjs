import { readFileSync } from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { preprocess } from 'svelte/compiler';

const inputPath = process.env.INPUT_FILE || '/dev/stdin';
const caseDirs = JSON.parse(readFileSync(inputPath, 'utf8'));

const results = {};
for (const dir of caseDirs) {
  const name = path.basename(dir);
  const source = readFileSync(path.join(dir, 'component.svelte'), 'utf-8');
  const preprocessorsUrl = pathToFileURL(path.join(dir, 'preprocessors.mjs'));
  const { default: createPreprocessors } = await import(preprocessorsUrl);

  const result = await preprocess(source, createPreprocessors(), { filename: `${name}.svelte` });

  results[dir] = {
    code: result.code,
    dependencies: [...result.dependencies].sort()
  };
}

console.log(JSON.stringify(results));
