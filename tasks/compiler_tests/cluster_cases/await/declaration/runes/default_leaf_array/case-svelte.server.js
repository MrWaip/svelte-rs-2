import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve([1]);
	$.await($$renderer, p, () => {}, ([a = 10, b = 20]) => {
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
