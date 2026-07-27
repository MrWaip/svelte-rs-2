import * as sass from 'sass';

export default function preprocessors() {
  return {
    style: ({ content, attributes, filename }) => {
      if (attributes.lang !== 'scss') return null;
      const result = sass.compileString(content, {
        sourceMap: true,
        sourceMapIncludeSources: true,
        url: new URL(`file:///${filename ?? 'component.scss'}`)
      });
      return {
        code: result.css,
        map: result.sourceMap
      };
    }
  };
}
