import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let url = "/api";
	let promise = $.derived(() => fetch(url));
	$.await($$renderer, promise(), () => {}, (value) => {
		$$renderer.push(`<p>${$.escape(value)}</p>`);
	});
	$$renderer.push(`<!--]-->`);
}
