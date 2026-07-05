import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let arr = [
		1,
		2,
		3
	];
	let elements = [];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(arr);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let item = each_array[i];
		$$renderer.push(`<b>${$.escape(item)}</b>`);
	}
	$$renderer.push(`<!--]-->`);
}
