import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let categories = [{
		id: 1,
		name: "fruit",
		selected: []
	}, {
		id: 2,
		name: "veg",
		selected: []
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(categories);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let category = each_array[$$index];
		$$renderer.push(`<input type="checkbox"${$.attr("checked", category.selected.includes("apple"), true)} value="apple"/>`);
	}
	$$renderer.push(`<!--]-->`);
}
