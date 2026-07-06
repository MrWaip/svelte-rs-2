import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let promise = Promise.resolve(42);
	$.await($$renderer, promise, () => {}, (value) => {
		const doubled = value * 2;
		$$renderer.push(`<p>${$.escape(doubled)}</p>`);
	});
	$$renderer.push(`<!--]-->`);
}
