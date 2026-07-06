import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let filtered;
	let items = [{ done: false }];
	$: filtered = items;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(filtered);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<p>${$.escape(item.done)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
