import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function flip() {}
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let n = each_array[$$index];
		const a = n;
		$$renderer.push(`<div></div>`);
	}
	$$renderer.push(`<!--]-->`);
}
