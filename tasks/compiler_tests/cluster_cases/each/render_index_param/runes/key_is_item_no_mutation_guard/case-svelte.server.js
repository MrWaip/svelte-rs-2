import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{ name: "a" }];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<p>${$.escape(item.name)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
