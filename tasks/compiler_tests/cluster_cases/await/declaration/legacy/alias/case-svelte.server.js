import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({
		a: 1,
		b: 2
	});
	$.await($$renderer, p, () => {}, ({ a: x, b: y }) => {
		$$renderer.push(`<button>${$.escape(x)}${$.escape(y)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
