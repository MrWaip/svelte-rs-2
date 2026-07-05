import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = $.fallback($$props["value"], 0);
	let label = $.fallback($$props["label"], "");
	function toPx(n) {
		return n + "px";
	}
	$$renderer.push(`<div${$.attr_style(`--w: ${$.stringify(toPx(value))};`)}${$.attr("data-testid", label)}></div>`);
	$.bind_props($$props, {
		value,
		label
	});
}
