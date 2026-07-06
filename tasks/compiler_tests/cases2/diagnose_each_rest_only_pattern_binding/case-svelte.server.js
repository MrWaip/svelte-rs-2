import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = $.fallback($$props["items"], () => [], true);
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { ...item } = each_array[$$index];
		$$renderer.push(`<span>${$.escape(item.x)}</span>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { items });
}
