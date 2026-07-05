import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	$$renderer.push(`<div${$.attr_class(value)}></div>`);
	$.bind_props($$props, { value });
}
