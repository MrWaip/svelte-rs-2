import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{ id: "a" }, { id: "b" }];
	let selected;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<input type="checkbox"${$.attr("value", item.id)}${$.attr("checked", selected.includes(item.id), true)}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
