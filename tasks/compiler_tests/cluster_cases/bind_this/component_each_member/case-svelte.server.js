import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer, $$props) {
	let items = $.fallback($$props["items"], () => [{ ref: null }], true);
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		Comp($$renderer, {});
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { items });
}
