import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve([[1, 2], [3, 4]]);
	$.await($$renderer, p, () => {}, ([[a, b], [c, d]]) => {
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
