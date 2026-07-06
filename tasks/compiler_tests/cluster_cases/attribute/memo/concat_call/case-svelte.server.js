import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let status = $.fallback($$props["status"], "neutral");
	function classify(s) {
		return s + "-x";
	}
	$$renderer.push(`<div${$.attr_class(`slider ${$.stringify(classify(status) || "")}`)}></div>`);
	$.bind_props($$props, { status });
}
