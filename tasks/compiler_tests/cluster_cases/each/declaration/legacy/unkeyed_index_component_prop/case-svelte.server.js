import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer, $$props) {
	let widgets = $.fallback($$props["widgets"], () => [{ name: "foo" }], true);
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(widgets);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let widget = each_array[i];
		Widget($$renderer, {
			widget,
			index: i
		});
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { widgets });
}
