import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let rows = $$props["rows"];
	let key = $$props["key"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { [key]: value } = each_array[$$index];
		$$renderer.push(`<input${$.attr("value", value)}/>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, {
		rows,
		key
	});
}
