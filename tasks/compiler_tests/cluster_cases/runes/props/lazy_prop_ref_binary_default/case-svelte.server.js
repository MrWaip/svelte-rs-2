import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a, b = a, c = b * b } = $$props;
	$$renderer.push(`<p>${$.escape(a)}${$.escape(b)}${$.escape(c)}</p>`);
}
