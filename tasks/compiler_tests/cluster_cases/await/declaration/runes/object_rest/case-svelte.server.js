import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({
		a: 1,
		b: 2,
		c: 3
	});
	$.await($$renderer, p, () => {}, ({ a, ...rest }) => {
		$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.b)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
