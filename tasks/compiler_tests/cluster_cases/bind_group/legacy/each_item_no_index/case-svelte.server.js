import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let selected_array = $$props["selected_array"];
	let values = $$props["values"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(selected_array);
	for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
		let selected = each_array[$$index_1];
		$$renderer.push(`<!--[-->`);
		const each_array_1 = $.ensure_array_like(values);
		for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
			let value = each_array_1[$$index];
			$$renderer.push(`<input type="checkbox"${$.attr("value", value)}${$.attr("checked", selected.includes(value), true)}/>`);
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, {
		selected_array,
		values
	});
}
