import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const store = {
		items: ["a", "b"],
		data: {
			a: 1,
			b: 2
		}
	};
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(store.items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<input${$.attr("value", store.data[item])}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
