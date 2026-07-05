import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let col = $$props["col"];
	$$renderer.push(`<div${$.attr_style("", { color: col })}></div>`);
	$.bind_props($$props, { col });
}
