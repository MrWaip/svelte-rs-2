import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	function fn(v) {
		return v + 1;
	}
	$$renderer.push(`<p${$.attr("data-x", fn(value))}></p>`);
	$.bind_props($$props, { value });
}
