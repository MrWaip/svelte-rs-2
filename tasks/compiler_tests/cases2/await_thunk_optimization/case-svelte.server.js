import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$.await($$renderer, fetch(), () => {}, (result) => {
		$$renderer.push(`<p>${$.escape(result)}</p>`);
	});
	$$renderer.push(`<!--]-->`);
}
