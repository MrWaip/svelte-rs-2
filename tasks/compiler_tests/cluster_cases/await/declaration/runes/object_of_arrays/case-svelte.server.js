import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({
		p: [1, 2],
		q: [3, 4]
	});
	$.await($$renderer, p, () => {}, ({ p: [a, b], q: [c, d] }) => {
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
