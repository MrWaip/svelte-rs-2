import * as $ from "svelte/internal/server";
import Sticker from "./Sticker.svelte";
export default function App($$renderer, $$props) {
	let items = $.fallback($$props["items"], () => [], true);
	const color = (x) => x.color ?? "red";
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$.css_props($$renderer, true, { "--bg": color(item) }, () => {
			Sticker($$renderer, $.spread_props([item]));
		});
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { items });
}
