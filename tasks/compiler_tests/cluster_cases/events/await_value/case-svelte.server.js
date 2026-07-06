import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let promise = Promise.resolve(() => {});
	$.await($$renderer, promise, () => {}, (handler) => {
		$$renderer.push(`<button>x</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
