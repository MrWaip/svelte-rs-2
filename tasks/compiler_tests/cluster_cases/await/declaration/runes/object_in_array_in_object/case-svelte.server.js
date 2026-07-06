import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({ outer: [{ inner: 1 }] });
	$.await($$renderer, p, () => {}, ({ outer: [{ inner }] }) => {
		$$renderer.push(`<button>${$.escape(inner)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
