import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let rows = $$props["rows"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let [first, second, ...[third, ...{ length }]] = each_array[$$index];
		$$renderer.push(`<span>${$.escape(first)}${$.escape(second)}</span> <input${$.attr("value", third)}/> <input${$.attr("value", length)}/>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { rows });
}
