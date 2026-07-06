import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({});
	$.await($$renderer, p, () => {}, ({ p: { a } = {} }) => {
		$$renderer.push(`<button>${$.escape(a)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
