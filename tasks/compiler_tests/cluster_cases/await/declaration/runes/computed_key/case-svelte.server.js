import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const k = "z";
	let p = Promise.resolve({ z: 1 });
	$.await($$renderer, p, () => {}, ({ [k]: v }) => {
		$$renderer.push(`<button>${$.escape(v)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
