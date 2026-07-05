import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = $$props["items"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		const { button } = item;
		$$renderer.push(`<a${$.attr("href", button.href)}>${$.escape(button.text)}</a>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { items });
}
