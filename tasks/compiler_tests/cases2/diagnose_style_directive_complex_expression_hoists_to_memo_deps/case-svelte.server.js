import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	let c = $$props["c"];
	function b(x) {
		return x;
	}
	$$renderer.push(`<div${$.attr_style("", { background: a || b(c) })}></div>`);
	$.bind_props($$props, {
		a,
		c
	});
}
