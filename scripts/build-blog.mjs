import { readdirSync, readFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { join, basename } from 'node:path';
import { marked } from 'marked';

const SRC = 'blog';
const OUT = join('playground', 'blog');

const SHELL = (title, body) => `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>${title}</title>
<style>
:root { color-scheme: light dark; --fg: #1a1a1a; --bg: #fdfdfc; --muted: #666; --line: #e2e2df; --code-bg: #f4f4f2; --link: #0b62c4; }
@media (prefers-color-scheme: dark) {
  :root { --fg: #e6e6e3; --bg: #16181a; --muted: #9a9a95; --line: #2c2f33; --code-bg: #1e2124; --link: #74b0ff; }
}
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--fg);
  font: 18px/1.65 -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; }
main { max-width: 44rem; margin: 0 auto; padding: 3rem 1.25rem 6rem; }
h1 { font-size: 2rem; line-height: 1.2; margin: 0 0 1.5rem; letter-spacing: -0.02em; }
h2 { font-size: 1.35rem; margin: 3rem 0 1rem; letter-spacing: -0.01em; }
h1 + p { color: var(--muted); font-size: 1.1rem; }
p, ul, ol { margin: 0 0 1.25rem; }
li { margin-bottom: 0.35rem; }
a { color: var(--link); }
hr { border: 0; border-top: 1px solid var(--line); margin: 2.5rem 0; }
code { background: var(--code-bg); padding: 0.1em 0.35em; border-radius: 3px; font-size: 0.88em;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
pre { background: var(--code-bg); padding: 1rem 1.1rem; border-radius: 6px; overflow-x: auto;
  border: 1px solid var(--line); font-size: 0.85rem; line-height: 1.5; }
pre code { background: none; padding: 0; font-size: inherit; }
strong { font-weight: 650; }
footer { margin-top: 4rem; padding-top: 1.5rem; border-top: 1px solid var(--line);
  color: var(--muted); font-size: 0.9rem; }
</style>
</head>
<body>
<main>
${body}
<footer><a href="/svelte-rs/">Try the compiler in the playground</a> &middot; <a href="https://github.com/MrWaip/svelte-rs">Source on GitHub</a></footer>
</main>
</body>
</html>
`;

mkdirSync(OUT, { recursive: true });

const files = readdirSync(SRC).filter((f) => f.endsWith('.md'));

for (const file of files) {
	const md = readFileSync(join(SRC, file), 'utf8');
	const heading = md.match(/^#\s+(.+)$/m);
	const title = heading ? heading[1].trim() : basename(file, '.md');
	let first = true;
	const renderer = {
		heading({ tokens, depth }) {
			const text = this.parser.parseInline(tokens);
			if (depth === 1 && first) {
				first = false;
				return `<h1>${text}</h1>\n`;
			}
			const level = Math.min(depth + 1, 6);
			return `<h${level}>${text}</h${level}>\n`;
		},
	};
	marked.use({ renderer });

	const html = SHELL(title, marked.parse(md));
	const out = join(OUT, basename(file, '.md') + '.html');
	writeFileSync(out, html);
	console.log('blog:', file, '->', out);
}
