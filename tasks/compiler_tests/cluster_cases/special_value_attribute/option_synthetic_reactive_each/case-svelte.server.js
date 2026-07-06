import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let selected = $$props["selected"];
	let items = $$props["items"];
	$$renderer.select({ value: selected }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.option({}, item);
		}
		$$renderer.push(`<!--]-->`);
	});
	$.bind_props($$props, {
		selected,
		items
	});
}
