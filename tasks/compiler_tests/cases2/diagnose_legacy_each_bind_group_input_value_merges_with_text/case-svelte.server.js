import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = $$props["items"];
	let value = $$props["value"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<label><span>${$.escape(item.label)}</span> <input type="radio"${$.attr("value", item.value)}${$.attr("checked", value === item.value, true)}/></label>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, {
		items,
		value
	});
}
