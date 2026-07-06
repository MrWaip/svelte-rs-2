import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = ["a", "b"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
		let item = each_array[idx];
		$$renderer.push(`<p>${$.escape(item)} ${$.escape(idx)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
