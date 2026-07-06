import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({
		p: { a: 1 },
		q: { b: 2 }
	});
	$.await($$renderer, p, () => {}, ({ p: { a }, q: { b } }) => {
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
