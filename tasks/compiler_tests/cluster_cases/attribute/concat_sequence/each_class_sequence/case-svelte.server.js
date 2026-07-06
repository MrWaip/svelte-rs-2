import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = $$props["items"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let item = each_array[i];
		$$renderer.push(`<div${$.attr_class(`${item.foo ? "foo" : ""} ${item.bar ? "bar" : ""}`)}>${$.escape(i + 1)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { items });
}
