import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve([
		1,
		2,
		3
	]);
	$.await($$renderer, p, () => {}, ([a, ...rest]) => {
		$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.length)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
