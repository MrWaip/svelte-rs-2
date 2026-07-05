import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({ k1: 1 });
	let num = 0;
	$.await($$renderer, p, () => {}, ({ [`k${num++}`]: v }) => {
		$$renderer.push(`<button>${$.escape(v)} ${$.escape(num)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
