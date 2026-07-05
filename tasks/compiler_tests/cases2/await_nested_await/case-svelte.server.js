import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const outer = fetch("/api/list");
	$.await($$renderer, outer, () => {}, (items) => {
		$.await($$renderer, items[0], () => {}, (detail) => {
			$$renderer.push(`<p>${$.escape(detail)}</p>`);
		});
		$$renderer.push(`<!--]-->`);
	});
	$$renderer.push(`<!--]-->`);
}
