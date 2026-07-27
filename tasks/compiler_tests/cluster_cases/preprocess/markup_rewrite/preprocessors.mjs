import { marked } from 'marked';

export default function preprocessors() {
  return {
    markup: ({ content }) => ({ code: marked.parse(content) })
  };
}
