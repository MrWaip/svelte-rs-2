import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const promise = fetch("/api");
	$.await($$renderer, promise, () => {}, ([a, b]) => {
		$$renderer.push(`<p>${$.escape(a)} and ${$.escape(b)}</p>`);
	});
	$$renderer.push(`<!--]-->`);
}
