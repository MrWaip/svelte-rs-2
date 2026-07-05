import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{ value: "x" }];
	function keep(it) {
		return true;
	}
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items.filter(keep));
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<input${$.attr("value", item.value)}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
