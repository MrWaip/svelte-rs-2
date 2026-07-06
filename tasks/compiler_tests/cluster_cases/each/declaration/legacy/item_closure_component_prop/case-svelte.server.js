import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer, $$props) {
	let clicked = $$props["clicked"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(["x"]);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let letter = each_array[$$index];
		Widget($$renderer, { handleClick: () => clicked = letter });
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { clicked });
}
