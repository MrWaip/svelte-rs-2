import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = $$props["count"];
	$$renderer.push(`<my-element${$.attr("value", count)}></my-element>`);
	$.bind_props($$props, { count });
}
