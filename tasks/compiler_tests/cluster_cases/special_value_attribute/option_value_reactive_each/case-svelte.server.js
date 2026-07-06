import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	let items = $$props["items"];
	$$renderer.select({ value: foo }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.option({ value: item }, ($$renderer) => {
				$$renderer.push(`${$.escape(item.id)}`);
			});
		}
		$$renderer.push(`<!--]-->`);
	});
	$.bind_props($$props, {
		foo,
		items
	});
}
