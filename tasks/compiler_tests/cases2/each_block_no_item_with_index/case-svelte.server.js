import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let rank = 0, $$length = each_array.length; rank < $$length; rank++) {
		$$renderer.push(`<div>${$.escape(rank)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
}
