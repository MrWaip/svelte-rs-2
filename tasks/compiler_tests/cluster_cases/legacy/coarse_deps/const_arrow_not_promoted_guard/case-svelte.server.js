import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = $$props["items"];
		const makeEmpty = () => ({ a: "" });
		$: if (items) {
			items = items.map((x) => ({
				...x,
				info: x.info ?? makeEmpty()
			}));
		}
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<input${$.attr("value", item.a)}/>`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { items });
	});
}
