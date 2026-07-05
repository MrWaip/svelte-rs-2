import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let list = ["Hello"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(list);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<input${$.attr("value", item)}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
