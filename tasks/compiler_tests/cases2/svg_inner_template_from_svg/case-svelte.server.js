import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	$$renderer.push(`<svg><!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<circle${$.attr("cx", item * 10)}${$.attr("cy", 10)}${$.attr("r", 5)}></circle>`);
	}
	$$renderer.push(`<!--]--></svg>`);
}
