import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let selected = $$props["selected"];
	let values = $$props["values"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(values);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let value = each_array[$$index];
		$$renderer.push(`<input type="checkbox"${$.attr("value", value)}${$.attr("checked", selected.includes(value), true)}/>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, {
		selected,
		values
	});
}
