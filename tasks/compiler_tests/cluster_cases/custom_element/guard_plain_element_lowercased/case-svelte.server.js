import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let obj = $$props["obj"];
	$$renderer.push(`<div${$.attr("camelcase", obj)}></div>`);
	$.bind_props($$props, { obj });
}
