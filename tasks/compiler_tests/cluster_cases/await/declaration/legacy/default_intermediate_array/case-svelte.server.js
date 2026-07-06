import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve([[1, 2], 3]);
	$.await($$renderer, p, () => {}, ([[a, b] = [8, 9], c]) => {
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
