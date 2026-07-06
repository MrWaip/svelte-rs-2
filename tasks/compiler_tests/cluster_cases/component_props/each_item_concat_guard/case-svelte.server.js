import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{ text: "t" }];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let badge = each_array[i];
		Badge($$renderer, { dataTestid: `badge-${$.stringify(badge.text)}` });
	}
	$$renderer.push(`<!--]-->`);
}
