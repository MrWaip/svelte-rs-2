import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	let list = $$props["list"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(list || []);
	for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
		let item = each_array[idx];
		Child($$renderer, { label: `ID (${$.stringify(idx + 1)})` });
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { list });
}
