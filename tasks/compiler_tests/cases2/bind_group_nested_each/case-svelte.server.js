import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let groups = [["a", "b"], ["c", "d"]];
	let selected = [];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(groups);
	for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
		let group = each_array[$$index_1];
		$$renderer.push(`<!--[-->`);
		const each_array_1 = $.ensure_array_like(group);
		for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
			let item = each_array_1[$$index];
			$$renderer.push(`<input type="checkbox"${$.attr("checked", selected.includes(item), true)}${$.attr("value", item)}/>`);
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
}
