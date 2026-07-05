import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({});
	let num = 0;
	$$renderer.push(`<button>inc</button> `);
	$.await($$renderer, p, () => {}, ({ v = num }) => {
		$$renderer.push(`<button>${$.escape(v)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
