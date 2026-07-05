import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve([{ a: 1 }, { b: 2 }]);
	$.await($$renderer, p, () => {}, ([{ a }, { b }]) => {
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
