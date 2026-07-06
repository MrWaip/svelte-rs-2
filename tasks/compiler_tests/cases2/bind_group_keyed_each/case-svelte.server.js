import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{
		id: 1,
		name: "a"
	}, {
		id: 2,
		name: "b"
	}];
	let selected = [];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<input type="checkbox"${$.attr("checked", selected.includes(item.name), true)}${$.attr("value", item.name)}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
