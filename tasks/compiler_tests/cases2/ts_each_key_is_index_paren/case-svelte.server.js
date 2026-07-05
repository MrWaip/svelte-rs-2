import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [
		"a",
		"b",
		"c"
	];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let item = each_array[i];
		$$renderer.push(`<p>${$.escape(i)}: ${$.escape(item)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
