import { transform } from 'oxc-transform';
import * as sass from 'sass';

export default function preprocessors() {
  return [
    {
      script: async ({ content, attributes, filename }) => {
        if (attributes.lang !== 'ts') return null;
        const result = await transform(filename ?? 'component.ts', content, {
          lang: 'ts',
          sourcemap: true,
          typescript: { onlyRemoveTypeImports: true }
        });
        return { code: result.code, map: result.map };
      }
    },
    {
      style: ({ content, attributes, filename }) => {
        if (attributes.lang !== 'scss') return null;
        const result = sass.compileString(content, {
          sourceMap: true,
          sourceMapIncludeSources: true,
          url: new URL(`file:///${filename ?? 'component.scss'}`)
        });
        return { code: result.css, map: result.sourceMap };
      }
    }
  ];
}
