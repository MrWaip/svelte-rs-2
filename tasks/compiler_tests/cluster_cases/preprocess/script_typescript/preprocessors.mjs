import { transform } from 'oxc-transform';

export default function preprocessors() {
  return {
    script: async ({ content, attributes, filename }) => {
      if (attributes.lang !== 'ts') return null;
      const result = await transform(filename ?? 'component.ts', content, {
        lang: 'ts',
        sourcemap: true,
        typescript: { onlyRemoveTypeImports: true }
      });
      return {
        code: result.code,
        map: result.map
      };
    }
  };
}
