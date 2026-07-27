import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let rows = $$props["rows"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { name: [first] } = each_array[$$index];
		$$renderer.push(`<input${$.attr("value", first)}/>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { rows });
}
