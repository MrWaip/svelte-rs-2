import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let data = {
		a: 1,
		b: []
	};
	let items = ["x", "y"];
	$$renderer.push(`<input type="radio"${$.attr("checked", data.a === 1, true)}${$.attr("value", 1)}/> <input type="radio"${$.attr("checked", data.a === 2, true)}${$.attr("value", 2)}/> <!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<input type="checkbox"${$.attr("checked", data.b.includes(item), true)}${$.attr("value", item)}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
