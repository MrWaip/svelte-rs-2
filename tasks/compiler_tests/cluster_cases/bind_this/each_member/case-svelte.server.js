import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let visible = $.fallback($$props["visible"], false);
	let items = $.fallback($$props["items"], () => [{
		value: "a",
		ref: null
	}], true);
	if (visible) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<div>${$.escape(item.value)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, {
		visible,
		items
	});
}
