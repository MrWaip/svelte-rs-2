import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const promise = fetch("/api");
	$.await($$renderer, promise, () => {
		$$renderer.push(`<div class="loading"><span>Please wait...</span></div>`);
	}, (value) => {
		$$renderer.push(`<div class="result"><h1>Result</h1> <p>${$.escape(value)}</p></div>`);
	});
	$$renderer.push(`<!--]-->`);
}
