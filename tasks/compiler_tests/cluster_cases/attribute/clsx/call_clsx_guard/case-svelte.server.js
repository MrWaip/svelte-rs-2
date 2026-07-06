import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = $$props["x"];
	function fn(v) {
		return v;
	}
	$$renderer.push(`<div${$.attr_class($.clsx(fn(x)))}></div>`);
	$.bind_props($$props, { x });
}
