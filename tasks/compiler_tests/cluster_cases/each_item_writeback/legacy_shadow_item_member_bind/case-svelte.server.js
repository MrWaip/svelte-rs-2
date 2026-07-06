import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let rows = [{ v: 1 }];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let rows = each_array[$$index];
		$$renderer.push(`<input${$.attr("value", rows.v)}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
