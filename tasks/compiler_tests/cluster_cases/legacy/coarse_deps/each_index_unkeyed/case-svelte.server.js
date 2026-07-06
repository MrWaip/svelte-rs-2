import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = $$props["items"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
		let item = each_array[idx];
		const label = `${idx}:${item.name}`;
		$$renderer.push(`<p>${$.escape(label)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { items });
}
