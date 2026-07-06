import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		function row($$renderer, v) {
			$$renderer.push(`<p>${$.escape(v)}</p>`);
		}
		row($$renderer, item);
	}
	$$renderer.push(`<!--]-->`);
}
