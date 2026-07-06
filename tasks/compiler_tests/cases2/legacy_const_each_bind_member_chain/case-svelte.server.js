import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const obj = {
		keys: ["a"],
		fields: { a: { value: 0 } }
	};
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(obj.keys);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let key = each_array[$$index];
		const field = obj.fields[key];
		$$renderer.push(`<input${$.attr("value", field.value)}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
