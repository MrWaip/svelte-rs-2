import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const promise = fetch("/api");
	$.await($$renderer, promise, () => {}, (value) => {
		$$renderer.push(`<p>${$.escape(value)}</p>`);
	});
	$$renderer.push(`<!--]-->`);
}
