import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let on = $.fallback($$props["on"], undefined);
	$$renderer.push(`<div></div>`);
	$.bind_props($$props, { on });
}
