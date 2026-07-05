import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({ k0: 1 });
	let num = 0;
	$$renderer.push(`<button>inc</button> `);
	$.await($$renderer, p, () => {}, ({ [`k${num}`]: v }) => {
		$$renderer.push(`<button>${$.escape(v)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
