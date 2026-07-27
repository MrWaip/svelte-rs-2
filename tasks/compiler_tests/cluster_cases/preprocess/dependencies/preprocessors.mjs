import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';
import * as sass from 'sass';

const caseDir = path.dirname(fileURLToPath(import.meta.url));

export default function preprocessors() {
  return {
    style: ({ content, attributes }) => {
      if (attributes.lang !== 'scss') return null;
      const entryUrl = pathToFileURL(path.join(caseDir, 'component.svelte.scss'));
      const result = sass.compileString(content, {
        sourceMap: true,
        sourceMapIncludeSources: true,
        loadPaths: [caseDir],
        url: entryUrl
      });
      const dependencies = result.loadedUrls
        .map((url) => fileURLToPath(url))
        .filter((file) => file !== fileURLToPath(entryUrl))
        .map((file) => path.relative(caseDir, file));
      return {
        code: result.css,
        map: result.sourceMap,
        dependencies
      };
    }
  };
}
