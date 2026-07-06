import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let status = $.fallback($$props["status"], "neutral");
	function classify(s) {
		return s + "-x";
	}
	function widthOf(s) {
		return s.length;
	}
	$$renderer.push(`<div${$.attr_class(`slider ${$.stringify(classify(status) || "")}`)}${$.attr_style(`width: ${$.stringify(widthOf(status))}px`)}></div>`);
	$.bind_props($$props, { status });
}
