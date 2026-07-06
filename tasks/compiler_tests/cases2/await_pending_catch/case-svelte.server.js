import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const promise = fetch("/api");
	$.await($$renderer, promise, () => {
		$$renderer.push(`<p>Loading...</p>`);
	}, () => {});
	$$renderer.push(`<!--]-->`);
}
