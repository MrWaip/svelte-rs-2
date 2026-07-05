import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.reject({ k1: 1 });
	let num = 0;
	$.await($$renderer, p, () => {}, (x) => {
		$$renderer.push(`ok`);
	});
	$$renderer.push(`<!--]-->`);
}
